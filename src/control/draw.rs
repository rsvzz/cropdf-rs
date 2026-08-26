use cairo::Context;

pub trait DrawingExt{
    /// draw cotrol
    /// * `context` cairo
    fn draw(&self, ctx: Option<&Context>);
}