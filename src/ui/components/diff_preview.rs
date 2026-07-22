use crate::merger::DiffLine;
use crate::ui::theme::Theme;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

pub fn render(
    frame: &mut Frame,
    area: Rect,
    diff_lines: &[DiffLine],
    scroll_offset: usize,
    is_focused: bool,
    theme: &Theme,
) {
    let (border_color, title_style) = if is_focused {
        (
            theme.border_focused,
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        (theme.border, Style::default().fg(theme.muted))
    };

    let title = ratatui::text::Line::from(vec![
        ratatui::text::Span::styled(" diff preview ", title_style),
        ratatui::text::Span::styled(
            format!("({} lines)", diff_lines.len()),
            Style::default().fg(theme.secondary_accent),
        ),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .title(title);

    if diff_lines.is_empty() {
        let p = Paragraph::new("  no pending changes")
            .style(Style::default().fg(theme.muted))
            .block(block);
        frame.render_widget(p, area);
        return;
    }

    let mut lines_spans = Vec::new();

    for line in diff_lines {
        match line {
            DiffLine::Unchanged(s) => {
                lines_spans.push(ratatui::text::Line::from(ratatui::text::Span::styled(
                    format!("  {}", s),
                    Style::default().fg(theme.muted),
                )));
            }
            DiffLine::Added(s) => {
                lines_spans.push(ratatui::text::Line::from(vec![
                    ratatui::text::Span::styled(
                        "+ ",
                        Style::default()
                            .fg(theme.success)
                            .add_modifier(Modifier::BOLD),
                    ),
                    ratatui::text::Span::styled(s.to_string(), Style::default().fg(theme.text)),
                ]));
            }
            DiffLine::Header(s) => {
                lines_spans.push(ratatui::text::Line::from(ratatui::text::Span::styled(
                    format!(" {}", s),
                    Style::default()
                        .fg(theme.warning)
                        .add_modifier(Modifier::BOLD),
                )));
            }
        }
    }

    let paragraph = Paragraph::new(lines_spans)
        .block(block)
        .scroll((scroll_offset.min(u16::MAX as usize) as u16, 0));

    frame.render_widget(paragraph, area);
}
