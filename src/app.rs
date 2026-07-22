use crate::config::Config;
use crate::merger::{DiffLine, GitignoreMerger};
use crate::scanner::{DetectedRule, WorkspaceScanner};
use crate::template::{TemplateItem, TemplateStore};
use crate::ui::components::{
    diff_preview, help_modal, stack_panel, status_bar, template_list, theme_preview,
};
use crate::ui::theme::Theme;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Frame;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusedPane {
    Stack,
    Templates,
    Diff,
}

pub struct App {
    pub config: Config,
    pub theme: Theme,
    pub detected_rules: Vec<DetectedRule>,
    pub template_store: TemplateStore,
    pub toggled_templates: HashSet<String>,
    pub selected_index: usize,
    pub search_query: String,
    pub is_search_mode: bool,
    pub show_help: bool,
    pub focused_pane: FocusedPane,
    pub diff_scroll: usize,
    pub status_message: Option<(String, std::time::Instant)>,
    pub should_quit: bool,

    pub show_theme_selector: bool,
    pub theme_selector_index: usize,

    cached_diff: Vec<DiffLine>,
    diff_dirty: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        let theme = Theme::from_name(&config.default_theme);
        let detected_rules = WorkspaceScanner::scan(".");
        let detected_keys: Vec<&str> = detected_rules.iter().map(|r| r.template_key()).collect();
        let template_store = TemplateStore::new(&detected_keys);

        let mut toggled_templates = HashSet::new();
        for key in &detected_keys {
            if template_store.find(key).is_some() {
                toggled_templates.insert(key.to_string());
            }
        }

        let cached_diff = Vec::new();

