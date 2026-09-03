#![cfg(feature = "ratatui")]

use crate::interpolate::Interpolate;
use crate::spring::{Distance, Integrate, SpringSpec};
use ratatui::style::{Color, Style};

impl Interpolate for Style {
    fn lerp(start: &Style, end: &Style, t: f32) -> Style {
        if t <= 0.0 {
            return *start;
        }
        if t >= 1.0 {
            return *end;
        }

        let fg = match (start.fg, end.fg) {
            (Some(s), Some(e)) => Some(Color::lerp(&s, &e, t)),
            _ => start.fg,
        };

        let bg = match (start.bg, end.bg) {
            (Some(s), Some(e)) => Some(Color::lerp(&s, &e, t)),
            _ => start.bg,
        };

        let underline_color = match (start.underline_color, end.underline_color) {
            (Some(s), Some(e)) => Some(Color::lerp(&s, &e, t)),
            _ => start.underline_color,
        };

        Style {
            fg,
            bg,
            underline_color,
            add_modifier: start.add_modifier,
            sub_modifier: start.sub_modifier,
        }
    }
}

impl Integrate for Style {
    type Velocity = [f32; 6];

    fn integrate(
        &self,
        target: &Style,
        velocity: &[f32; 6],
        params: SpringSpec,
        dt: f32,
    ) -> (Style, [f32; 6]) {
        let fg_vel = [velocity[0], velocity[1], velocity[2]];
        let bg_vel = [velocity[3], velocity[4], velocity[5]];

        let (fg, [nvr, nvg, nvb]) = match (self.fg, target.fg) {
            (Some(c), Some(t)) => {
                let (next, next_vel) = c.integrate(&t, &fg_vel, params, dt);
                (Some(next), next_vel)
            }
            _ => (target.fg, [0.0; 3]),
        };

        let (bg, [nbr, nbg, nbb]) = match (self.bg, target.bg) {
            (Some(c), Some(t)) => {
                let (next, next_vel) = c.integrate(&t, &bg_vel, params, dt);
                (Some(next), next_vel)
            }
            _ => (target.bg, [0.0; 3]),
        };

        (
            Style {
                fg,
                bg,
                underline_color: self.underline_color,
                add_modifier: self.add_modifier,
                sub_modifier: self.sub_modifier,
            },
            [nvr, nvg, nvb, nbr, nbg, nbb],
        )
    }
}

impl Distance for Style {
    #[inline]
    fn distance(&self, other: &Style) -> f32 {
        let d_fg = match (self.fg, other.fg) {
            (Some(a), Some(b)) => a.distance(&b),
            _ => 0.0,
        };
        let d_bg = match (self.bg, other.bg) {
            (Some(a), Some(b)) => a.distance(&b),
            _ => 0.0,
        };
        (d_fg * d_fg + d_bg * d_bg).sqrt()
    }
}
