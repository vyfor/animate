#[derive(Clone, Copy, Debug)]
pub struct SpringParams {
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
    pub epsilon: f32,
}

impl Default for SpringParams {
    fn default() -> Self {
        Self {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            epsilon: 0.001,
        }
    }
}

impl SpringParams {
    pub const fn new(stiffness: f32, damping: f32, mass: f32) -> Self {
        Self {
            stiffness,
            damping,
            mass,
            epsilon: 0.001,
        }
    }

    #[inline]
    pub(crate) fn step(&self, pos: f32, target: f32, vel: f32, dt: f32) -> (f32, f32) {
        let displacement = pos - target;
        let accel = (-self.stiffness * displacement - self.damping * vel) / self.mass;
        let new_vel = vel + accel * dt;
        let new_pos = pos + new_vel * dt;
        
        (new_pos, new_vel)
    }
}
