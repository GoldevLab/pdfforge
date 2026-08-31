use resuma::prelude::*;
use resuma::FlowPageRegistry;

pub struct PagesRegistry;

impl FlowPageRegistry for PagesRegistry {
    fn routes(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("/", "index"),
            ("/pdf-compress", "pdf_compress"),
            ("/pdf-merge", "pdf_merge"),
            ("/pdf-split", "pdf_split"),
            ("/pdf-to-jpg", "pdf_to_jpg"),
            ("/pdf-to-text", "pdf_to_text"),
            ("/app", "app"),
            ("/app/compress", "app_compress"),
            ("/app/merge", "app_merge"),
            ("/app/split", "app_split"),
            ("/app/to-jpg", "app_to_jpg"),
            ("/app/to-text", "app_to_text"),
        ]
    }

    fn layout_for(&self, _pattern: &str) -> &'static [&'static str] {
        &["/"]
    }

    fn render(&self, module: &str, req: FlowRequest) -> Option<View> {
        match module {
            "index" => Some(super::index::page(req)),
            "pdf_compress" => Some(super::pdf_compress::page(req)),
            "pdf_merge" => Some(super::pdf_merge::page(req)),
            "pdf_split" => Some(super::pdf_split::page(req)),
            "pdf_to_jpg" => Some(super::pdf_to_jpg::page(req)),
            "pdf_to_text" => Some(super::pdf_to_text::page(req)),
            "app" => Some(super::app::page(req)),
            "app_compress" => Some(super::app_compress::page(req)),
            "app_merge" => Some(super::app_merge::page(req)),
            "app_split" => Some(super::app_split::page(req)),
            "app_to_jpg" => Some(super::app_to_jpg::page(req)),
            "app_to_text" => Some(super::app_to_text::page(req)),
            _ => None,
        }
    }
}
