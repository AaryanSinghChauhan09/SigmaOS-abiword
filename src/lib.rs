//! SigmaOS-abiword — comprehensive word processor.
//!
//! Provides word processing functionality with document management,
//! rich text formatting, styles, and document operations.

pub mod document;
pub mod style;
pub mod manager;
pub mod options;

pub use document::{Document, Paragraph, TextRun, DocumentFormat};
pub use style::{Style, StyleType, FontStyle, ParagraphStyle};
pub use manager::{DocumentManager, DocumentManagerOptions};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_document_creation() {
        let doc = Document::new("Test Document".into());
        assert_eq!(doc.title, "Test Document");
        assert_eq!(doc.format, DocumentFormat::Docx);
        assert!(doc.paragraphs.is_empty());
    }
    
    #[test]
    fn test_add_paragraph() {
        let mut doc = Document::new("Test".into());
        let para_id = doc.add_paragraph("Hello World".into());
        
        assert_eq!(doc.paragraphs.len(), 1);
        assert_eq!(doc.paragraphs[0].text, "Hello World");
    }
    
    #[test]
    fn test_add_text_run() {
        let mut doc = Document::new("Test".into());
        let para_id = doc.add_paragraph("".into());
        
        doc.add_text_run(&para_id, "Hello".into(), FontStyle::default());
        doc.add_text_run(&para_id, " World".into(), FontStyle { bold: true, ..Default::default() });
        
        let para = doc.get_paragraph(&para_id).unwrap();
        assert_eq!(para.runs.len(), 2);
        assert!(para.runs[1].style.bold);
    }
    
    #[test]
    fn test_style_creation() {
        let style = Style::new("Heading1".into(), StyleType::Paragraph);
        assert_eq!(style.name, "Heading1");
        assert_eq!(style.style_type, StyleType::Paragraph);
    }
    
    #[test]
    fn test_document_manager_creation() {
        let mut manager = DocumentManager::new();
        assert!(manager.list_documents().is_empty());
    }
    
    #[test]
    fn test_create_document() {
        let mut manager = DocumentManager::new();
        let doc_id = manager.create_document("Test".into());
        
        assert!(manager.get_document(&doc_id).is_some());
        assert_eq!(manager.list_documents().len(), 1);
    }
    
    #[test]
    fn test_save_document() {
        let mut manager = DocumentManager::new();
        let doc_id = manager.create_document("Test".into());
        
        let mut doc = manager.get_document(&doc_id).unwrap().clone();
        doc.add_paragraph("Content".into());
        manager.save_document(doc);
        
        let saved_doc = manager.get_document(&doc_id).unwrap();
        assert_eq!(saved_doc.paragraphs.len(), 1);
    }
    
    #[test]
    fn test_delete_document() {
        let mut manager = DocumentManager::new();
        let doc_id = manager.create_document("Test".into());
        
        assert!(manager.delete_document(&doc_id));
        assert!(manager.get_document(&doc_id).is_none());
    }
    
    #[test]
    fn test_search_documents() {
        let mut manager = DocumentManager::new();
        let doc1 = manager.create_document("Test Document".into());
        let doc2 = manager.create_document("Other Document".into());
        
        let results = manager.search_documents("Test");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, doc1);
    }
    
    #[test]
    fn test_document_statistics() {
        let mut manager = DocumentManager::new();
        manager.create_document("Doc1".into());
        manager.create_document("Doc2".into());
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_documents, 2);
        assert_eq!(stats.total_words, 0); // No content yet
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = DocumentManager::new();
        let doc_id = manager.create_document("Test".into());
        
        let mut doc = manager.get_document(&doc_id).unwrap().clone();
        doc.add_paragraph("Content".into());
        manager.save_document(doc);
        
        let export = manager.export_data();
        
        let mut manager2 = DocumentManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_documents().len(), 1);
        assert_eq!(manager2.get_document(&doc_id).unwrap().paragraphs.len(), 1);
    }
    
    #[test]
    fn test_document_formats() {
        let formats = vec![
            DocumentFormat::Docx,
            DocumentFormat::Odt,
            DocumentFormat::Txt,
            DocumentFormat::Pdf,
            DocumentFormat::Html,
        ];
        assert_eq!(formats.len(), 5);
    }
    
    #[test]
    fn test_style_types() {
        let types = vec![
            StyleType::Character,
            StyleType::Paragraph,
            StyleType::Page,
            StyleType::Table,
        ];
        assert_eq!(types.len(), 4);
    }
}
