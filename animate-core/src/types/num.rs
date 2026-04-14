use crate::Lerp;

impl Lerp for f64 {
    fn lerp(start: &f64, end: &f64, t: f64) -> f64 {
        start + (end - start) * t
    }
}

impl Lerp for f32 {
    fn lerp(start: &f32, end: &f32, t: f64) -> f32 {
        (*start as f64 + (*end as f64 - *start as f64) * t) as f32
    }
}

impl Lerp for usize {
    fn lerp(start: &usize, end: &usize, t: f64) -> usize {
        (*start as f64 + (*end as f64 - *start as f64) * t).round() as usize
    }
}

impl Lerp for isize {
    fn lerp(start: &isize, end: &isize, t: f64) -> isize {
        (*start as f64 + (*end as f64 - *start as f64) * t).round() as isize
    }
}

impl Lerp for u64 {
    fn lerp(start: &u64, end: &u64, t: f64) -> u64 {
        (*start as f64 + (*end as f64 - *start as f64) * t).round() as u64
    }
}

impl Lerp for i64 {
    fn lerp(start: &i64, end: &i64, t: f64) -> i64 {
        (*start as f64 + (*end as f64 - *start as f64) * t).round() as i64
    }
}

impl Lerp for u32 {
    fn lerp(start: &u32, end: &u32, t: f64) -> u32 {
        (*start as f64 + (*end as f64 - *start as f64) * t).round() as u32
    }
}

impl Lerp for i32 {
    fn lerp(start: &i32, end: &i32, t: f64) -> i32 {
        (*start as f64 + (*end as f64 - *start as f64) * t).round() as i32
    }
}

impl Lerp for u16 {
    fn lerp(start: &u16, end: &u16, t: f64) -> u16 {
        (*start as f64 + (*end as f64 - *start as f64) * t).round() as u16
    }
}

impl Lerp for i16 {
    fn lerp(start: &i16, end: &i16, t: f64) -> i16 {
        (*start as f64 + (*end as f64 - *start as f64) * t).round() as i16
    }
}

impl Lerp for u8 {
    fn lerp(start: &u8, end: &u8, t: f64) -> u8 {
        (*start as f64 + (*end as f64 - *start as f64) * t).round() as u8
    }
}

impl Lerp for i8 {
    fn lerp(start: &i8, end: &i8, t: f64) -> i8 {
        (*start as f64 + (*end as f64 - *start as f64) * t).round() as i8
    }
}
