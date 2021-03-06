use crate::world::Traceable;

pub const RAY_BOUNCE_MAX: u8 = 5;


#[derive(Copy, Clone)]
pub struct Ray {
    // EQ:: Origin + Direction*t
    // TODO:: manage distance travelled
    pub o: nl::Vector3<f32>,
    pub d: nl::Vector3<f32>,
    pub count: u8, // number of collisions the ray has gone through so far
    pub distances: [f32; RAY_BOUNCE_MAX as usize],
//    pub collided: [&Box<dyn Traceable>; RAY_BOUNCE_MAX as usize],
}

impl Ray {
    pub fn eval(&self, t: f32) -> nl::Vector3<f32> {
        return self.o + (self.d * t);
    }

    pub fn total_distance(&self) -> f32 {
        return self.distances.iter().sum();
    }
}