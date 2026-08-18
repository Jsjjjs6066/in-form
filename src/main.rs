use in_form::Renderable;

#[derive(Renderable)]
pub struct Test {
    pub url: String,
    pub force: bool,
}

fn main() {
    println!("{:#?}", Test::get_component());
}
