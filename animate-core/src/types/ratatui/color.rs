#![cfg(feature = "ratatui")]

use crate::interpolate::Interpolate;
use crate::spring::{Distance, Integrate, SpringSpec};
use ratatui::style::Color;

impl Interpolate for Color {
    fn lerp(start: &Color, end: &Color, t: f32) -> Color {
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

impl Integrate for Color {
    type Velocity = [f32; 3];

    fn integrate(
        &self,
        target: &Color,
        velocity: &[f32; 3],
        params: SpringSpec,
        dt: f32,
    ) -> (Color, [f32; 3]) {
        let (cr, cg, cb) = rgb(*self).unwrap_or((0, 0, 0));
        let (tr, tg, tb) = rgb(*target).unwrap_or((0, 0, 0));
        let [vr, vg, vb] = *velocity;

        let (nr, nvr) = params.step(cr as f32, tr as f32, vr, dt);
        let (ng, nvg) = params.step(cg as f32, tg as f32, vg, dt);
        let (nb, nvb) = params.step(cb as f32, tb as f32, vb, dt);

        let clamp_u8 = |v: f32| v.clamp(0.0, 255.0).round() as u8;
        (
            Color::Rgb(clamp_u8(nr), clamp_u8(ng), clamp_u8(nb)),
            [nvr, nvg, nvb],
        )
    }
}

impl Distance for Color {
    #[inline]
    fn distance(&self, other: &Color) -> f32 {
        match (rgb(*self), rgb(*other)) {
            (Some((r1, g1, b1)), Some((r2, g2, b2))) => {
                let dr = r1 as f32 - r2 as f32;
                let dg = g1 as f32 - g2 as f32;
                let db = b1 as f32 - b2 as f32;
                (dr * dr + dg * dg + db * db).sqrt()
            }
            _ => 0.0,
        }
    }
}

fn rgb(c: Color) -> Option<(u8, u8, u8)> {
    match c {
        Color::Rgb(r, g, b) => Some((r, g, b)),
        _ => None,
    }
}
