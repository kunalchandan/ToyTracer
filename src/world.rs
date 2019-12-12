use crate::ray::*;
use crate::display::*;

pub struct Plane {
    // EQ:: Ax + By + Cz + D = 0
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32
}

pub trait Traceable {
    fn trace(&self, r: Ray) -> Ray;
    fn normal(&self, r: Ray) -> Ray;
    fn intersect(&self, r: Ray) -> (f32, nl::Vector3<f32>); // Basically when and where the beam hit
}

impl Traceable for Plane {
    fn trace(&self, r: Ray) -> Ray {
        if self.normal(r).d.dot(&r.d) < (0.01 / (WIDTH + HEIGHT) as f32) {
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

pub struct Sphere {
    // EQ:: (x-x0)^2 + (y-y0)^2 + (z-z0)^2 - r^2 = 0
    pub x0: f32,
    pub y0: f32,
    pub z0: f32,
    pub r: f32
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
            d: self.intersect(r).1 - nl::Vector3::new(self.x0, self.y0, self.z0),
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
        let t: f32 = t0.max(t1);
        return (t, r.eval(t))
    }
}
