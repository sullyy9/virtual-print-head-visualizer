use std::u32;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

type Colour = [u8; 4];

pub struct GraphicsWindow {
    window: Window,
    pub width: u32,
    pub height: u32,

    pixel_buffer: Pixels,
}
impl GraphicsWindow {
    ///
    /// Create a new graphics window.
    ///
    pub fn new(width: u32, height: u32, event_loop: &EventLoop<()>) -> GraphicsWindow {
        // Create the actual window
        let window = {
            let size = PhysicalSize::new(width, height);
            WindowBuilder::new()
                .with_title("3D Graphics")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        // Create a pixel buffer within the window
        let pixel_buffer = {
            let surface_texture = SurfaceTexture::new(width, height, &window);

            Pixels::new(width, height, surface_texture).expect("Error: create pixel buffer")
        };

        GraphicsWindow {
            window,
            width,
            height,
            pixel_buffer,
        }
    }

    ///
    /// Resize the pixel buffer to match the window size.
    ///
    pub fn resize(&mut self, width: u32, height: u32) {
        self.pixel_buffer.resize_surface(width, height);
        self.pixel_buffer.resize_buffer(width, height);

        self.width = width;
        self.height = height;
    }

    ///
    /// Clear the pixel buffer and z buffer.
    ///
    pub fn clear(&mut self) {
        for i in self.pixel_buffer.get_frame().iter_mut() {
            *i = 0;
        }
    }

    ///
    /// Set a pixels colour via x and y coordinates with the origin in the bottom left corner.
    ///
    pub fn draw_pixel(&mut self, x: u32, y: u32, colour: Colour) {
        // Figure out which 4 elements we need from the x and y coordinates then get a
        // pointer to the 4 elements which consitute the pixel.
        let element = {
            let y_invert = self.height - (y + 1);
            (((y_invert * self.width) + x) * 4) as usize
        };

        let pixel = self
            .pixel_buffer
            .get_frame()
            .get_mut(element..(element + 4))
            .unwrap();

        pixel.copy_from_slice(&colour);
    }

    ///
    /// Render the pixel buffer to the screen.
    ///
    pub fn render(&mut self) {
        match self.pixel_buffer.render() {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to render pixel buffer")
            }
        }
    }

    ///
    /// Redraw the window.
    ///
    pub fn redraw(&self) {
        // Trgger a redraw event.
        self.window.request_redraw();
    }
}
