use clap::Parser;
use crossterm::{
    event::KeyCode,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use gitnibble::{
    app::{App, FocusedPane},
    cli::{Cli, Command},
    config::Config,
    event::{AppEvent, EventHandler},
    merger::GitignoreMerger,
    scanner::WorkspaceScanner,
    template::TemplateStore,
    ui::theme::Theme,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::fs;
use std::io;
use std::process::ExitCode;

fn restore_terminal() {
    let _ = execute!(io::stdout(), LeaveAlternateScreen);
    let _ = disable_raw_mode();
}

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        restore_terminal();
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let config = Config::load().unwrap_or_default();

    if cli.fetch {
        #[cfg(not(feature = "fetch"))]
        {
            eprintln!(
                "warning: {}",
                gitnibble::template_fetch::fetch_disabled_notice()
            );
        }
    }

    if let Some(cmd) = cli.command {
        match cmd {
            Command::Detect => {
                let detected = WorkspaceScanner::scan(".");
                if detected.is_empty() {
                    println!("no stack detected.");
                } else {
                    println!("detected stack:");
                    for rule in detected {
                        println!("  {:<16} [{}]", rule.template_key(), rule.category_label());
                    }
                }
                return ExitCode::SUCCESS;
            }
            Command::List => {
                let store = TemplateStore::new(&[]);
                println!("available templates ({}):", store.all().len());
                for t in store.all() {
                    println!("  {}", t.name);
                }
                return ExitCode::SUCCESS;
            }
            Command::Add { templates } => {
                let existing = fs::read_to_string(".gitignore").unwrap_or_default();
                let store = TemplateStore::new(&[]);
                let matched_templates = match resolve_templates(&store, &templates) {
                    Ok(m) => m,
                    Err(_) => return ExitCode::from(2),
                };

                if cli.dry_run || config.always_dry_run_first {
                    let diff =
                        GitignoreMerger::preview(&existing, &matched_templates, config.add_headers);
                    println!("dry run diff preview:");
                    for line in diff {
                        match line {
                            gitnibble::merger::DiffLine::Unchanged(_) => {}
                            gitnibble::merger::DiffLine::Added(s) => println!("+ {}", s),
                            gitnibble::merger::DiffLine::Header(s) => println!("{}", s),
                        }
                    }
                    println!("\ndry-run mode: no changes written.");
                    return ExitCode::SUCCESS;
                }

                if !cli.yes {
                    print!("Write changes to .gitignore? [y/N]: ");
                    use std::io::Write;
                    let _ = std::io::stdout().flush();
                    let mut input = String::new();
                    if std::io::stdin().read_line(&mut input).is_err()
                        || !input.trim().eq_ignore_ascii_case("y")
                    {
                        println!("Aborted.");
                        return ExitCode::SUCCESS;
                    }
                }

                let result =
                    GitignoreMerger::merge(&existing, &matched_templates, config.add_headers);
                if let Err(e) = fs::write(".gitignore", result.output) {
                    eprintln!("error writing .gitignore: {}", e);
                    return ExitCode::FAILURE;
                }
                println!(
                    "successfully updated .gitignore (+{} lines, {} skipped duplicates).",
                    result.added_lines, result.skipped_duplicates
                );
                return ExitCode::SUCCESS;
            }
            Command::Diff { templates } => {
                let existing = fs::read_to_string(".gitignore").unwrap_or_default();
                let store = TemplateStore::new(&[]);
                let matched_templates = match resolve_templates(&store, &templates) {
                    Ok(m) => m,
                    Err(_) => return ExitCode::from(2),
                };

                let result =
                    GitignoreMerger::merge(&existing, &matched_templates, config.add_headers);
                let diff =
                    GitignoreMerger::preview(&existing, &matched_templates, config.add_headers);

                for line in diff {
                    match line {
                        gitnibble::merger::DiffLine::Unchanged(_) => {}
                        gitnibble::merger::DiffLine::Added(s) => println!("+ {}", s),
                        gitnibble::merger::DiffLine::Header(s) => println!("{}", s),
                    }
                }

                if result.added_lines > 0 {
                    return ExitCode::from(1);
                } else {
                    return ExitCode::SUCCESS;
                }
            }
        }
    }

    if let Err(e) = run_tui(config) {
        eprintln!("tui error: {}", e);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn resolve_templates<'a>(
    store: &'a TemplateStore,
    names: &[String],
) -> Result<Vec<(&'a str, &'a str)>, ()> {
    let mut matched = Vec::new();
    for name in names {
        if let Some(item) = store.find(name) {
            matched.push((item.name.as_str(), item.content.as_str()));
        } else {
            eprintln!("error: template '{}' not found.", name);
            return Err(());
        }
    }
    Ok(matched)
}

fn run_tui(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore_terminal();
        prev_hook(info);
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let _guard = TerminalGuard;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new(config);

    loop {
        terminal.draw(|f| app.render(f))?;

        if app.should_quit {
            break;
        }

        match EventHandler::next()? {
            AppEvent::Key(code, modifiers) => {
                // Ctrl+C should quit immediately
                if modifiers.contains(crossterm::event::KeyModifiers::CONTROL)
                    && code == KeyCode::Char('c')
                {
                    app.should_quit = true;
                    continue;
                }

                if app.is_search_mode {
                    match code {
                        KeyCode::Esc | KeyCode::Enter => {
                            app.is_search_mode = false;
                        }
                        KeyCode::Tab => {
                            app.is_search_mode = false;
                            app.cycle_pane();
                        }
                        KeyCode::Backspace => {
                            app.search_query.pop();
                            app.selected_index = 0;
                        }
                        KeyCode::Char(c) => {
                            app.search_query.push(c);
                            app.selected_index = 0;
                        }
                        _ => {}
                    }
                    continue;
                }

                if app.show_help {
                    match code {
                        KeyCode::Char('?') | KeyCode::Esc | KeyCode::Char('q') => {
                            app.show_help = false;
                        }
                        KeyCode::Tab => {
                            app.show_help = false;
                            app.cycle_pane();
                        }
                        _ => {}
                    }
                    continue;
                }

                if app.show_theme_selector {
                    match code {
                        KeyCode::Esc => {
                            app.show_theme_selector = false;
                        }
                        KeyCode::Enter => {
                            let themes = Theme::all();
                            if let Some(t) = themes.get(app.theme_selector_index) {
                                app.set_theme(t.name);
                            }
                            app.show_theme_selector = false;
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            let themes = Theme::all();
                            let count = themes.len();
                            if app.theme_selector_index + 1 < count {
                                app.theme_selector_index += 1;
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up if app.theme_selector_index > 0 => {
                            app.theme_selector_index -= 1;
                        }
                        _ => {}
                    }
                    continue;
                }

                match code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.should_quit = true;
                    }
                    KeyCode::Char('?') => {
                        app.show_help = true;
                    }
                    KeyCode::Tab => {
                        app.cycle_pane();
                    }
                    KeyCode::Char('/') => {
                        app.is_search_mode = true;
                        app.focused_pane = FocusedPane::Templates;
                    }
                    KeyCode::Char('r') => {
                        app.rescan();
                    }
                    KeyCode::Char('t') => {
                        app.open_theme_selector();
                    }
                    KeyCode::Char('a') => {
                        app.select_all_detected();
                    }
                    KeyCode::Char('c') => {
                        app.clear_selection();
                    }
                    KeyCode::Char('y') => {
                        app.copy_diff_to_clipboard();
                    }
                    KeyCode::Char('j') | KeyCode::Down => match app.focused_pane {
                        FocusedPane::Templates => {
                            let len = app.filtered_templates().len();
                            if len > 0 && app.selected_index < len - 1 {
                                app.selected_index += 1;
                            }
                        }
                        FocusedPane::Diff => {
                            app.diff_scroll = app.diff_scroll.saturating_add(1);
                        }
                        _ => {}
                    },
                    KeyCode::Char('k') | KeyCode::Up => match app.focused_pane {
                        FocusedPane::Templates if app.selected_index > 0 => {
                            app.selected_index -= 1;
                        }
                        FocusedPane::Diff => {
                            app.diff_scroll = app.diff_scroll.saturating_sub(1);
                        }
                        _ => {}
                    },
                    KeyCode::PageDown if app.focused_pane == FocusedPane::Diff => {
                        app.diff_scroll = app.diff_scroll.saturating_add(10);
                    }
                    KeyCode::PageUp if app.focused_pane == FocusedPane::Diff => {
                        app.diff_scroll = app.diff_scroll.saturating_sub(10);
                    }
                    KeyCode::Char('g') if app.focused_pane == FocusedPane::Diff => {
                        app.diff_scroll = 0;
                    }
                    KeyCode::Char('G') if app.focused_pane == FocusedPane::Diff => {
                        app.diff_scroll = usize::MAX; // clamped in render
                    }
                    KeyCode::Char(' ') if app.focused_pane == FocusedPane::Templates => {
                        let name = app
                            .filtered_templates()
                            .get(app.selected_index)
                            .map(|t| t.name.clone());
                        if let Some(ref n) = name {
                            app.toggle_template(n);
                        }
                    }
                    KeyCode::Enter => {
                        if let Err(e) = app.apply_changes() {
                            app.set_status(format!("error writing file: {}", e));
                        }
                    }
                    _ => {}
                }
            }
            AppEvent::Resize | AppEvent::Tick => {}
        }
    }

    Ok(())
}
