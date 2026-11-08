//! Document model for abiword.

use std::collections::HashMap;
use crate::style::{FontStyle, ParagraphStyle, ParagraphAlignment, Style, StyleType};

/// Document format
#[derive(Debug, Clone, PartialEq)]
pub enum DocumentFormat {
    /// Microsoft Word format
    Docx,
    /// OpenDocument format
    Odt,
    /// Plain text format
    Txt,
    /// PDF format
    Pdf,
    /// HTML format
    Html,
}

/// Text run with formatting
#[derive(Debug, Clone)]
pub struct TextRun {
    /// Text content
    pub text: String,
    /// Font style
    pub style: FontStyle,
}







/// Paragraph
#[derive(Debug, Clone)]
pub struct Paragraph {
    /// Paragraph ID
    pub id: String,
    /// Text content
    pub text: String,
    /// Text runs with formatting
    pub runs: Vec<TextRun>,
    /// Paragraph style
    pub style: ParagraphStyle,
}

impl Paragraph {
    /// Create new paragraph
    pub fn new(id: String, text: String) -> Self {
        Self {
            id,
            text,
            runs: vec![],
            style: ParagraphStyle::default(),
        }
    }
    
    /// Add text run
    pub fn add_run(&mut self, text: String, style: FontStyle) {
        self.runs.push(TextRun { text, style });
    }
    
    /// Get plain text
    pub fn get_plain_text(&self) -> String {
        if self.runs.is_empty() {
            self.text.clone()
        } else {
            self.runs.iter().map(|r| r.text.as_str()).collect()
        }
    }
    
    /// Get word count
    pub fn get_word_count(&self) -> usize {
        self.get_plain_text().split_whitespace().count()
    }
    
    /// Get character count
    pub fn get_character_count(&self) -> usize {
        self.get_plain_text().chars().count()
    }
}

/// Document
#[derive(Debug, Clone)]
pub struct Document {
    /// Document ID
    pub id: String,
    /// Document title
    pub title: String,
    /// Document format
    pub format: DocumentFormat,
    /// Paragraphs
    pub paragraphs: Vec<Paragraph>,
    /// Styles
    pub styles: HashMap<String, Style>,
    /// Created timestamp
    pub created_at: u64,
    /// Modified timestamp
    pub modified_at: u64,
    /// Author
    pub author: String,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl Document {
    /// Create new document
    pub fn new(title: String) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut doc = Self {
            id: format!("doc-{}", now),
            title,
            format: DocumentFormat::Docx,
            paragraphs: vec![],
            styles: HashMap::new(),
            created_at: now,
            modified_at: now,
            author: String::new(),
            metadata: HashMap::new(),
        };
        
        // Add default styles
        doc.add_default_styles();
        
        doc
    }
    
    /// Add default styles
    fn add_default_styles(&mut self) {
        // Heading 1
        let mut heading1 = Style::new("Heading1".into(), StyleType::Paragraph);
        heading1.font_style.bold = true;
        heading1.font_style.font_size = 24.0;
        self.styles.insert("Heading1".into(), heading1);
        
        // Heading 2
        let mut heading2 = Style::new("Heading2".into(), StyleType::Paragraph);
        heading2.font_style.bold = true;
        heading2.font_style.font_size = 18.0;
        self.styles.insert("Heading2".into(), heading2);
        
        // Normal
        let normal = Style::new("Normal".into(), StyleType::Paragraph);
        self.styles.insert("Normal".into(), normal);
    }
    
    /// Add paragraph
    pub fn add_paragraph(&mut self, text: String) -> String {
        let id = format!("para-{}", self.paragraphs.len());
        let paragraph = Paragraph::new(id.clone(), text);
        self.paragraphs.push(paragraph);
        self.modified_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        id
    }
    
    /// Add text run to paragraph
    pub fn add_text_run(&mut self, paragraph_id: &str, text: String, style: FontStyle) -> bool {
        if let Some(paragraph) = self.paragraphs.iter_mut().find(|p| p.id == paragraph_id) {
            paragraph.add_run(text, style);
            self.modified_at = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            true
        } else {
            false
        }
    }
    
    /// Get paragraph
    pub fn get_paragraph(&self, paragraph_id: &str) -> Option<&Paragraph> {
        self.paragraphs.iter().find(|p| p.id == paragraph_id)
    }
    
    /// Get paragraph mutably
    pub fn get_paragraph_mut(&mut self, paragraph_id: &str) -> Option<&mut Paragraph> {
        self.paragraphs.iter_mut().find(|p| p.id == paragraph_id)
    }
    
    /// Delete paragraph
    pub fn delete_paragraph(&mut self, paragraph_id: &str) -> bool {
        if let Some(pos) = self.paragraphs.iter().position(|p| p.id == paragraph_id) {
            self.paragraphs.remove(pos);
            self.modified_at = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            true
        } else {
            false
        }
    }
    
    /// Add style
    pub fn add_style(&mut self, style: Style) {
        self.styles.insert(style.name.clone(), style);
    }
    
    /// Get style
    pub fn get_style(&self, name: &str) -> Option<&Style> {
        self.styles.get(name)
    }
    
    /// Set document format
    pub fn set_format(&mut self, format: DocumentFormat) {
        self.format = format;
        self.modified_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Set author
    pub fn set_author(&mut self, author: String) {
        self.author = author;
        self.modified_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Set metadata
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.modified_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Get metadata
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
    
    /// Get plain text content
    pub fn get_plain_text(&self) -> String {
        self.paragraphs.iter()
            .map(|p| p.get_plain_text())
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    /// Get word count
    pub fn get_word_count(&self) -> usize {
        self.get_plain_text().split_whitespace().count()
    }
    
    /// Get character count
    pub fn get_character_count(&self) -> usize {
        self.get_plain_text().chars().count()
    }
    
    /// Get paragraph count
    pub fn get_paragraph_count(&self) -> usize {
        self.paragraphs.len()
    }
    
    /// Search text
    pub fn search_text(&self, query: &str) -> Vec<usize> {
        let mut results = vec![];
        for (i, paragraph) in self.paragraphs.iter().enumerate() {
            if paragraph.get_plain_text().to_lowercase().contains(&query.to_lowercase()) {
                results.push(i);
            }
        }
        results
    }
    
    /// Replace text
    pub fn replace_text(&mut self, old_text: &str, new_text: &str) -> usize {
        let mut count = 0;
        for paragraph in &mut self.paragraphs {
            let text = paragraph.get_plain_text();
            if text.contains(old_text) {
                let new_para_text = text.replace(old_text, new_text);
                paragraph.text = new_para_text;
                paragraph.runs.clear(); // Clear runs since we replaced text
                count += 1;
            }
        }
        if count > 0 {
            self.modified_at = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
        count
    }
}


