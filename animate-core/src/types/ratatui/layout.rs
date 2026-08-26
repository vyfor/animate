#![cfg(feature = "ratatui")]

use crate::interpolate::Interpolate;
use crate::spring::{Distance, Integrate, SpringParams};
use ratatui::layout::{Constraint, Margin, Rect};
use ratatui::widgets::Padding;

impl Interpolate for Rect {
    fn lerp(start: &Rect, end: &Rect, t: f32) -> Rect {
        Rect {
            x: u16::lerp(&start.x, &end.x, t),
            y: u16::lerp(&start.y, &end.y, t),
            width: u16::lerp(&start.width, &end.width, t),
            height: u16::lerp(&start.height, &end.height, t),
        }
    }
}

impl Integrate for Rect {
    type Velocity = [f32; 4];

    fn integrate(&self, target: &Rect, velocity: &[f32; 4], params: SpringParams, dt: f32) -> (Rect, [f32; 4]) {
        let [vx, vy, vw, vh] = *velocity;

        let (nx, nvx) = params.step(self.x as f32, target.x as f32, vx, dt);
        let (ny, nvy) = params.step(self.y as f32, target.y as f32, vy, dt);
        let (nw, nvw) = params.step(self.width as f32, target.width as f32, vw, dt);
        let (nh, nvh) = params.step(self.height as f32, target.height as f32, vh, dt);

        (
            Rect {
                x: nx.round() as u16,
                y: ny.round() as u16,
                width: nw.round() as u16,
                height: nh.round() as u16,
            },
            [nvx, nvy, nvw, nvh],
        )
    }
}

impl Distance for Rect {
    #[inline]
    fn distance(&self, other: &Rect) -> f32 {
        let dx = self.x as f32 - other.x as f32;
        let dy = self.y as f32 - other.y as f32;
        let dw = self.width as f32 - other.width as f32;
        let dh = self.height as f32 - other.height as f32;
        (dx * dx + dy * dy + dw * dw + dh * dh).sqrt()
    }
}

impl Interpolate for Margin {
    fn lerp(start: &Margin, end: &Margin, t: f32) -> Margin {
        Margin {
            vertical: u16::lerp(&start.vertical, &end.vertical, t),
            horizontal: u16::lerp(&start.horizontal, &end.horizontal, t),
        }
    }
}

impl Integrate for Margin {
    type Velocity = [f32; 2];

    fn integrate(&self, target: &Margin, velocity: &[f32; 2], params: SpringParams, dt: f32) -> (Margin, [f32; 2]) {
        let [vv, vh] = *velocity;

        let (nv, nvv) = params.step(self.vertical as f32, target.vertical as f32, vv, dt);
        let (nh, nvh) =
            params.step(self.horizontal as f32, target.horizontal as f32, vh, dt);

        (
            Margin {
                vertical: nv.round() as u16,
                horizontal: nh.round() as u16,
            },
            [nvv, nvh],
        )
    }
}

impl Distance for Margin {
    #[inline]
    fn distance(&self, other: &Margin) -> f32 {
        let dv = self.vertical as f32 - other.vertical as f32;
        let dh = self.horizontal as f32 - other.horizontal as f32;
        (dv * dv + dh * dh).sqrt()
    }
}

impl Interpolate for Padding {
    fn lerp(start: &Padding, end: &Padding, t: f32) -> Padding {
        Padding {
            left: u16::lerp(&start.left, &end.left, t),
            right: u16::lerp(&start.right, &end.right, t),
            top: u16::lerp(&start.top, &end.top, t),
            bottom: u16::lerp(&start.bottom, &end.bottom, t),
        }
    }
}

impl Integrate for Padding {
    type Velocity = [f32; 4];

    fn integrate(&self, target: &Padding, velocity: &[f32; 4], params: SpringParams, dt: f32) -> (Padding, [f32; 4]) {
        let [vl, vr, vt, vb] = *velocity;

        let (nl, nvl) = params.step(self.left as f32, target.left as f32, vl, dt);
        let (nr, nvr) = params.step(self.right as f32, target.right as f32, vr, dt);
        let (nt, nvt) = params.step(self.top as f32, target.top as f32, vt, dt);
        let (nb, nvb) = params.step(self.bottom as f32, target.bottom as f32, vb, dt);

        (
            Padding {
                left: nl.round() as u16,
                right: nr.round() as u16,
                top: nt.round() as u16,
                bottom: nb.round() as u16,
            },
            [nvl, nvr, nvt, nvb],
        )
    }
}

