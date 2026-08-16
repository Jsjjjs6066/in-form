use crate::NamedComponent;

pub trait Formable {
    fn render_form(&self, c: &NamedComponent) -> String;
}
