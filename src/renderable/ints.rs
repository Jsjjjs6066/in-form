use crate::impl_renderable;
use crate::Renderable;
use crate::Formable;

impl_renderable!(i8 f {
    f.render_i8()
});

impl_renderable!(u8 f {
    f.render_u8()
});

impl_renderable!(i16 f {
    f.render_i16()
});

impl_renderable!(u16 f {
    f.render_u16()
});

impl_renderable!(i32 f {
    f.render_i32()
});

impl_renderable!(u32 f {
    f.render_u32()
});

impl_renderable!(i64 f {
    f.render_i64()
});

impl_renderable!(u64 f {
    f.render_u64()
});