impl Distance for Padding {
    #[inline]
    fn distance(&self, other: &Padding) -> f32 {
        let dl = self.left as f32 - other.left as f32;
        let dr = self.right as f32 - other.right as f32;
        let dt_ = self.top as f32 - other.top as f32;
        let db = self.bottom as f32 - other.bottom as f32;
        (dl * dl + dr * dr + dt_ * dt_ + db * db).sqrt()
    }
}

impl Distance for Constraint {
    #[inline]
    fn distance(&self, other: &Constraint) -> f32 {
        fn scalar(a: u16, b: u16) -> f32 {
            a as f32 - b as f32
        }
        match (self, other) {
            (Constraint::Percentage(s), Constraint::Percentage(e)) => scalar(*s, *e).abs(),
            (Constraint::Length(s), Constraint::Length(e)) => scalar(*s, *e).abs(),
            (Constraint::Max(s), Constraint::Max(e)) => scalar(*s, *e).abs(),
            (Constraint::Min(s), Constraint::Min(e)) => scalar(*s, *e).abs(),
            (Constraint::Fill(s), Constraint::Fill(e)) => scalar(*s, *e).abs(),
            _ => 0.0,
        }
    }
}

impl Interpolate for Constraint {
    fn lerp(start: &Constraint, end: &Constraint, t: f32) -> Constraint {
        match (start, end) {
            (Constraint::Percentage(s), Constraint::Percentage(e)) => {
                Constraint::Percentage(u16::lerp(s, e, t))
            }
            (Constraint::Ratio(sn, sd), Constraint::Ratio(en, ed)) => {
                let n = u32::lerp(sn, en, t);
                let d = u32::lerp(sd, ed, t);
                Constraint::Ratio(n, d)
            }
            (Constraint::Length(s), Constraint::Length(e)) => {
                Constraint::Length(u16::lerp(s, e, t))
            }
            (Constraint::Max(s), Constraint::Max(e)) => Constraint::Max(u16::lerp(s, e, t)),
            (Constraint::Min(s), Constraint::Min(e)) => Constraint::Min(u16::lerp(s, e, t)),
            (Constraint::Fill(s), Constraint::Fill(e)) => Constraint::Fill(u16::lerp(s, e, t)),
            _ => *end,
        }
    }
}

impl Integrate for Constraint {
    type Velocity = [f32; 2];

    fn integrate(&self, target: &Constraint, velocity: &[f32; 2], params: SpringParams, dt: f32) -> (Constraint, [f32; 2]) {
        match (self, target) {
            (Constraint::Percentage(s), Constraint::Percentage(e)) => {
                let (v, vel) = params.step(*s as f32, *e as f32, velocity[0], dt);
                (Constraint::Percentage(v.round() as u16), [vel, 0.0])
            }
            (Constraint::Ratio(sn, sd), Constraint::Ratio(en, ed)) => {
                let (n, vn) = params.step(*sn as f32, *en as f32, velocity[0], dt);
                let (d, vd) = params.step(*sd as f32, *ed as f32, velocity[1], dt);
                (
                    Constraint::Ratio(n.round() as u32, d.round() as u32),
                    [vn, vd],
                )
            }
            (Constraint::Length(s), Constraint::Length(e)) => {
                let (v, vel) = params.step(*s as f32, *e as f32, velocity[0], dt);
                (Constraint::Length(v.round() as u16), [vel, 0.0])
            }
            (Constraint::Max(s), Constraint::Max(e)) => {
                let (v, vel) = params.step(*s as f32, *e as f32, velocity[0], dt);
                (Constraint::Max(v.round() as u16), [vel, 0.0])
            }
            (Constraint::Min(s), Constraint::Min(e)) => {
                let (v, vel) = params.step(*s as f32, *e as f32, velocity[0], dt);
                (Constraint::Min(v.round() as u16), [vel, 0.0])
            }
            (Constraint::Fill(s), Constraint::Fill(e)) => {
                let (v, vel) = params.step(*s as f32, *e as f32, velocity[0], dt);
                (Constraint::Fill(v.round() as u16), [vel, 0.0])
            }
            _ => (*target, [0.0, 0.0]),
        }
    }
}