        Self {
            config,
            theme,
            detected_rules,
            template_store,
            toggled_templates,
            selected_index: 0,
            search_query: String::new(),
            is_search_mode: false,
            show_help: false,
            focused_pane: FocusedPane::Templates,
            diff_scroll: 0,
            status_message: None,
            should_quit: false,
            show_theme_selector: false,
            theme_selector_index: 0,
            cached_diff,
            diff_dirty: true,
        }
    }

    pub fn set_status<S: Into<String>>(&mut self, msg: S) {
        self.status_message = Some((msg.into(), std::time::Instant::now()));
    }

    pub fn cycle_pane(&mut self) {
        self.focused_pane = match self.focused_pane {
            FocusedPane::Stack => FocusedPane::Templates,
            FocusedPane::Templates => FocusedPane::Diff,
            FocusedPane::Diff => FocusedPane::Stack,
        };
    }

    pub fn toggle_template(&mut self, name: &str) {
        if self.toggled_templates.contains(name) {
            self.toggled_templates.remove(name);
        } else {
            self.toggled_templates.insert(name.to_string());
        }
        self.diff_dirty = true;
    }

    pub fn set_theme(&mut self, name: &str) {
        self.theme = Theme::from_name(name);
        self.config.default_theme = name.to_string();
        self.save_config();
        self.set_status(format!("theme: {}", self.theme.name));
    }

    pub fn open_theme_selector(&mut self) {
        let themes = Theme::all();
        self.theme_selector_index = themes
            .iter()
            .position(|t| t.name == self.theme.name)
            .unwrap_or(0);
        self.show_theme_selector = true;
    }

    fn save_config(&self) {
        if let Err(e) = Config::save(&self.config) {
            eprintln!("warning: failed to save config: {}", e);
        }
    }

    pub fn filtered_templates(&self) -> Vec<&TemplateItem> {
        self.template_store.search(&self.search_query)
    }

    pub fn rescan(&mut self) {
        self.detected_rules = WorkspaceScanner::scan(".");
        let detected_keys: Vec<&str> = self
            .detected_rules
            .iter()
            .map(|r| r.template_key())
            .collect();
        self.template_store = TemplateStore::new(&detected_keys);
        self.toggled_templates.clear();
        for key in &detected_keys {
            if self.template_store.find(key).is_some() {
                self.toggled_templates.insert(key.to_string());
            }
        }
        self.selected_index = 0;
        self.diff_dirty = true;
        self.set_status("workspace rescanned successfully");
    }

    pub fn select_all_detected(&mut self) {
        for rule in &self.detected_rules {
            if self.template_store.find(rule.template_key()).is_some() {
                self.toggled_templates
                    .insert(rule.template_key().to_string());
            }
        }
        self.diff_dirty = true;
        self.set_status("selected all detected templates");
    }

    pub fn clear_selection(&mut self) {
        self.toggled_templates.clear();
        self.diff_dirty = true;
        self.set_status("cleared template selection");
    }

    fn selected_template_pairs(&self) -> Vec<(&str, &str)> {
        self.toggled_templates
            .iter()
            .filter_map(|name| {
                self.template_store
                    .find(name)
                    .map(|t| (t.name.as_str(), t.content.as_str()))
            })
            .collect()
    }

    pub fn current_diff(&self) -> Vec<DiffLine> {
        let existing = fs::read_to_string(".gitignore").unwrap_or_default();
        let selected = self.selected_template_pairs();
        GitignoreMerger::preview(&existing, &selected, self.config.add_headers)
    }

    pub fn apply_changes(&mut self) -> Result<(), std::io::Error> {
        let existing = fs::read_to_string(".gitignore").unwrap_or_default();
        let selected = self.selected_template_pairs();
        if selected.is_empty() {
            self.set_status("no templates selected to apply");
            return Ok(());
        }
        let result = GitignoreMerger::merge(&existing, &selected, self.config.add_headers);
        fs::write(".gitignore", result.output)?;
        self.diff_dirty = true;
        self.set_status(format!(
            "added {} lines, skipped {} duplicates",
            result.added_lines, result.skipped_duplicates
        ));
        Ok(())
    }

    pub fn copy_diff_to_clipboard(&mut self) {
        #[cfg(feature = "clipboard")]
        {
            use arboard::Clipboard;
            let diff = self.current_diff();
            let mut diff_str = String::new();
            for line in diff {
                match line {
                    DiffLine::Unchanged(s) => diff_str.push_str(&format!("  {}\n", s)),
                    DiffLine::Added(s) => diff_str.push_str(&format!("+ {}\n", s)),
                    DiffLine::Header(s) => diff_str.push_str(&format!("{}\n", s)),
                }
            }

            if let Ok(mut cb) = Clipboard::new() {
                if cb.set_text(diff_str).is_ok() {
                    self.set_status("diff preview copied to clipboard");
                    return;
                }
            }
            self.set_status("failed to copy to clipboard");
        }

        #[cfg(not(feature = "clipboard"))]
        {
            self.set_status("clipboard feature disabled in build");
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let full_size = frame.area();

        // Subtle outer margin for floating stage effect on larger displays
        let size = if full_size.width > 100 && full_size.height > 24 {
            let outer_vertical = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(full_size);

            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(2),
                    Constraint::Min(0),
                    Constraint::Length(2),
                ])
                .split(outer_vertical[1])[1]
        } else {
            full_size
        };

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(size);

        let active_pane_str = match self.focused_pane {
            FocusedPane::Stack => "stack",
            FocusedPane::Templates => "templates",
            FocusedPane::Diff => "diff",
        };

        // Floating workspace panel gaps (1-char spacing between columns)
        let pane_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(24),
                Constraint::Length(1), // Gap
                Constraint::Percentage(35),
                Constraint::Length(1), // Gap
                Constraint::Min(0),
            ])
            .split(main_chunks[0]);

        stack_panel::render(
            frame,
            pane_chunks[0],
            &self.detected_rules,
            self.focused_pane == FocusedPane::Stack,
            &self.theme,
        );

        let filtered_len = self.filtered_templates().len();
        // Ensure selected index stays in bounds (no mutation in render)
        if filtered_len == 0 {
            self.selected_index = 0;
        } else if self.selected_index >= filtered_len {
            self.selected_index = filtered_len - 1;
        }

        let filtered = self.filtered_templates();

        template_list::render(
            frame,
            pane_chunks[2],
            &filtered,
            self.selected_index,
            &self.toggled_templates,
            &self.search_query,
            self.is_search_mode,
            self.focused_pane == FocusedPane::Templates,
            self.config.use_nerd_fonts,
            &self.theme,
        );

        if self.diff_dirty {
            self.cached_diff = self.current_diff();
            self.diff_dirty = false;
        }

        let diff_height = pane_chunks[4].height.saturating_sub(2) as usize; // subtract borders
        let max_scroll = self.cached_diff.len().saturating_sub(diff_height);
        if self.diff_scroll > max_scroll {
            self.diff_scroll = max_scroll;
        }

        diff_preview::render(
            frame,
            pane_chunks[4],
            &self.cached_diff,
            self.diff_scroll,
            self.focused_pane == FocusedPane::Diff,
            &self.theme,
        );

        let current_status_str = if let Some((msg, inst)) = &self.status_message {
            if inst.elapsed().as_secs() < 3 {
                Some(msg.as_str())
            } else {
                None
            }
        } else {
            None
        };

        status_bar::render(
            frame,
            main_chunks[1],
            current_status_str,
            self.toggled_templates.len(),
            self.template_store.all().len(),
            active_pane_str,
            &self.theme,
        );

        if self.show_help {
            help_modal::render(frame, full_size, &self.theme);
        }

        if self.show_theme_selector {
            let themes = Theme::all();
            theme_preview::render(
                frame,
                full_size,
                &themes,
                self.theme_selector_index,
                &self.theme,
            );
        }
    }
}
