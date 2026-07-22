use crate::template::TemplateItem;
use crate::ui::theme::Theme;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, ListState};
use ratatui::Frame;
use std::collections::HashSet;

#[allow(clippy::too_many_arguments)]
pub fn render(
    frame: &mut Frame,
    area: Rect,
    items: &[&TemplateItem],
    selected_index: usize,
    toggled: &HashSet<String>,
    search_query: &str,
    is_search_mode: bool,
    is_focused: bool,
    use_nerd_fonts: bool,
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

    let search_indicator = if is_search_mode {
        format!(" / {}", search_query)
    } else if !search_query.is_empty() {
        format!(" [{}]", search_query)
    } else {
        String::new()
    };

    let title = ratatui::text::Line::from(vec![
        ratatui::text::Span::styled(" templates ", title_style),
        ratatui::text::Span::styled(
            format!("({})", items.len()),
            Style::default().fg(theme.secondary_accent),
        ),
        ratatui::text::Span::styled(search_indicator, Style::default().fg(theme.highlight)),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .title(title);

    if items.is_empty() {
        let empty_list = List::new(vec![
            ListItem::new("  no matching templates").style(Style::default().fg(theme.muted))
        ])
        .block(block);
        frame.render_widget(empty_list, area);
        return;
    }

    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            let is_toggled = toggled.contains(&item.name);
            let is_selected = idx == selected_index && is_focused;

            let cursor_span = if is_selected {
                ratatui::text::Span::styled("▎ ", Style::default().fg(theme.accent))
            } else {
                ratatui::text::Span::raw("  ")
            };

            let check_span = if is_toggled {
                let glyph = if use_nerd_fonts { "✓ " } else { "[x] " };
                ratatui::text::Span::styled(
                    glyph,
                    Style::default()
                        .fg(theme.success)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                let glyph = if use_nerd_fonts { "• " } else { "[ ] " };
                ratatui::text::Span::styled(glyph, Style::default().fg(theme.muted))
            };

            let name_style = if is_selected {
                Style::default().fg(theme.text).add_modifier(Modifier::BOLD)
            } else if is_toggled {
                Style::default().fg(theme.text)
            } else {
                Style::default().fg(theme.muted)
            };

            let name_span = ratatui::text::Span::styled(format!("{:<18}", item.name), name_style);

            let detected_span = if item.is_detected {
                ratatui::text::Span::styled(" • detected", Style::default().fg(theme.highlight))
            } else {
                ratatui::text::Span::raw("")
            };

            let mut item_style = Style::default();
            if is_selected {
                item_style = item_style.bg(theme.selection_bg);
            }

            let line =
                ratatui::text::Line::from(vec![cursor_span, check_span, name_span, detected_span]);

            ListItem::new(line).style(item_style)
        })
        .collect();

    let mut state = ListState::default();
    if !items.is_empty() {
        state.select(Some(selected_index));
    }

    let list = List::new(list_items).block(block);
    frame.render_stateful_widget(list, area, &mut state);
}
