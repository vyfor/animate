#![cfg(feature = "ratatui")]

use ratatui::layout::{Constraint, Margin, Rect};
use ratatui::widgets::Padding;

use crate::types::num::spring_step;
use crate::{SpringAnim, SpringParams, TweenAnim};

impl TweenAnim for Rect {
    fn tween(start: &Rect, end: &Rect, t: f64) -> Rect {
        Rect {
            x: u16::tween(&start.x, &end.x, t),
            y: u16::tween(&start.y, &end.y, t),
            width: u16::tween(&start.width, &end.width, t),
            height: u16::tween(&start.height, &end.height, t),
        }
    }
}

impl SpringAnim for Rect {
    type Velocity = [f64; 4];

    fn spring(
        current: &Rect,
        target: &Rect,
        velocity: &[f64; 4],
        params: SpringParams,
        dt: f64,
    ) -> (Rect, [f64; 4]) {
        let [vx, vy, vw, vh] = *velocity;

        let (nx, nvx) = spring_step(current.x as f64, target.x as f64, vx, params, dt);
        let (ny, nvy) = spring_step(current.y as f64, target.y as f64, vy, params, dt);
        let (nw, nvw) = spring_step(current.width as f64, target.width as f64, vw, params, dt);
        let (nh, nvh) = spring_step(current.height as f64, target.height as f64, vh, params, dt);

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

impl TweenAnim for Margin {
    fn tween(start: &Margin, end: &Margin, t: f64) -> Margin {
        Margin {
            vertical: u16::tween(&start.vertical, &end.vertical, t),
            horizontal: u16::tween(&start.horizontal, &end.horizontal, t),
        }
    }
}

impl SpringAnim for Margin {
    type Velocity = [f64; 2];

    fn spring(
        current: &Margin,
        target: &Margin,
        velocity: &[f64; 2],
        params: SpringParams,
        dt: f64,
    ) -> (Margin, [f64; 2]) {
        let [vv, vh] = *velocity;

        let (nv, nvv) = spring_step(current.vertical as f64, target.vertical as f64, vv, params, dt);
        let (nh, nvh) =
            spring_step(current.horizontal as f64, target.horizontal as f64, vh, params, dt);

        (
            Margin {
                vertical: nv.round() as u16,
                horizontal: nh.round() as u16,
            },
            [nvv, nvh],
        )
    }
}

impl TweenAnim for Padding {
    fn tween(start: &Padding, end: &Padding, t: f64) -> Padding {
        Padding {
            left: u16::tween(&start.left, &end.left, t),
            right: u16::tween(&start.right, &end.right, t),
            top: u16::tween(&start.top, &end.top, t),
            bottom: u16::tween(&start.bottom, &end.bottom, t),
        }
    }
}

impl SpringAnim for Padding {
    type Velocity = [f64; 4];

    fn spring(
        current: &Padding,
        target: &Padding,
        velocity: &[f64; 4],
        params: SpringParams,
        dt: f64,
    ) -> (Padding, [f64; 4]) {
        let [vl, vr, vt, vb] = *velocity;

        let (nl, nvl) = spring_step(current.left as f64, target.left as f64, vl, params, dt);
        let (nr, nvr) = spring_step(current.right as f64, target.right as f64, vr, params, dt);
        let (nt, nvt) = spring_step(current.top as f64, target.top as f64, vt, params, dt);
        let (nb, nvb) = spring_step(current.bottom as f64, target.bottom as f64, vb, params, dt);

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

impl TweenAnim for Constraint {
    fn tween(start: &Constraint, end: &Constraint, t: f64) -> Constraint {
        match (start, end) {
            (Constraint::Percentage(s), Constraint::Percentage(e)) => {
                Constraint::Percentage(u16::tween(s, e, t))
            }
            (Constraint::Ratio(sn, sd), Constraint::Ratio(en, ed)) => {
                let n = u32::tween(sn, en, t);
                let d = u32::tween(sd, ed, t);
                Constraint::Ratio(n, d)
            }
            (Constraint::Length(s), Constraint::Length(e)) => {
                Constraint::Length(u16::tween(s, e, t))
            }
            (Constraint::Max(s), Constraint::Max(e)) => Constraint::Max(u16::tween(s, e, t)),
            (Constraint::Min(s), Constraint::Min(e)) => Constraint::Min(u16::tween(s, e, t)),
            (Constraint::Fill(s), Constraint::Fill(e)) => Constraint::Fill(u16::tween(s, e, t)),
            _ => *end,
        }
    }
}

impl SpringAnim for Constraint {
    type Velocity = [f64; 2];

    fn spring(
        current: &Constraint,
        target: &Constraint,
        velocity: &[f64; 2],
        params: SpringParams,
        dt: f64,
    ) -> (Constraint, [f64; 2]) {
        match (current, target) {
            (Constraint::Percentage(s), Constraint::Percentage(e)) => {
                let (v, vel) = spring_step(*s as f64, *e as f64, velocity[0], params, dt);
                (Constraint::Percentage(v.round() as u16), [vel, 0.0])
            }
            (Constraint::Ratio(sn, sd), Constraint::Ratio(en, ed)) => {
                let (n, vn) = spring_step(*sn as f64, *en as f64, velocity[0], params, dt);
                let (d, vd) = spring_step(*sd as f64, *ed as f64, velocity[1], params, dt);
                (
                    Constraint::Ratio(n.round() as u32, d.round() as u32),
                    [vn, vd],
                )
            }
            (Constraint::Length(s), Constraint::Length(e)) => {
                let (v, vel) = spring_step(*s as f64, *e as f64, velocity[0], params, dt);
                (Constraint::Length(v.round() as u16), [vel, 0.0])
            }
            (Constraint::Max(s), Constraint::Max(e)) => {
                let (v, vel) = spring_step(*s as f64, *e as f64, velocity[0], params, dt);
                (Constraint::Max(v.round() as u16), [vel, 0.0])
            }
            (Constraint::Min(s), Constraint::Min(e)) => {
                let (v, vel) = spring_step(*s as f64, *e as f64, velocity[0], params, dt);
                (Constraint::Min(v.round() as u16), [vel, 0.0])
            }
            (Constraint::Fill(s), Constraint::Fill(e)) => {
                let (v, vel) = spring_step(*s as f64, *e as f64, velocity[0], params, dt);
                (Constraint::Fill(v.round() as u16), [vel, 0.0])
            }
            _ => (*target, [0.0, 0.0]),
        }
    }
}


