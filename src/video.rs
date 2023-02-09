use serde::{Deserialize, Serialize};

// This should use a builder pattern
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Video {
    pub path: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub madeforkids: Option<bool>,
    pub tags: Vec<String>,
    pub automatic_chapters: Option<bool>,
}

#[allow(dead_code)]
impl Video {
    pub fn new(path: &str) -> Self {
        Video {
            path: path.to_owned(),
            title: None,
            description: None,
            madeforkids: None,
            tags: vec![],
            automatic_chapters: None,
        }
    }

    pub fn add_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
    }

    pub fn add_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }

    pub fn add_madeforkids(mut self, madeforkids: bool) -> Self {
        self.madeforkids = Some(madeforkids);
        self
    }

    pub fn add_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_owned());
        self
    }

    pub fn add_tags(mut self, tags: Vec<&str>) -> Self {
        tags.iter()
            .for_each(|tag| self.tags.push(tag.to_owned().to_owned()));
        self
    }

    pub fn get_tags_for_text_input(&self) -> String {
        self.tags.join(",") + "\n"
    }

    pub fn add_automatic_chapters(mut self, automatic_chapters: bool) -> Self {
        self.automatic_chapters = Some(automatic_chapters);
        self
    }
}
