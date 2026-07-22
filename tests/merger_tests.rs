use gitnibble::merger::{DiffLine, GitignoreMerger};

#[test]
fn merge_skips_existing_duplicate_lines() {
    let existing = "target/\n*.log\n";
    let templates = [("Rust", "target/\nCargo.lock\n")];
    let result = GitignoreMerger::merge(existing, &templates, true);
    assert_eq!(result.skipped_duplicates, 1);
    assert_eq!(result.added_lines, 1);
    assert!(result.output.contains("Cargo.lock"));
}

#[test]
fn merge_preserves_user_comments() {
    let existing = "# my custom notes\nfoo.txt\n";
    let templates = [("Node", "node_modules/\n")];
    let result = GitignoreMerger::merge(existing, &templates, false);
    assert!(result.output.contains("# my custom notes"));
    assert!(result.output.contains("foo.txt"));
}

#[test]
fn preview_marks_new_lines_as_added() {
    let existing = "target/\n";
    let templates = [("Rust", "target/\nCargo.lock\n")];
    let diff = GitignoreMerger::preview(existing, &templates, false);
    assert!(diff
        .iter()
        .any(|l| matches!(l, DiffLine::Added(s) if s == "Cargo.lock")));
}
