use crate::ui::theme::Theme;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::Frame;

pub fn render(frame: &mut Frame, area: Rect, theme: &Theme) {
    let popup_area = crate::ui::centered_rect(58, 62, area);

    frame.render_widget(Clear, popup_area);

    let key_items = vec![
        ("j / down", "move selection down"),
        ("k / up", "move selection up"),
        ("pgdn / pgup", "scroll diff by 10 lines"),
        ("g / G", "jump to top / bottom of diff"),
        ("space", "toggle template selection"),
        ("a", "select all detected templates"),
        ("c", "clear selection"),
        ("/", "fuzzy search templates"),
        ("tab", "switch active pane"),
        ("r", "rescan workspace"),
        ("t", "theme selector"),
        ("enter", "apply templates to .gitignore"),
        ("y", "copy diff preview to clipboard"),
        ("?", "toggle help modal"),
        ("q / esc", "quit application"),
    ];

    let mut text = vec![ratatui::text::Line::from("")];

    for (key, desc) in key_items {
        text.push(ratatui::text::Line::from(vec![
            ratatui::text::Span::raw("  "),
            ratatui::text::Span::styled(
                format!("{:<14}", key),
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            ratatui::text::Span::styled(desc, Style::default().fg(theme.text)),
        ]));
    }

    text.push(ratatui::text::Line::from(""));
    text.push(ratatui::text::Line::from(ratatui::text::Span::styled(
        "  press esc or ? to close",
        Style::default().fg(theme.muted),
    )));

    let block = Block::default()
        .title(
            ratatui::text::Line::from(" keybindings ").style(
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_focused));

    let paragraph = Paragraph::new(text).block(block).alignment(Alignment::Left);
    frame.render_widget(paragraph, popup_area);
}
