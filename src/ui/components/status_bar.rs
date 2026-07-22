use crate::ui::theme::Theme;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use std::time::SystemTime;

pub fn render(
    frame: &mut Frame,
    area: Rect,
    status_msg: Option<&str>,
    selected_count: usize,
    total_templates: usize,
    active_pane: &str,
    theme: &Theme,
) {
    let right_len = 26 + theme.name.len() + active_pane.len();
    let right_width = (right_len as u16).min(area.width.saturating_sub(20));

    let chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Min(0),
            ratatui::layout::Constraint::Length(right_width),
        ])
        .split(area);

    let pulse_stage = (SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_millis() / 400)
        .unwrap_or(0)
        % 3) as usize;

    let pulse_dot = match pulse_stage {
        0 => ratatui::text::Span::styled("● ", Style::default().fg(theme.accent)),
        1 => ratatui::text::Span::styled("• ", Style::default().fg(theme.secondary_accent)),
        _ => ratatui::text::Span::styled("· ", Style::default().fg(theme.muted)),
    };

    let left_spans = if let Some(m) = status_msg {
        vec![
            pulse_dot,
            ratatui::text::Span::styled(
                m,
                Style::default()
                    .fg(theme.highlight)
                    .add_modifier(Modifier::BOLD),
            ),
        ]
    } else {
        vec![
            pulse_dot,
            ratatui::text::Span::styled(
                format!("{}/{} sel", selected_count, total_templates),
                Style::default().fg(theme.text),
            ),
            ratatui::text::Span::raw("  "),
            ratatui::text::Span::styled("space", Style::default().fg(theme.accent)),
            ratatui::text::Span::styled(" toggle ", Style::default().fg(theme.muted)),
            ratatui::text::Span::styled("enter", Style::default().fg(theme.accent)),
            ratatui::text::Span::styled(" apply ", Style::default().fg(theme.muted)),
            ratatui::text::Span::styled("/", Style::default().fg(theme.accent)),
            ratatui::text::Span::styled(" search ", Style::default().fg(theme.muted)),
            ratatui::text::Span::styled("tab", Style::default().fg(theme.accent)),
            ratatui::text::Span::styled(" pane", Style::default().fg(theme.muted)),
        ]
    };

    let p_left = Paragraph::new(ratatui::text::Line::from(left_spans));
    frame.render_widget(p_left, chunks[0]);

    let title = ratatui::text::Line::from(vec![
        ratatui::text::Span::styled(
            "gitnibble ",
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        ),
        ratatui::text::Span::styled(
            concat!("v", env!("CARGO_PKG_VERSION")),
            Style::default().fg(theme.muted),
        ),
        ratatui::text::Span::raw(" • "),
        ratatui::text::Span::styled(active_pane, Style::default().fg(theme.secondary_accent)),
        ratatui::text::Span::raw(" • "),
        ratatui::text::Span::styled(theme.name, Style::default().fg(theme.highlight)),
    ]);

    let p_right = Paragraph::new(title)
        .alignment(ratatui::layout::Alignment::Right)
        .style(Style::default().fg(theme.text));
    frame.render_widget(p_right, chunks[1]);
}
