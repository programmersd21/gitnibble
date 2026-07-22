use crate::scanner::DetectedRule;
use crate::ui::theme::Theme;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem};
use ratatui::Frame;

pub fn render(
    frame: &mut Frame,
    area: Rect,
    rules: &[DetectedRule],
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
        ratatui::text::Span::styled(" detected stack ", title_style),
        ratatui::text::Span::styled(
            format!("({})", rules.len()),
            Style::default().fg(theme.secondary_accent),
        ),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .title(title);

    if rules.is_empty() {
        let empty_list = List::new(vec![
            ListItem::new("  no stack detected").style(Style::default().fg(theme.muted))
        ])
        .block(block);
        frame.render_widget(empty_list, area);
        return;
    }

    let items: Vec<ListItem> = rules
        .iter()
        .map(|r| {
            let line = ratatui::text::Line::from(vec![
                ratatui::text::Span::raw("  "),
                ratatui::text::Span::styled(
                    format!("{:<13}", r.template_key()),
                    Style::default().fg(theme.text).add_modifier(Modifier::BOLD),
                ),
                ratatui::text::Span::styled(
                    format!(" {}", r.category_label()),
                    Style::default().fg(theme.muted),
                ),
            ]);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}
