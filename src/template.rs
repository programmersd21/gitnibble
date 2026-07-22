use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config as MatcherConfig, Matcher, Utf32Str};

include!(concat!(env!("OUT_DIR"), "/embedded_templates.rs"));

#[derive(Debug, Clone)]
pub struct TemplateItem {
    pub name: String,
    pub content: String,
    pub is_detected: bool,
}

pub struct TemplateStore {
    templates: Vec<TemplateItem>,
}

impl TemplateStore {
    pub fn new(detected_keys: &[&str]) -> Self {
        let templates = EMBEDDED_TEMPLATES
            .iter()
            .map(|&(name, content)| {
                let is_detected = detected_keys.contains(&name);
                TemplateItem {
                    name: name.to_string(),
                    content: content.to_string(),
                    is_detected,
                }
            })
            .collect();

        Self { templates }
    }

    pub fn all(&self) -> &[TemplateItem] {
        &self.templates
    }

    pub fn find(&self, query: &str) -> Option<&TemplateItem> {
        self.templates
            .iter()
            .find(|t| t.name.eq_ignore_ascii_case(query))
    }

    pub fn search(&self, query: &str) -> Vec<&TemplateItem> {
        if query.trim().is_empty() {
            return self.templates.iter().collect();
        }

        let mut matcher = Matcher::new(MatcherConfig::DEFAULT);
        let pattern = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart);

        let mut matches: Vec<(&TemplateItem, u32)> = Vec::new();

        for item in &self.templates {
            let mut utf32_buf = Vec::new();
            let snapshot = Utf32Str::new(&item.name, &mut utf32_buf);
            if let Some(score) = pattern.score(snapshot, &mut matcher) {
                matches.push((item, score));
            }
        }

        matches.sort_by_key(|b| std::cmp::Reverse(b.1));
        matches.into_iter().map(|(item, _)| item).collect()
    }
}
