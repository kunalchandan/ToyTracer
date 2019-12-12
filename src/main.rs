extern crate image as im;
extern crate imageproc as proc;
extern crate nalgebra as nl;

use crate::ray::*;
use crate::world::*;
use crate::display::*;

mod ray;
mod world;
mod display;

// Camera Position
pub const CAM_X: f32 = 0.0;
pub const CAM_Y: f32 = 0.0;
pub const CAM_Z: f32 = 0.0;
// Camera rotation
pub const CAM_T: f32 = 0.0; // Theta ( Right Left plane )
pub const CAM_P: f32 = 0.0; // Phi ( Up down plane)

pub const SCR_X: f32 = 1.0; // Right left width of screen
pub const SCR_Y: f32 = 1.0; // Up down width of screen
pub const SCR_Z: f32 = 0.5; // How far forwards the screen is


pub struct World {
    pub components: Vec<Box<dyn Traceable>>
}

impl World where {
    fn push<S: Traceable + 'static>(&mut self, component: S) -> &mut Self {
        self.components.push(Box::new(component));
        return self;
    }
    fn new () -> Self {
        Self {
            components: Vec::new()
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
    let cam_pos: nl::Vector3<f32> = nl::Vector3::new(CAM_X, CAM_Y, CAM_Z);
    let mut world = World::new();
    println!("0");

    let p1 = Plane {
        a: 1.0,
        b: 2.0,
        c: 3.0,
        d: 4.0
    };
    let p2 = Plane {
        a: 0.0,
        b: 1.0,
        c: 2.0,
        d: 3.0
    };
    let s1 = Sphere {
        x0: 2.0,
        y0: 2.0,
        z0: 0.0,
        r: 3.0
    };
    let s2 = Sphere {
        x0: 1.0,
        y0: 1.0,
        z0: 1.0,
        r: 1.0
    };
    world.push(p1);
    println!("1");
    world.push(p2);
    println!("2");
    world.push(s1);
    println!("3");
    world.push(s2);
    println!("4");
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            // Create Ray from position to screen
            // TODO:: Factor in Camera rotation; Rotation matrix must be applied to scr_pos vector..
            let scr_pos: nl::Vector3<f32> = nl::Vector3::new((i as f32)*SCR_X/(WIDTH as f32),
                                                             (j as f32)*SCR_Y/(HEIGHT as f32),
                                                             SCR_Z) + cam_pos;
            let r = create_ray(cam_pos, scr_pos, RAY_BOUNCE_MAX);
            for obj in world.components.iter() {
                let r_new = obj.trace(r);
                if r_new.count == r.count {
                    println!("miss");
                    // There was nothing to hit, Let is draw the background.
                }
                else {
                    println!("hit");
                    // Draw to pixel here the colour of the object
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
