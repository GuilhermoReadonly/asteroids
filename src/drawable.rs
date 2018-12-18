use sdl2::render::WindowCanvas;

pub trait Drawable {
    fn draw(&self, canvas: & mut WindowCanvas) -> ();
}
