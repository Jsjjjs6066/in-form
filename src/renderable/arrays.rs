use crate::Renderable;
use crate::Component::{
    self,
    *,
};

impl<T: Renderable, const N: usize> Renderable for [T; N] {
    fn get_component() -> Component {
        Array { ty: Box::new(T::get_component()), count: N }
    }
}

impl<T: Renderable> Renderable for std::vec::Vec<T> {
    fn get_component() -> Component {
        Vec(Box::new(T::get_component()))
    }
}
