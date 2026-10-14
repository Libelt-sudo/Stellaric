use std::collections::VecDeque;

use stellaric_core::physics::*;
use kiss3d::prelude::*;

const TRAIL_LEN: usize = 500;

pub struct VisBody {
    body: SceneNode3d,
    trail: VecDeque<Vec3>,
    color: Color,
}

impl VisBody {
    pub fn new(scene: &mut SceneNode3d, radius: f32, color: Color) -> Self {
        Self {
            body: scene.add_sphere(radius).set_color(color),
            trail: VecDeque::with_capacity(TRAIL_LEN),
            color,
        }
    }

    /// Sync the visual state to the sim body and record the trail.
    pub fn update(&mut self, sim_body: &Body) {
        let p = sim_body.position;
        let pos = Vec3::new(p.x as f32, p.y as f32, p.z as f32);
        self.body.set_position(pos);
        if self.trail.len() == TRAIL_LEN {
            self.trail.pop_front();
        }
        self.trail.push_back(pos);
    }

    pub fn draw_trail(&self, window: &mut Window) {
        let n = self.trail.len();
        for j in 1..n {
            let mut c = self.color;
            c.a = j as f32 / n as f32;
            window.draw_line(self.trail[j - 1], self.trail[j], c, 2.0, false);
        }
    }
}
