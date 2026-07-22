use crate::ui::theme::Theme;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::Frame;

pub fn render(
    frame: &mut Frame,
    area: Rect,
    themes: &[Theme],
    selected: usize,
    current_theme: &Theme,
) {
    let popup_area = crate::ui::centered_rect(62, 64, area);
    frame.render_widget(Clear, popup_area);

    let mut text = vec![ratatui::text::Line::from("")];

    for (i, t) in themes.iter().enumerate() {
        let is_selected = i == selected;
        let is_active = t.name == current_theme.name;

        let cursor_span = if is_selected {
            ratatui::text::Span::styled("  ▎ ", Style::default().fg(current_theme.accent))
        } else {
            ratatui::text::Span::raw("    ")
        };

        let swatches = vec![
            ratatui::text::Span::styled("● ", Style::default().fg(t.accent)),
            ratatui::text::Span::styled("● ", Style::default().fg(t.secondary_accent)),
            ratatui::text::Span::styled("● ", Style::default().fg(t.success)),
            ratatui::text::Span::styled("● ", Style::default().fg(t.warning)),
            ratatui::text::Span::styled("● ", Style::default().fg(t.highlight)),
        ];

        let name_style = if is_selected {
            Style::default()
                .fg(current_theme.text)
                .add_modifier(Modifier::BOLD)
        } else if is_active {
            Style::default().fg(current_theme.text)
        } else {
            Style::default().fg(current_theme.muted)
        };

        let name_span = ratatui::text::Span::styled(format!("  {:<18}", t.name), name_style);

        let active_indicator = if is_active {
            ratatui::text::Span::styled(
                "  • active",
                Style::default().fg(current_theme.secondary_accent),
            )
        } else {
            ratatui::text::Span::raw("")
        };

        let mut line_spans = vec![cursor_span];
        line_spans.extend(swatches);
        line_spans.push(name_span);
        line_spans.push(active_indicator);

        let line = ratatui::text::Line::from(line_spans);

        if is_selected {
            text.push(line.style(Style::default().bg(current_theme.selection_bg)));
        } else {
            text.push(line);
        }

        // Generous spacing between items for airiness
        text.push(ratatui::text::Line::from(""));
    }

    text.push(ratatui::text::Line::from(ratatui::text::Span::styled(
        "  j/k navigate  •  enter select  •  esc close",
        Style::default().fg(current_theme.muted),
    )));

    let block = Block::default()
        .title(
            ratatui::text::Line::from(" select theme ").style(
                Style::default()
                    .fg(current_theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(current_theme.border_focused));

    let paragraph = Paragraph::new(text).block(block).alignment(Alignment::Left);
    frame.render_widget(paragraph, popup_area);
}
