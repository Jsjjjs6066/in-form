pub mod ints;
pub mod floats;
pub mod misc;
pub mod arrays;
use crate::Component;

pub trait Renderable {
    fn get_component() -> Component;
}

#[macro_export]
macro_rules! impl_renderable {
    ($ty:ident f { $($fn:tt)* }) => {
        impl Renderable for $ty {
            fn get_component() -> Component {
                $($fn)*
            }
        }
    }
}

