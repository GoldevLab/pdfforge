use resuma::prelude::*;

use crate::family::Tool;
use crate::landing::{self, Surface};

pub fn page(_req: FlowRequest) -> View {
    landing::page(Tool::ToJpg, Surface::Seo)
}
