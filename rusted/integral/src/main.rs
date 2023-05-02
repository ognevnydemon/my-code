// oint algorithms (Rust)

use meval::{eval_str_with_context, Context};
use scan_fmt::scan_fmt;
use std::io::stdin;

fn f(x: f64, expr: &str) -> f64 {
    let mut context = Context::new();
    context.var("x", x);
    eval_str_with_context(expr, &context).unwrap()
}

fn lrect(a: f64, b: f64, n: u64, expr: &str) -> f64 {
    let mut s: f64 = 0.0;
    for i in 0..=n - 1 {
        s += f(a + i as f64 * (b - a) / (n as f64), expr)
    }
    s * ((b - a) / n as f64)
}

fn rrect(a: f64, b: f64, n: u64, expr: &str) -> f64 {
    let mut s: f64 = 0.0;
    for i in 1..=n {
        s += f(a + i as f64 * (b - a) / (n as f64), expr)
    }
    s * ((b - a) / n as f64)
}

fn mrect(a: f64, b: f64, n: u64, expr: &str) -> f64 {
    let mut s: f64 = 0.0;
    for i in 0..=n - 1 {
        s += f(
            a + (b - a) * (2.0 * i as f64 + 1.0) / (2.0 * n as f64),
            expr,
        )
    }
    s * ((b - a) / n as f64)
}

fn trapezoid(a: f64, b: f64, n: u64, expr: &str) -> f64 {
    let mut s: f64 = 0.0;
    for i in 0..=n - 1 {
        s += (f(a + i as f64 * (b - a) / n as f64, expr)
            + f(a + (i + 1) as f64 * (b - a) / n as f64, expr))
            / 2.0
    }
    s * ((b - a) / n as f64)
}

fn main() {
    let (mut seg, mut eps, mut expr) = (String::new(), String::new(), String::new());

    stdin().read_line(&mut seg).unwrap();
    stdin().read_line(&mut eps).unwrap();
    stdin().read_line(&mut expr).unwrap();

    let input = format!("{seg} {eps}");

    let (a, b, eps) = scan_fmt!(&input, "{} {} {}", f64, f64, f64).unwrap();

    let (mut n, mut s1, mut s2): (u64, f64, f64) = (1, 0.0, f(a, &expr) * (b - a));

    while (s2 - s1).abs() > eps {
        n *= 2;
        (s1, s2) = (s2, lrect(a, b, n, &expr));
    }

    println!("\nl = {s2}");

    (s1, s2) = (0.0, f(a, &expr) * (b - a));

    while (s2 - s1).abs() > eps {
        n *= 2;
        (s1, s2) = (s2, rrect(a, b, n, &expr));
    }

    println!("r = {s2}");

    (s1, s2) = (0.0, f((a + b) / 2.0, &expr) * (b - a));

    while (s2 - s1).abs() > eps {
        n *= 2;
        (s1, s2) = (s2, mrect(a, b, n, &expr));
    }

    println!("m = {s2}");

    (s1, s2) = (0.0, (b - a) * (f(a, &expr) + f(b, &expr)) / 2.0);

    while (s2 - s1).abs() > eps {
        n *= 2;
        (s1, s2) = (s2, trapezoid(a, b, n, &expr));
    }

    print!("t = {s2}");
}
