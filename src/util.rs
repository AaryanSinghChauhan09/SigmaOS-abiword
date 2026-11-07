//! abiword model — word processor.

#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub documents: Vec<Document>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { documents: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create(&mut self, title: &str) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.documents.push(Document { id: id.clone(), title: title.into(), content: String::new() });
        id
    }

    pub fn save(&mut self, id: &str, content: &str) -> bool {
        if let Some(d) = self.documents.iter_mut().find(|d| d.id == id) {
            d.content = content.into();
            true
        } else { false }
    }

    pub fn list(&self) -> &[Document] { &self.documents }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn document_create_save() {
        let mut a = Model::new();
        let id = a.create("Letter");
        assert!(a.save(&id, "content"));
    }
}
