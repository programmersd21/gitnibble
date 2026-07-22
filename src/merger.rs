use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffLine {
    Unchanged(String),
    Added(String),
    Header(String),
}

pub struct MergeResult {
    pub output: String,
    pub added_lines: usize,
    pub skipped_duplicates: usize,
}

pub struct GitignoreMerger;

impl GitignoreMerger {
    /// Merges new templates into an existing .gitignore string.
    /// Additive only: preserves all existing lines, comments, and structure.
    pub fn merge(existing: &str, new_templates: &[(&str, &str)], add_headers: bool) -> MergeResult {
        let mut existing_lines_set: HashSet<String> = HashSet::new();
        for line in existing.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                existing_lines_set.insert(trimmed.to_string());
            }
        }

        let mut output = String::from(existing);
        if !output.is_empty() && !output.ends_with('\n') {
            output.push('\n');
        }

        let mut added_lines = 0;
        let mut skipped_duplicates = 0;

        for &(name, content) in new_templates {
            let mut template_added_lines = Vec::new();
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed.starts_with('#') {
                    if !existing_lines_set.contains(trimmed) {
                        template_added_lines.push(line.to_string());
                    }
                    continue;
                }
                if existing_lines_set.contains(trimmed) {
                    skipped_duplicates += 1;
                } else {
                    existing_lines_set.insert(trimmed.to_string());
                    template_added_lines.push(line.to_string());
                    added_lines += 1;
                }
            }

            if !template_added_lines.is_empty() {
                if add_headers {
                    if !output.is_empty() && !output.ends_with("\n\n") {
                        if !output.ends_with('\n') {
                            output.push('\n');
                        }
                        output.push('\n');
                    }
                    output.push_str(&format!("### {} ###\n", name));
                }
                for line in template_added_lines {
                    output.push_str(&line);
                    output.push('\n');
                }
            }
        }

        MergeResult {
            output,
            added_lines,
            skipped_duplicates,
        }
    }

    /// Previews the diff line-by-line for rendering in TUI diff panel or CLI preview.
    pub fn preview(
        existing: &str,
        new_templates: &[(&str, &str)],
        add_headers: bool,
    ) -> Vec<DiffLine> {
        let mut lines = Vec::new();
        let mut existing_lines_set: HashSet<String> = HashSet::new();

        for line in existing.lines() {
            lines.push(DiffLine::Unchanged(line.to_string()));
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                existing_lines_set.insert(trimmed.to_string());
            }
        }

        for &(name, content) in new_templates {
            let mut template_added_lines = Vec::new();
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed.starts_with('#') {
                    if !existing_lines_set.contains(trimmed) {
                        template_added_lines.push(line.to_string());
                    }
                    continue;
                }
                if !existing_lines_set.contains(trimmed) {
                    existing_lines_set.insert(trimmed.to_string());
                    template_added_lines.push(line.to_string());
                }
            }

            if !template_added_lines.is_empty() {
                if add_headers {
                    lines.push(DiffLine::Header(format!("### {} ###", name)));
                }
                for line in template_added_lines {
                    lines.push(DiffLine::Added(line));
                }
            }
        }

        lines
    }
}
