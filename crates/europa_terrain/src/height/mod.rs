use std::sync::Arc;

/// continuous height function on the XZ plane
pub trait HeightSource: Send + Sync + 'static {
    fn height_at(&self, x: f32, z: f32) -> f32;
}

pub type HeightFn = Arc<dyn HeightSource>;

pub mod comb;
pub mod noise;
pub mod warp;

pub fn arc<S: HeightSource>(s: S) -> HeightFn {
    Arc::new(s)
}
