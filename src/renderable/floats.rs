use crate::impl_renderable;
use crate::Renderable;
use crate::Component::{
    self,
    *,
};

impl_renderable!(f32 f {
    Number
});

impl_renderable!(f64 f {
    Number
});
