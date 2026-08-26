use animate::{Clock, Repeat, Tween, easing};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::time::Duration;

fn bench_once_f32(c: &mut Criterion) {
    c.bench_function("once_f32", |b| {
        b.iter(|| {
            let mut clock = Clock::new();
            let mut anim = Tween::new(0.0f32)
                .duration(Duration::from_secs(10))
                .easing(easing::linear);
            anim.to(100.0);

            for _ in 0..10_000 {
                let time = clock.advance(Duration::from_millis(1));
                black_box(anim.advance(time));
            }
        })
    });
}

fn bench_alternate_f32(c: &mut Criterion) {
    c.bench_function("alternate_f32", |b| {
        b.iter(|| {
            let mut clock = Clock::new();
            let mut anim = Tween::new(0.0f32)
                .duration(Duration::from_secs(10))
                .easing(easing::linear)
                .alternate(true)
                .repeat(Repeat::Infinite);
            anim.to(100.0);

            for _ in 0..10_000 {
                let time = clock.advance(Duration::from_millis(1));
                black_box(anim.advance(time));
            }
        })
    });
}

fn bench_cycle_f32(c: &mut Criterion) {
    c.bench_function("cycle_f32", |b| {
        b.iter(|| {
            let mut clock = Clock::new();
            let mut anim = Tween::new(0.0f32)
                .duration(Duration::from_secs(10))
                .easing(easing::linear)
                .repeat(Repeat::Infinite);
            anim.to(100.0);

            for _ in 0..10_000 {
                let time = clock.advance(Duration::from_millis(1));
                black_box(anim.advance(time));
            }
        })
    });
}

fn bench_many_fields(c: &mut Criterion) {
    c.bench_function("many_fields_100", |b| {
        b.iter(|| {
            let mut clock = Clock::new();
            let mut anims = (0..100)
                .map(|_| Tween::new(0.0f32).duration(Duration::from_secs(10)))
                .collect::<Vec<_>>();

            for a in &mut anims {
                a.to(100.0);
            }

            for _ in 0..1_000 {
                let time = clock.advance(Duration::from_millis(1));
                for a in &mut anims {
                    black_box(a.advance(time));
                }
            }
        })
    });
}

criterion_group!(
    benches,
    bench_once_f32,
    bench_alternate_f32,
    bench_cycle_f32,
    bench_many_fields
);

criterion_main!(benches);
