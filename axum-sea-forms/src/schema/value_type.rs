//! Field value types.

use serde::{Deserialize, Serialize};

/// Supported field types for form fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueType {
    // Text inputs
    /// Single-line text input.
    Text,
    /// Email input with format validation.
    Email,
    /// URL input with format validation.
    Url,
    /// Telephone number input.
    Tel,
    /// Numeric input.
    Number,
    /// Multi-line text input.
    Textarea,

    // Selection
    /// Single-select dropdown.
    Select,
    /// Multi-select dropdown.
    MultiSelect,
    /// Radio button group.
    Radio,
    /// Single checkbox.
    Checkbox,

    // Date/Time
    /// Date picker.
    Date,
    /// Date and time picker.
    DateTime,
    /// Time picker.
    Time,

    // Files (Phase 2)
    /// File upload.
    File,
    /// Image upload.
    Image,

    // Display-only
    /// Hidden field.
    Hidden,
    /// Heading text (display only).
    Heading,
    /// Paragraph text (display only).
    Paragraph,

    // Survey (Phase 3)
    /// Star rating.
    Rating,
    /// Numeric scale (e.g., 1-10).
    Scale,
    /// Net Promoter Score (0-10).
    Nps,
    /// Matrix/grid of questions.
    Matrix,
}

impl ValueType {
    /// Returns the HTML input type for this field type.
    #[must_use]
    pub fn html_input_type(&self) -> &'static str {
        match self {
            Self::Text | Self::Hidden => "text",
            Self::Email => "email",
            Self::Url => "url",
            Self::Tel => "tel",
            Self::Number | Self::Rating | Self::Scale | Self::Nps => "number",
            Self::Date => "date",
            Self::DateTime => "datetime-local",
            Self::Time => "time",
            Self::File | Self::Image => "file",
            Self::Checkbox => "checkbox",
            Self::Radio => "radio",
            // These don't map directly to input types
            Self::Textarea | Self::Select | Self::MultiSelect | Self::Heading | Self::Paragraph | Self::Matrix => "",
        }
    }

    /// Returns true if this field type requires options (select, radio, etc.).
    #[must_use]
    pub fn requires_options(&self) -> bool {
        matches!(
            self,
            Self::Select | Self::MultiSelect | Self::Radio | Self::Matrix
        )
    }

    /// Returns true if this field type is display-only (no input value).
    #[must_use]
    pub fn is_display_only(&self) -> bool {
        matches!(self, Self::Heading | Self::Paragraph)
    }

    /// Returns true if this field type accepts file uploads.
    #[must_use]
    pub fn is_file_type(&self) -> bool {
        matches!(self, Self::File | Self::Image)
    }

    /// Returns true if this field type accepts multiple values.
    #[must_use]
    pub fn is_multi_value(&self) -> bool {
        matches!(self, Self::MultiSelect | Self::Checkbox | Self::Matrix)
    }
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Url => "url",
            Self::Tel => "tel",
            Self::Number => "number",
            Self::Textarea => "textarea",
            Self::Select => "select",
            Self::MultiSelect => "multi_select",
            Self::Radio => "radio",
            Self::Checkbox => "checkbox",
            Self::Date => "date",
            Self::DateTime => "datetime",
            Self::Time => "time",
            Self::File => "file",
            Self::Image => "image",
            Self::Hidden => "hidden",
            Self::Heading => "heading",
            Self::Paragraph => "paragraph",
            Self::Rating => "rating",
            Self::Scale => "scale",
            Self::Nps => "nps",
            Self::Matrix => "matrix",
        };
        write!(f, "{s}")
    }
}

impl std::str::FromStr for ValueType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(Self::Text),
            "email" => Ok(Self::Email),
            "url" => Ok(Self::Url),
            "tel" | "telephone" | "phone" => Ok(Self::Tel),
            "number" | "numeric" | "integer" | "decimal" => Ok(Self::Number),
            "textarea" | "longtext" | "long_text" => Ok(Self::Textarea),
            "select" | "dropdown" => Ok(Self::Select),
            "multi_select" | "multiselect" => Ok(Self::MultiSelect),
            "radio" => Ok(Self::Radio),
            "checkbox" => Ok(Self::Checkbox),
            "date" => Ok(Self::Date),
            "datetime" | "date_time" => Ok(Self::DateTime),
            "time" => Ok(Self::Time),
            "file" => Ok(Self::File),
            "image" => Ok(Self::Image),
            "hidden" => Ok(Self::Hidden),
            "heading" | "header" | "h1" | "h2" | "h3" => Ok(Self::Heading),
            "paragraph" | "text_block" | "description" => Ok(Self::Paragraph),
            "rating" | "stars" => Ok(Self::Rating),
            "scale" | "slider" => Ok(Self::Scale),
            "nps" => Ok(Self::Nps),
            "matrix" | "grid" => Ok(Self::Matrix),
            _ => Err(format!("Unknown field type: {s}")),
        }
    }
}
