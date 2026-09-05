//! PDFForge jobs: SEO landings vs compact /app tools.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Compress,
    Merge,
    Split,
    FromJpg,
    ToJpg,
    ToText,
}

impl Tool {
    pub fn all() -> [Tool; 6] {
        [
            Self::Compress,
            Self::Merge,
            Self::Split,
            Self::FromJpg,
            Self::ToJpg,
            Self::ToText,
        ]
    }

    pub fn path(self) -> &'static str {
        match self {
            Self::Compress => "/pdf-compress",
            Self::Merge => "/pdf-merge",
            Self::Split => "/pdf-split",
            Self::FromJpg => "/jpg-to-pdf",
            Self::ToJpg => "/pdf-to-jpg",
            Self::ToText => "/pdf-to-text",
        }
    }

    pub fn app_path(self) -> &'static str {
        match self {
            Self::Compress => "/app/compress",
            Self::Merge => "/app/merge",
            Self::Split => "/app/split",
            Self::FromJpg => "/app/from-jpg",
            Self::ToJpg => "/app/to-jpg",
            Self::ToText => "/app/to-text",
        }
    }

    pub fn nav_label(self) -> &'static str {
        match self {
            Self::Compress => "Compress",
            Self::Merge => "Merge",
            Self::Split => "Split",
            Self::FromJpg => "JPG→PDF",
            Self::ToJpg => "PDF→JPG",
            Self::ToText => "PDF→Text",
        }
    }

    pub fn card_mark(self) -> &'static str {
        match self {
            Self::Compress => "kB",
            Self::Merge => "+",
            Self::Split => "✂",
            Self::FromJpg => "IMG",
            Self::ToJpg => "JPG",
            Self::ToText => "Aa",
        }
    }

    pub fn card_title(self) -> &'static str {
        match self {
            Self::Compress => "Compress PDF",
            Self::Merge => "Merge PDFs",
            Self::Split => "Split PDF",
            Self::FromJpg => "JPG to PDF",
            Self::ToJpg => "PDF to JPG",
            Self::ToText => "PDF to text",
        }
    }

    pub fn card_blurb(self) -> &'static str {
        match self {
            Self::Compress => "Rewrite the file and drop unused objects. No account.",
            Self::Merge => "Stack several PDFs into one document, in the order you pick.",
            Self::Split => "Keep a page range, or burst every page into a zip.",
            Self::FromJpg => "Turn JPG or PNG files into a PDF, one image per page.",
            Self::ToJpg => "Rasterize pages to JPG with a pure-Rust renderer.",
            Self::ToText => "Pull selectable text runs into a .txt download.",
        }
    }

    pub fn card_cta(self) -> &'static str {
        "Open tool"
    }
}
