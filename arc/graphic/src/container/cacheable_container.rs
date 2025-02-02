use core::AsAny;

pub trait CacheableContainer: AsAny {
    fn update_cache(&mut self, cache: crate::TextureCache);
    fn get_cache(&self) -> &crate::TextureCache;
}
