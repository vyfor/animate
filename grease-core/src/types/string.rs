use crate::Lerp;

impl Lerp for String {
    fn lerp(start: &String, end: &String, t: f64) -> String {
        if t <= 0.0 {
            return start.clone();
        }
        if t >= 1.0 {
            return end.clone();
        }

        let chars = end.chars().count();
        let reveal = (chars as f64 * t).round() as usize;

        end.chars().take(reveal).collect()
    }
}
