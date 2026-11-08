//! Document manager for abiword.

use std::collections::HashMap;
use crate::document::{Document, DocumentFormat};

/// Document manager options
#[derive(Debug, Clone)]
pub struct DocumentManagerOptions {
    /// Maximum documents
    pub max_documents: usize,
    /// Auto-save interval in seconds
    pub auto_save_interval: u64,
    /// Enable auto-save
    pub auto_save_enabled: bool,
    /// Default document format
    pub default_format: DocumentFormat,
}

impl Default for DocumentManagerOptions {
    fn default() -> Self {
        Self {
            max_documents: 100,
            auto_save_interval: 300,
            auto_save_enabled: true,
            default_format: DocumentFormat::Docx,
        }
    }
}

/// Document statistics
#[derive(Debug, Clone)]
pub struct DocumentStatistics {
    /// Total documents
    pub total_documents: usize,
    /// Total words
    pub total_words: usize,
    /// Total characters
    pub total_characters: usize,
    /// Total paragraphs
    pub total_paragraphs: usize,
    /// Average words per document
    pub avg_words_per_document: f64,
}

/// Document manager
#[derive(Debug, Clone)]
pub struct DocumentManager {
    /// Documents
    documents: HashMap<String, Document>,
    /// Options
    options: DocumentManagerOptions,
    /// Statistics
    statistics: DocumentStatistics,
}

impl Default for DocumentManager {
    fn default() -> Self {
        Self {
            documents: HashMap::new(),
            options: DocumentManagerOptions::default(),
            statistics: DocumentStatistics {
                total_documents: 0,
                total_words: 0,
                total_characters: 0,
                total_paragraphs: 0,
                avg_words_per_document: 0.0,
            },
        }
    }
}

impl DocumentManager {
    /// Create new document manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create document manager with options
    pub fn with_options(options: DocumentManagerOptions) -> Self {
        let mut manager = Self::default();
        manager.options = options;
        manager
    }
    
    /// Create new document
    pub fn create_document(&mut self, title: String) -> String {
        let mut doc = Document::new(title);
        doc.format = self.options.default_format.clone();
        
        // Generate unique ID
        let id = format!("doc-{}-{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos(), self.documents.len());
        doc.id = id.clone();
        
        self.documents.insert(id.clone(), doc);
        self.update_statistics();
        id
    }
    
    /// Open document
    pub fn open_document(&mut self, document: Document) -> String {
        let id = document.id.clone();
        self.documents.insert(id.clone(), document);
        self.update_statistics();
        id
    }
    
    /// Save document
    pub fn save_document(&mut self, document: Document) -> bool {
        let id = document.id.clone();
        self.documents.insert(id, document);
        self.update_statistics();
        true
    }
    
    /// Delete document
    pub fn delete_document(&mut self, document_id: &str) -> bool {
        if self.documents.remove(document_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get document
    pub fn get_document(&self, document_id: &str) -> Option<&Document> {
        self.documents.get(document_id)
    }
    
    /// Get document mutably
    pub fn get_document_mut(&mut self, document_id: &str) -> Option<&mut Document> {
        self.documents.get_mut(document_id)
    }
    
    /// List all documents
    pub fn list_documents(&self) -> Vec<&Document> {
        self.documents.values().collect()
    }
    
    /// Search documents
    pub fn search_documents(&self, query: &str) -> Vec<&Document> {
        let query_lower = query.to_lowercase();
        self.documents.values()
            .filter(|doc| {
                doc.title.to_lowercase().contains(&query_lower) ||
                doc.get_plain_text().to_lowercase().contains(&query_lower)
            })
            .collect()
    }
    
    /// Get documents by author
    pub fn get_documents_by_author(&self, author: &str) -> Vec<&Document> {
        self.documents.values()
            .filter(|doc| doc.author == author)
            .collect()
    }
    
    /// Get documents by format
    pub fn get_documents_by_format(&self, format: &DocumentFormat) -> Vec<&Document> {
        self.documents.values()
            .filter(|doc| doc.format == *format)
            .collect()
    }
    
    /// Update document statistics
    fn update_statistics(&mut self) {
        self.statistics.total_documents = self.documents.len();
        self.statistics.total_words = self.documents.values()
            .map(|doc| doc.get_word_count())
            .sum();
        self.statistics.total_characters = self.documents.values()
            .map(|doc| doc.get_character_count())
            .sum();
        self.statistics.total_paragraphs = self.documents.values()
            .map(|doc| doc.get_paragraph_count())
            .sum();
        
        if self.statistics.total_documents > 0 {
            self.statistics.avg_words_per_document = self.statistics.total_words as f64 / self.statistics.total_documents as f64;
        }
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &DocumentStatistics {
        &self.statistics
    }
    
    /// Export document data
    pub fn export_data(&self) -> Vec<Document> {
        self.documents.values().cloned().collect()
    }
    
    /// Import document data
    pub fn import_data(&mut self, documents: Vec<Document>) {
        self.documents.clear();
        for doc in documents {
            self.documents.insert(doc.id.clone(), doc);
        }
        self.update_statistics();
    }
    
    /// Clean up old documents
    pub fn cleanup_old_documents(&mut self, max_age_secs: u64) -> usize {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let old_docs: Vec<String> = self.documents.values()
            .filter(|doc| now - doc.modified_at > max_age_secs)
            .map(|doc| doc.id.clone())
            .collect();
        
        let count = old_docs.len();
        for id in old_docs {
            self.documents.remove(&id);
        }
        
        self.update_statistics();
        count
    }
}
