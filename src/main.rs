extern crate image as im;
extern crate imageproc as proc;
extern crate nalgebra as nl;

pub const WIDTH: u32 = 400;
pub const HEIGHT: u32 = 400;

// Camera Position
pub const CAM_X: f32 = 0.;
pub const CAM_Y: f32 = 0.;
pub const CAM_Z: f32 = 0.;
// Camera rotation
pub const CAM_T: f32 = 0.; // Theta ( Right Left plane )
pub const CAM_P: f32 = 0.; // Phi ( Up down plane)


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
    r: f32,
}

//fn ray_collision(ray: Ray, )


fn main() {
    println!("Hello, world!");
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            // Create Ray from position to screen
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
