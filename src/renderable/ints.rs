use crate::impl_renderable;
use crate::Renderable;
use crate::Component::{
    self,
    *,
};

macro_rules! impl_int {
    ($ty:ident) => {
        impl_renderable!($ty f { Int {
            min: $ty::MIN as i64,
            max: $ty::MAX as u64,
        } });
    }
}

impl_int!(i8);
impl_int!(u8);
impl_int!(i16);
impl_int!(u16);
impl_int!(i32);
impl_int!(u32);
impl_int!(i64);
impl_int!(u64);
