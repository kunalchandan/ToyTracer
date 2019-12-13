// Display Crates
extern crate image as im;
extern crate imageproc as proc;
extern crate piston_window;

// Math Crates
extern crate nalgebra as nl;

// Std tings
use std::borrow::BorrowMut;

// How to clutter workspace 101
use piston_window::*;
// Internal Crates
use crate::ray::*;
use crate::world::*;
use crate::display::*;

mod ray;
mod world;
mod display;

// INTERNAL CONSTANTS

// Camera Position
pub const CAM_X: f32 = 0.0;
pub const CAM_Y: f32 = 0.0;
pub const CAM_Z: f32 = 0.0;
// Camera rotation
pub const CAM_T: f32 = 0.0; // Theta ( Right Left plane )
pub const CAM_P: f32 = 0.0; // Phi ( Up down plane)

pub const SCR_X: f32 = 1.0; // Right left width of screen
pub const SCR_Y: f32 = 1.0; // Up down width of screen
pub const SCR_Z: f32 = 0.0; // How far forwards the screen is


pub struct Tracer {
    objects: Vec<Box<dyn Traceable>>,
    canvas: im::ImageBuffer<im::Rgba<u8>, Vec<u8>>,
}

impl Tracer {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            canvas: im::ImageBuffer::new(WIDTH, HEIGHT),
        }
    }

    pub fn add_object<S: Traceable + 'static>(&mut self, object: S) -> &mut Self {
        self.objects.push(Box::new(object));
        return self;
    }

    pub fn create_world(&mut self) {
        self.add_object( Plane {
            a: 1.0,
            b: 2.0,
            c: 3.0,
            d: 4.0
        });
        self.add_object(Plane {
            a: 0.0,
            b: 1.0,
            c: 2.0,
            d: 3.0
        });
        self.add_object(Sphere {
            x0: 2.0,
            y0: 2.0,
            z0: 0.0,
            r: 3.0
        });
        self.add_object(Sphere {
            x0: 1.0,
            y0: 1.0,
            z0: 1.0,
            r: 1.0
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.set_hit_img();
    }

    pub fn draw_background(&mut self) {
        for j in 0..HEIGHT {
            for i in 0..WIDTH {
                let r = (255.0 * (j as f32 / HEIGHT as f32 )) as u8;
                let b = (255.0 * (i as f32 / WIDTH  as f32 )) as u8;
                let g = (255.0 * ((i + j) as f32 / (WIDTH + HEIGHT) as f32 )) as u8;
                *self.canvas.get_pixel_mut(i, j) = im::Rgba([r, b, g, 255])
            }
        }
    }

    pub fn set_hit_img(&mut self) {
        let cam_pos: nl::Vector3<f32> = nl::Vector3::new(CAM_X, CAM_Y, CAM_Z);

        // Let is draw the background.
        // self.get_background(i, j);

        for j in 0..HEIGHT {
            for i in 0..WIDTH {
                // Create Ray from position to screen
                // TODO:: Factor in Camera rotation; Rotation matrix must be applied to scr_pos vector..
                let scr_pos: nl::Vector3<f32> = nl::Vector3::new((i as f32)*SCR_X/(WIDTH as f32),
                                                                 (j as f32)*SCR_Y/(HEIGHT as f32),
                                                                 SCR_Z) + cam_pos;
                let r = create_ray(cam_pos, scr_pos, RAY_BOUNCE_MAX);
                for obj in self.objects.iter() {
                    let r_new = obj.trace(r);
                    if r_new.count == r.count {
                    }
                    else {
                        // Draw to pixel here the colour of the object
                        *self.canvas.get_pixel_mut(i, j) = im::Rgba([0, 255, 0, 255])
                    }
                    // If ray intersects object
                    // Find normal of object at this point
                    // Find partial derivative of normal on each axis??
                    // define colour of point recursively by tracing again from here to wherever the
                    // ray goes next until it hits nothing or a light source
                }
            }
        }
    }
}


fn create_ray(o: nl::Vector3<f32>, q: nl::Vector3<f32>, count: u8) -> Ray {
    let d: nl::Vector3<f32> = q - o;
    return Ray {
        o,
        d,
        count
    }
}


fn main() {

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: PistonWindow =
        WindowSettings::new(
            "Toy Tracer",
            [WIDTH, HEIGHT])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();


    let mut tracer: Tracer = Tracer::new();
    tracer.create_world();
    tracer.set_hit_img();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        e.mouse_cursor(|pos| {
            // Update cursor position
        });

        if let Some(r) = e.render_args() {

            tracer.draw_background();
            tracer.set_hit_img();
            let mut texture_context = TextureContext {
                factory: window.factory.clone(),
                encoder: window.factory.create_command_buffer().into()
            };

            let mut texture: G2dTexture = Texture::from_image(
                &mut texture_context,
                &tracer.canvas,
                &TextureSettings::new()
            ).unwrap();

            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });

            texture.update(&mut texture_context, &tracer.canvas).unwrap();
        }

        if let Some(u) = e.update_args() {
            tracer.update(&u);
        }
    }

}
