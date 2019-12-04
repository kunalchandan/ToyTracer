extern crate image as im;
extern crate imageproc as proc;
extern crate nalgebra as nl;

use std::cmp::max;


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


#[derive(Copy, Clone)]
pub struct Ray {
    // EQ:: Origin + Direction*t
    // TODO:: manage distance travelled
    o: nl::Vector3<f32>,
    d: nl::Vector3<f32>,
    count: u8 // number of collisions the ray has gone through so far
}

impl Ray {
    pub fn eval(&self, t: f32) -> nl::Vector3<f32> {
        return self.o + (self.d * t);
    }
}

struct Plane {
    // EQ:: Ax + By + Cz + D = 0
    a: f32,
    b: f32,
    c: f32,
    d: f32
}

pub trait Traceable {
    fn trace(&self, r: Ray) -> Ray;
    fn normal(&self, r: Ray) -> Ray;
    fn intersect(&self, r: Ray) -> (f32, nl::Vector3<f32>); // Basically when and where the beam hit
}

impl Traceable for Plane {
    fn trace(&self, r: Ray) -> Ray {
        if self.normal(r).d.dot(&r.d) < (0.01 / WIDTH as f32) {
            // This has a solution, i.e. can be seen because it is not parallel
            // For direction vector, we need the magnitude of the normal that equals the magnitude
            // of the vector in parallel to the normal of the plane
            let new_d: nl::Vector3<f32> = r.d - (2.0 * r.d.dot(&(self.normal(r).d)) * self.normal(r).d.normalize());
            let (t, o) = self.intersect(r);
            if t > 0.0 {
                return Ray {
                    o,
                    d: new_d,
                    count: r.count-1
                };
            }
            else {
                // Intersection Behind Camera
                return r;
            }
        }
        else{
            // They are practically parallel
            return r;
        }
    }

    fn normal(&self, r: Ray) -> Ray {
        return Ray {
            o: self.intersect(r).1, // This is not needed, consider commenting out
            d: nl::Vector3::new(self.a, self.b, self.c),
            count: 0
        };
    }

    fn intersect(&self, r: Ray) -> (f32, nl::Vector3<f32>) {
        let t: f32 = (-self.d)/self.normal(r).d.dot(&(r.d));
        return (t, r.eval(t));
    }
}

struct Sphere {
    // EQ:: (x-x0)^2 + (y-y0)^2 + (z-z0)^2 - r^2 = 0
    x0: f32,
    y0: f32,
    z0: f32,
    r: f32
}

impl Traceable for Sphere {
    fn trace(&self, r: Ray) -> Ray {
        let (t, o) = self.intersect(r);
        if t > 0.0 {
            // Collision
            let new_d: nl::Vector3<f32> = r.d - (2.0 * r.d.dot(&(self.normal(r).d)) * self.normal(r).d.normalize());
            return Ray {
                o,
                d: new_d,
                count: r.count - 1
            }

        }
        return r;
    }

    fn normal(&self, r: Ray) -> Ray {
        return Ray {
            o: self.intersect(r).1, // This is not needed, consider commenting out
            d: self.intersect(r).1 - nl::Vector::new(self.x0, self.y0, self.z0),
            count: r.count - 1
        };
    }

    fn intersect(&self, r: Ray) -> (f32, nl::Vector3<f32>) {
        let c: f32 = (r.o[0] - self.x0).powi(2) +
                     (r.o[1] - self.y0).powi(2) +
                     (r.o[2] - self.z0).powi(2);
        let b: f32 = (2.0 * r.d[0] * (r.o[0] - self.x0)) +
                     (2.0 * r.d[1] * (r.o[1] - self.y0)) +
                     (2.0 * r.d[2] * (r.o[2] - self.z0));
        let a: f32 = r.d[0].powi(2) +
                     r.d[1].powi(2) +
                     r.d[2].powi(2);

        let t0: f32 = (-b + (b.powi(2) - (4.0 * a * c)).sqrt())/(2.0 * a);
        let t1: f32 = (-b - (b.powi(2) - (4.0 * a * c)).sqrt())/(2.0 * a);
        let t: f32 = max(t0, t1);
        return (t, r.eval(t))
    }
}

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
    world.push(p2);
    world.push(s1);
    world.push(s2);
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            // Create Ray from position to screen
            // TODO:: Factor in Camera rotation; Rotation matrix must be applied to scr_pos vector..
            let scr_pos: nl::Vector3<f32> = nl::Vector3::new((i as f32)*SCR_X/(WIDTH as f32),
                                                             (j as f32)*SCR_Y/(HEIGHT as f32),
                                                             SCR_Z) + cam_pos;
            let mut r = create_ray(cam_pos, scr_pos, RAY_BOUNCE_MAX);
            for obj in world.components.iter() {
                let r_new = obj.trace(r);
                if r_new == r {
                    // There was nothing to hit, Let is draw the backhround.
                }
                else {
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
