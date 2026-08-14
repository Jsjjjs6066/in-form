use crate::{
    Formable,
    Renderable,
};

impl<T: Renderable, const N: usize> Renderable for [T; N] {
    fn render<F: Formable>(f: &F) -> Self {
        std::array::from_fn(|_| T::render(f))
    }
}

impl<T> Renderable for Vec<T> {
    fn render<F: Formable>(f: &F) -> Self {
        f.render_vec()
    }
}
