use animate::{Alternate, Animate as _, Cycle, Lerp, Once, easing, tick};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_once_f64(c: &mut Criterion) {
    c.bench_function("once_f64", |b| {
        b.iter(|| {
            let mut anim = Once::new(0.0, 1000.0, easing::linear, f64::lerp);
            anim.set(100.0);

            for _ in 0..10_000 {
                tick(1);
                anim.update();
                black_box(anim.get());
            }
        })
    });
}

fn bench_alternate_f64(c: &mut Criterion) {
    c.bench_function("alternate_f64", |b| {
        b.iter(|| {
            let mut anim = Alternate::new(0.0, 1000.0, easing::linear, f64::lerp);
            anim.set(100.0);

            for _ in 0..10_000 {
                tick(1);
                anim.update();
                black_box(anim.get());
            }
        })
    });
}

fn bench_cycle_f64(c: &mut Criterion) {
    c.bench_function("cycle_f64", |b| {
        b.iter(|| {
            let mut anim = Cycle::new(0.0, 1000.0, easing::linear, f64::lerp);
            anim.set(100.0);

            for _ in 0..10_000 {
                tick(1);
                anim.update();
                black_box(anim.get());
            }
        })
    });
}

fn bench_many_fields(c: &mut Criterion) {
    c.bench_function("many_fields_100", |b| {
        b.iter(|| {
            let mut anims = (0..100)
                .map(|_| Once::new(0.0, 1000.0, easing::linear, f64::lerp))
                .collect::<Vec<_>>();

            for a in &mut anims {
                a.set(100.0);
            }

            for _ in 0..1000 {
                tick(1);
                for a in &mut anims {
                    a.update();
                    black_box(a.get());
                }
            }
        })
    });
}

criterion_group!(
    benches,
    bench_once_f64,
    bench_alternate_f64,
    bench_cycle_f64,
    bench_many_fields
);

criterion_main!(benches);
