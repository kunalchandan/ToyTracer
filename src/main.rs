// Display Crates
extern crate image as im;
extern crate imageproc as proc;
extern crate piston_window;

// Math Crates
extern crate nalgebra as nl;

// Std tings
use std::borrow::Borrow;

// How to clutter workspace 101
use piston_window::*;
use std::f32::consts::*;

// Internal Crates
use crate::ray::*;
use crate::world::*;
use crate::display::*;
use std::f32::INFINITY;

mod ray;
mod world;
mod display;

// INTERNAL CONSTANTS

// Camera Position
pub const CAM_X: f32 = 0.0;
pub const CAM_Y: f32 = 0.0;
pub const CAM_Z: f32 = 0.0;
// Camera rotation
pub const CAM_T: f32 = PI; // Theta ( Right Left plane )
pub const CAM_P: f32 = FRAC_PI_2; // Phi ( Up down plane)

pub const SCR_T: f32 = FRAC_PI_3; // Right left width of screen
pub const SCR_P: f32 = FRAC_PI_6; // Up down width of screen
pub const SCR_Z: f32 = 1.0; // How far forwards the screen is


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
            x0: -7.0,
            y0: 2.0,
            z0: 0.0,
            r: 3.0
        });
        self.add_object(Sphere {
            x0: -5.0,
            y0: 0.0,
            z0: 0.0,
            r: 4.00
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

    pub fn recursive_trace(&self, r: Ray) -> im::Rgba<u8> {
        if r.count > 0 {
            let min_dist = INFINITY;
            let mut to_trace: &Box<dyn Traceable> = self.objects[0].borrow();
            let mut ray_new = r;
            for obj in self.objects.iter() {
                let r_new = obj.trace(r);
                if r_new.total_distance() < min_dist {
                    to_trace = obj;
                    ray_new = r_new;
                }
            }
            if r.count == ray_new.count {
                return im::Rgba([0, 0, 0, 255]);
            }
            return self.recursive_trace(ray_new);
        }
        else {
            return im::Rgba([63, 127, 255, 255]);
        }
    }

    pub fn set_hit_img(&mut self) {
        let cam_pos: nl::Vector3<f32> = nl::Vector3::new(CAM_X, CAM_Y, CAM_Z);

        // Centre
        let cn = nl::Vector3::new(CAM_X, CAM_Y, CAM_Z) + (SCR_Z * nl::Vector3::new(CAM_P.sin() * CAM_T.cos(),
                                                                                   CAM_P.sin() * CAM_T.sin(),
                                                                                   CAM_P.cos()));
        let ul = nl::Vector3::new(CAM_X, CAM_Y, CAM_Z) +
                (nl::Vector3::new((CAM_P - SCR_P).sin() * (CAM_T + SCR_T).cos(),
                                  (CAM_P - SCR_P).sin() * (CAM_T + SCR_T).sin(),
                                  (CAM_P - SCR_P).cos()) * SCR_Z);

        let ur = nl::Vector3::new(CAM_X, CAM_Y, CAM_Z) +
                (nl::Vector3::new((CAM_P - SCR_P).sin() * (CAM_T - SCR_T).cos(),
                                  (CAM_P - SCR_P).sin() * (CAM_T - SCR_T).sin(),
                                  (CAM_P - SCR_P).cos()) * SCR_Z);

        let bl = nl::Vector3::new(CAM_X, CAM_Y, CAM_Z) +
                (nl::Vector3::new((CAM_P + SCR_P).sin() * (CAM_T + SCR_T).cos(),
                                  (CAM_P + SCR_P).sin() * (CAM_T + SCR_T).sin(),
                                  (CAM_P + SCR_P).cos()) * SCR_Z);

        let br = nl::Vector3::new(CAM_X, CAM_Y, CAM_Z) +
                (nl::Vector3::new((CAM_P + SCR_P).sin() * (CAM_T - SCR_T).cos(),
                                  (CAM_P + SCR_P).sin() * (CAM_T - SCR_T).sin(),
                                  (CAM_P + SCR_P).cos()) * SCR_Z);

        let v_down = (bl - ul)/(HEIGHT as f32);
        let mut cur_y0 = ul;
        let mut cur_y1 = ur;
        for j in 0..HEIGHT {
            cur_y0 += v_down;
            cur_y1 += v_down;
            let v_right = (cur_y1 - cur_y0)/(WIDTH as f32);
            let mut scr_pos = cur_y0;
            for i in 0..WIDTH {
                scr_pos += v_right;
                // Create Ray from position to screen
                let r = create_ray(cam_pos, scr_pos, RAY_BOUNCE_MAX);
                let pix_colour = self.recursive_trace(r);
                for obj in self.objects.iter() {
                    let r_new = obj.trace(r);
                    if r_new.count != r.count {
                        // Draw to pixel here the colour of the object
                        let dist = r_new.total_distance() as u8;
                        *self.canvas.get_pixel_mut(i, j) = im::Rgba([63/dist, 127/dist, 255/dist, 255]);
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
        count,
        distances: [0.0; RAY_BOUNCE_MAX as usize],
//        collided: vec![]
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

    let mut obj_selector = 0;
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        e.mouse_cursor(|pos| {
            // Update cursor position
        });
        if let Some(Button::Keyboard(key)) = e.press_args() {
            tracer.objects[obj_selector].get_location();
            if key == Key::Left {
                println!("Move X-");
                tracer.objects[obj_selector].move_xn();
            }
            if key == Key::Right {
                println!("Move X+");
                tracer.objects[obj_selector].move_xp();
            }
            if key == Key::Up {
                println!("Move Y-");
                tracer.objects[obj_selector].move_yn();
            }
            if key == Key::Down {
                println!("Move Y+");
                tracer.objects[obj_selector].move_yp();
            }
            if key == Key::LShift {
                println!("Move Z-");
                tracer.objects[obj_selector].move_zn();
            }
            if key == Key::LCtrl {
                println!("Move Z+");
                tracer.objects[obj_selector].move_zp();
            }
            if key == Key::Tab {
                println!("Swap Objects");
                obj_selector += 1;
                obj_selector %= tracer.objects.len();
            }
        }

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
