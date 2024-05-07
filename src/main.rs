#[macro_use]
extern crate glium;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let events_loop = glium::glutin::event_loop::EventLoop::new();

    let window = glium::glutin::window::WindowBuilder::new().with_inner_size(glium::glutin::dpi::LogicalSize::new(1600.0, 900.0))
        .with_title("Modern Era");

    let context = glium::glutin::ContextBuilder::new().with_depth_buffer(24).
        with_stencil_buffer(8).
        with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let texture_array = glium::texture::SrgbTexture2dArray::empty_with_format(
        &display,
        glium::texture::SrgbFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        2048,
        2048,
        100,
    ).unwrap();
    println!("Success");
    Ok(())
}
