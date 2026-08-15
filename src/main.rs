use in_form::Renderable;
use in_form_derive::Renderable;

#[derive(Renderable)]
pub struct Test {
    pub url: String,
    pub force: bool,
}

fn main() {
    println!("{:#?}", Test::get_component());
}
