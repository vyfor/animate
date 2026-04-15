use animate::{Alternate, Cycle, Lerp, Once, easing, tick};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_once_f64(c: &mut Criterion) {
    c.bench_function("once_f64", |b| {
        b.iter(|| {
            let mut anim = Once::new(0.0, 1000.0, easing::linear, f64::lerp);
            anim.set(100.0);

            for _ in 0..10_000 {
                tick(1);
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
                black_box(anim.get());
            }
        })
    });
}

fn bench_many_fields(c: &mut Criterion) {
    c.bench_function("many_fields_100", |b| {
        b.iter(|| {
            let mut anims: Vec<Once<f64>> = (0..100)
                .map(|_| Once::new(0.0, 1000.0, easing::linear, f64::lerp))
                .collect();

            for a in &mut anims {
                a.set(100.0);
            }

            for _ in 0..1000 {
                tick(1);
                for a in &anims {
                    black_box(a.get());
                }
            }
        })
    });
}

fn bench_get_only(c: &mut Criterion) {
    c.bench_function("get_only", |b| {
        let mut anim = Once::new(0.0, 1000.0, easing::linear, f64::lerp);
        anim.set(100.0);

        b.iter(|| {
            for _ in 0..50_000 {
                tick(1);
                black_box(anim.get());
            }
        })
    });
}

criterion_group!(
    benches,
    bench_once_f64,
    bench_alternate_f64,
    bench_cycle_f64,
    bench_many_fields,
    bench_get_only
);
criterion_main!(benches);
