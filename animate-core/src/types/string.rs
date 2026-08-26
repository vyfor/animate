use crate::interpolate::Interpolate;

impl Interpolate for String {
    fn lerp(start: &String, end: &String, t: f32) -> String {
        if t <= 0.0 {
            return start.clone();
        }
        if t >= 1.0 {
            return end.clone();
        }

        let start_chars: Vec<char> = start.chars().collect();
        let end_chars: Vec<char> = end.chars().collect();

        let shared = start_chars
            .iter()
            .zip(end_chars.iter())
            .take_while(|(a, b)| a == b)
            .count();

        let erase_len = start_chars.len() - shared;
        let reveal_len = end_chars.len() - shared;
        let total = (erase_len + reveal_len) as f32;

        let mut result: String = end_chars[..shared].iter().collect();

        if total == 0.0 {
            return result;
        }

        let progress = total * t;

        if progress < erase_len as f32 {
            let remaining = erase_len - progress.round() as usize;
            result.extend(start_chars[shared..shared + remaining].iter());
        } else {
            let revealed = (progress - erase_len as f32).round() as usize;
            result.extend(end_chars[shared..shared + revealed.min(reveal_len)].iter());
        }

        result
    }
}