pub mod app;
pub mod app_compress;
pub mod app_merge;
pub mod app_split;
pub mod app_to_jpg;
pub mod app_to_text;
pub mod index;
pub mod pdf_compress;
pub mod pdf_merge;
pub mod pdf_split;
pub mod pdf_to_jpg;
pub mod pdf_to_text;

mod _registry;
pub use _registry::PagesRegistry;
