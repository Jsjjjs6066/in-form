use crate::{
    impl_renderable,
    Formable,
    Renderable,
};

impl_renderable!(f32 f {
    f.render_f32()
});

impl_renderable!(f64 f {
    f.render_f64()
});
