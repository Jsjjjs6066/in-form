use crate::impl_renderable;
use crate::Renderable;
use crate::Component::{
    self,
    *,
};

impl_renderable!(f32 f {
    Float {
        min: f32::MIN as f64,
        max: f32::MAX as f64,
    }
});

impl_renderable!(f64 f {
    Float {
        min: f64::MIN,
        max: f64::MAX,
    }
});
