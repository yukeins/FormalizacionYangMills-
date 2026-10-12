fn check_triangular(a: f64, b: f64, c: f64) -> bool {
    (a + b - c) >= 0.0 && (a - b + c) >= 0.0 && (-a + b + c) >= 0.0 && ((a + b + c).fract() < 1e-9)
}
fn main() {
    println!("{}", check_triangular(0.5, 0.5, 0.5));
    println!("{}", check_triangular(1.0, 1.0, 1.0));
}
