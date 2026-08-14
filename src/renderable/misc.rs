use crate::{
    impl_renderable,
    Formable,
    Renderable,
};

impl_renderable!(String f {
    f.render_string()
});

impl_renderable!(bool f {
    f.render_bool()
});

impl_renderable!(char f {
    f.render_char()
});
