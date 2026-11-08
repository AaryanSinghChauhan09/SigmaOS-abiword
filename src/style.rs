//! Style definitions for abiword.

/// Font style
#[derive(Debug, Clone, Default)]
pub struct FontStyle {
    /// Bold
    pub bold: bool,
    /// Italic
    pub italic: bool,
    /// Underline
    pub underline: bool,
    /// Strikethrough
    pub strikethrough: bool,
    /// Font name
    pub font_name: String,
    /// Font size
    pub font_size: f32,
    /// Font color (RGB)
    pub font_color: (u8, u8, u8),
    /// Background color (RGB)
    pub background_color: Option<(u8, u8, u8)>,
}

/// Paragraph alignment
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ParagraphAlignment {
    /// Left aligned
    #[default]
    Left,
    /// Center aligned
    Center,
    /// Right aligned
    Right,
    /// Justified
    Justified,
}

/// Paragraph style
#[derive(Debug, Clone, Default)]
pub struct ParagraphStyle {
    /// Alignment
    pub alignment: ParagraphAlignment,
    /// Line spacing
    pub line_spacing: f32,
    /// Space before
    pub space_before: f32,
    /// Space after
    pub space_after: f32,
    /// Indentation
    pub indentation: f32,
    /// Bullets
    pub bullets: bool,
    /// Numbering
    pub numbering: bool,
}

/// Style types
#[derive(Debug, Clone, PartialEq)]
pub enum StyleType {
    /// Character style
    Character,
    /// Paragraph style
    Paragraph,
    /// Page style
    Page,
    /// Table style
    Table,
}

/// Style
#[derive(Debug, Clone)]
pub struct Style {
    /// Style name
    pub name: String,
    /// Style type
    pub style_type: StyleType,
    /// Font style
    pub font_style: FontStyle,
    /// Paragraph style
    pub paragraph_style: ParagraphStyle,
    /// Based on style
    pub based_on: Option<String>,
    /// Next style
    pub next_style: Option<String>,
}

impl Style {
    /// Create new style
    pub fn new(name: String, style_type: StyleType) -> Self {
        Self {
            name,
            style_type,
            font_style: FontStyle::default(),
            paragraph_style: ParagraphStyle::default(),
            based_on: None,
            next_style: None,
        }
    }
    
    /// Set font style
    pub fn set_font_style(&mut self, font_style: FontStyle) {
        self.font_style = font_style;
    }
    
    /// Set paragraph style
    pub fn set_paragraph_style(&mut self, paragraph_style: ParagraphStyle) {
        self.paragraph_style = paragraph_style;
    }
    
    /// Set based on style
    pub fn set_based_on(&mut self, based_on: String) {
        self.based_on = Some(based_on);
    }
    
    /// Set next style
    pub fn set_next_style(&mut self, next_style: String) {
        self.next_style = Some(next_style);
    }
}
