pub trait Formable {
    fn render_string(&self) -> String;
    fn render_bool(&self) -> bool;
    fn render_char(&self) -> char;
    fn render_vec<T>(&self) -> Vec<T>;

    fn render_i8(&self) -> i8;
    fn render_u8(&self) -> u8;
    fn render_i16(&self) -> i16;
    fn render_u16(&self) -> u16;
    fn render_i32(&self) -> i32;
    fn render_u32(&self) -> u32;
    fn render_i64(&self) -> i64;
    fn render_u64(&self) -> u64;

    fn render_f32(&self) -> f32;
    fn render_f64(&self) -> f64;
}
