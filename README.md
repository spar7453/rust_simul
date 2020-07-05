# Install
Add following line inside Cargo.toml
```
[dependencies]
rust_simul = { git = "https://github.com/spar7453/rust_simul" }
```

# Depedencies
- Matrix and Vector calculation requires *[peroxide](https://github.com/Axect/Peroxide)*
- Random number generator can be used from other libraries

# Example
```rust
use peroxide::fuga::*;
use rust_simul::{FloatSimul, MonteCarlo};

struct Point {
    x: f64,
    y: f64
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
struct Circle;

impl Circle {
    fn is_inside_unit_circle(p: Point) -> bool {
        let length = p.x.powi(2) + p.y.powi(2);
        length <= 1.0
    }
}

fn inside_circle(random: &TPDist<f64>) -> f64 {
    let coord = random.sample(2).mul_scalar(1.0);
    let p = Point::new(coord[0], coord[1]);
    match Circle::is_inside_unit_circle(p) {
        true => 1.0,
        false => 0.0
    }
}

fn main() {
    let uniform:TPDist<f64> = Uniform(0.0,1.0);
    let pi_simul: Option<f64> = FloatSimul::simulate(&inside_circle, &uniform, 100000).map(|x| x * 4.0);
    println!("pi simulate = {:?}", pi_simul);
}
```
