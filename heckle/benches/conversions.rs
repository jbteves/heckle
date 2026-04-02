use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use heckle::{ToBillyMaysMode, ToSpongebobCase};

// ---------------------------------------------------------------------------
// Shared inputs
// ---------------------------------------------------------------------------

const SHORT: &str = "hello, world!";
const MEDIUM: &str =
    "the quick brown fox jumps over the lazy dog, and then it does it again for good measure!!!";
const PUNCTUATION_HEAVY: &str =
    "w@a!i#t$,% ^t&h*e(r)e-'s= m[o]r{e}!!! b|u;y: n'o\"w,. o`n~ly $19.99!!!";

fn long_string() -> String {
    "the quick brown fox jumps over the lazy dog. ".repeat(222)
}

fn multiline_string() -> String {
    "hello, world!\nwait, there's more!!!\nbuy now for only $19.99!\n\nact fast!!!\n".repeat(50)
}

// ---------------------------------------------------------------------------
// heckle benchmarks
// ---------------------------------------------------------------------------

fn bench_spongebob(c: &mut Criterion) {
    let long = long_string();
    let multiline = multiline_string();

    let mut group = c.benchmark_group("spongebob_case");

    for (label, input) in [
        ("short", SHORT),
        ("medium", MEDIUM),
        ("punctuation_heavy", PUNCTUATION_HEAVY),
        ("long", long.as_str()),
        ("multiline", multiline.as_str()),
    ] {
        group.bench_with_input(BenchmarkId::new("input", label), input, |b, s| {
            b.iter(|| black_box(s).to_spongebob_case())
        });
    }

    group.finish();
}

fn bench_billy_mays(c: &mut Criterion) {
    let long = long_string();
    let multiline = multiline_string();

    let mut group = c.benchmark_group("billy_mays_mode");

    for (label, input) in [
        ("short", SHORT),
        ("medium", MEDIUM),
        ("punctuation_heavy", PUNCTUATION_HEAVY),
        ("long", long.as_str()),
        ("multiline", multiline.as_str()),
    ] {
        group.bench_with_input(BenchmarkId::new("input", label), input, |b, s| {
            b.iter(|| black_box(s).to_billy_mays_mode())
        });
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// heck benchmarks — snake_case, upper_camel_case, and shouty_snake_case as
// representative comparisons. These are different operations but share the
// same char-iteration / String-building profile, making them a fair baseline.
// ---------------------------------------------------------------------------

fn bench_heck_snake(c: &mut Criterion) {
    let long = long_string();
    let multiline = multiline_string();

    let mut group = c.benchmark_group("heck_snake_case");

    for (label, input) in [
        ("short", SHORT),
        ("medium", MEDIUM),
        ("punctuation_heavy", PUNCTUATION_HEAVY),
        ("long", long.as_str()),
        ("multiline", multiline.as_str()),
    ] {
        group.bench_with_input(BenchmarkId::new("input", label), input, |b, s| {
            b.iter(|| black_box(s).to_snake_case())
        });
    }

    group.finish();
}

fn bench_heck_upper_camel(c: &mut Criterion) {
    let long = long_string();
    let multiline = multiline_string();

    let mut group = c.benchmark_group("heck_upper_camel_case");

    for (label, input) in [
        ("short", SHORT),
        ("medium", MEDIUM),
        ("punctuation_heavy", PUNCTUATION_HEAVY),
        ("long", long.as_str()),
        ("multiline", multiline.as_str()),
    ] {
        group.bench_with_input(BenchmarkId::new("input", label), input, |b, s| {
            b.iter(|| black_box(s).to_upper_camel_case())
        });
    }

    group.finish();
}

fn bench_heck_shouty_snake(c: &mut Criterion) {
    let long = long_string();
    let multiline = multiline_string();

    let mut group = c.benchmark_group("heck_shouty_snake_case");

    for (label, input) in [
        ("short", SHORT),
        ("medium", MEDIUM),
        ("punctuation_heavy", PUNCTUATION_HEAVY),
        ("long", long.as_str()),
        ("multiline", multiline.as_str()),
    ] {
        group.bench_with_input(BenchmarkId::new("input", label), input, |b, s| {
            b.iter(|| black_box(s).to_shouty_snake_case())
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_spongebob,
    bench_billy_mays,
    bench_heck_snake,
    bench_heck_upper_camel,
    bench_heck_shouty_snake,
);
criterion_main!(benches);
