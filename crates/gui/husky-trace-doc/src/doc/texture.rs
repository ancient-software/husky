use egui::TextureHandle;
use husky_visual_protocol::visual::image::ImageVisual;
use rustc_hash::FxHashMap;

#[derive(Default)]
pub struct TextureCache {
    //todo: optimize by LRU??
    texture_handles: FxHashMap<TextureCacheKey, TextureHandle>,
}

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TextureCacheKey {
    ImageVisual(ImageVisual),
}

impl<I> std::ops::Index<I> for TextureCache
where
    I: Into<TextureCacheKey> + Eq,
{
    type Output = TextureHandle;

    fn index(&self, i: I) -> &Self::Output {
        &self.texture_handles[&i.into()]
    }
}
