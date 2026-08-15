use crate::impl_renderable;
use crate::Renderable;
use crate::Component::{
    self,
    *,
};

impl_renderable!(String f {
    Text
});

impl_renderable!(bool f {
    Bool
});

impl_renderable!(char f {
    Char
});
