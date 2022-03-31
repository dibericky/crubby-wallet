use crate::overview::Overview;

#[derive(Debug)]
struct Section {
    title: String,
    content: String
}

#[derive(Debug)]
pub struct Page {
    body: Vec<Section>
}

impl Page {
    pub fn new (overview: Overview) -> Page {
        let overview_section = Section{
            title: "## Overview".to_owned(),
            content: overview.as_markdown()
        };
        let body = vec![
            overview_section
        ];
        Page {
            body
        }
    }

    pub fn as_markdown(&self) -> String {
        let mut sections : Vec<String> = vec![];
        for section in &self.body {
            sections.push(format!("{}\n\n{}", section.title, section.content))
        }
        sections.join("\n\n")
    }
}