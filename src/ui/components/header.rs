use crate::ui::theme::Theme;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub fn render(frame: &mut Frame, area: Rect, active_pane: &str, theme: &Theme) {
    let title = ratatui::text::Line::from(vec![
        ratatui::text::Span::styled(
            " gitnibble ",
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        ),
        ratatui::text::Span::styled(
            concat!("v", env!("CARGO_PKG_VERSION")),
            Style::default().fg(theme.muted),
        ),
        ratatui::text::Span::raw("  •  "),
        ratatui::text::Span::styled(
            format!("pane: {}", active_pane),
            Style::default().fg(theme.muted),
        ),
        ratatui::text::Span::raw("  •  "),
        ratatui::text::Span::styled(
            format!("theme: {}", theme.name),
            Style::default().fg(theme.highlight),
        ),
    ]);

    let p = Paragraph::new(title).style(Style::default().fg(theme.text));
    frame.render_widget(p, area);
}
