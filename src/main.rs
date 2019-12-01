extern crate image as im;
extern crate imageproc as proc;
extern crate nalgebra as nl;

pub const WIDTH: u32 = 400;
pub const HEIGHT: u32 = 400;

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

pub const RAY_BOUNCE_MAX: u8 = 5;


struct Ray {
    o: nl::Vector3<f32>,
    d: nl::Vector3<f32>,
    count: u8 // number of collisions the ray has gone through so far
}

struct Plane {
    // EQ:: Ax + By + Cz + D = 0
    a: f32,
    b: f32,
    c: f32,
    d: f32
}

struct Sphere {
    // EQ:: (x-x0)^2 + (y-y0)^2 + (z-z0)^2 - r^2 = 0
    x0: f32,
    y0: f32,
    z0: f32,
    r: f32
}

//fn ray_collision(ray: Ray, )

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
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            // Create Ray from position to screen
            // TODO:: Factor in Camera rotation; Rotation matrix must be applied to scr_pos vector..
            let scr_pos: nl::Vector3<f32> = nl::Vector3::new(i*SCR_X/WIDTH, j*SCR_Y/HEIGHT, SCR_Z) + cam_pos;
            let mut r = create_ray(cam_pos, scr_pos, RAY_BOUNCE_MAX);
            for obj in set_of_objects {
                // If ray intersects object
                    // Find normal of object at this point
                    // Find partial derivative of normal on each axis??
                    // define colour of point recursively by tracing again from here to wherever the
                    // ray goes next until it hits nothing or a light source
            }
        }
    }
}
