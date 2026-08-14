pub mod ints;
pub mod floats;
pub mod misc;
pub mod arrays;
use crate::formable::Formable;

pub trait Renderable {
    fn render<F: Formable>(f: &F) -> Self;
}

#[macro_export]
macro_rules! impl_renderable {
    ($ty:ident $f:ident { $($fn:tt)* }) => {
        impl Renderable for $ty {
            fn render<F: Formable>($f: &F) -> Self {
                $($fn)*
            }
        }
    }
}

