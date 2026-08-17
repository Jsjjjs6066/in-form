use crate::Formable;

#[derive(Clone, Debug)]
pub enum Component {
    Text,
    Int { min: i64, max: u64 },
    Float { min: f64, max: f64 },
    Bool,
    Char,
    Array { ty: Box<Component>, count: usize },
    Vec(Box<Component>),
    FieldStruct(Vec<NamedComponent>),
}

#[derive(Clone, Debug)]
pub struct NamedComponent {
    pub name: &'static str,
    pub comp: Component,
}

impl NamedComponent {
    pub fn render<F: Formable>(&self, f: &F) -> String {
        f.render_form(self)
    }
}
