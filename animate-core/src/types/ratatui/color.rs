use crate::Lerp;
use ratatui::style::Color;

impl Lerp for Color {
    fn lerp(start: &Color, end: &Color, t: f64) -> Color {
        match (rgb(*start), rgb(*end)) {
            (Some((sr, sg, sb)), Some((er, eg, eb))) => {
                let r = u8::lerp(&sr, &er, t);
                let g = u8::lerp(&sg, &eg, t);
                let b = u8::lerp(&sb, &eb, t);
                Color::Rgb(r, g, b)
            }
            _ => *end,
        }
    }
}

fn rgb(c: Color) -> Option<(u8, u8, u8)> {
    match c {
        Color::Rgb(r, g, b) => Some((r, g, b)),
        _ => todo!(),
    }
}
