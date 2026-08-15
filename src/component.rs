#[derive(Clone, Debug)]
pub enum Component {
    Text,
    Number,
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
