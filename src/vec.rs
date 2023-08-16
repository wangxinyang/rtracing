pub struct Vec3 {
    e: [f64; 3],
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn format_color(self) -> String {
        format!(
            "{} {} {}",
            (255.999 * self.e[0]) as u64,
            (255.999 * self.e[1]) as u64,
            (255.999 * self.e[2]) as u64
        )
    }
}
