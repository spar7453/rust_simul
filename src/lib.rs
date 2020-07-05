use peroxide::fuga::{Matrix, Vector};
struct FloatSimul;
struct VectorSimul;
struct MatrixSimul;

trait MonteCarlo {
    type RHS;
    fn sample<'a, F, T>(f: &'a F, param: &'a T) -> Self::RHS
    where
        F: Fn(&'a T) -> Self::RHS {
        f(param)
    }

    fn simulate<'a, F, T>(f: &'a F, param: &'a T, n: usize) -> Option<Self::RHS>
    where
        F: Fn(&T) -> Self::RHS;
}

impl MonteCarlo for MatrixSimul {
    type RHS = Matrix;
    fn sample<'a, F, T>(f: &'a F, param: &'a T) -> Matrix
    where
        F: Fn(&'a T) -> Matrix {
        f(param)
    }

    fn simulate<'a, F, T>(f: &'a F, param: &'a T, n: usize) -> Option<Matrix>
    where
        F: Fn(&T) -> Matrix {
        match n {
            0 => None,
            1 => Some(f(param)),
            _ => {
                let mut init: Matrix = f(param);
                for _ in 1..n {
                    init = init + f(param)
                }
                Some(init / (n as f64))
            }
        }
    }
}

impl MonteCarlo for VectorSimul {
    type RHS = Vec<f64>;

    fn sample<'a, F, T>(f: &'a F, param: &'a T) -> Vec<f64>
    where
        F: Fn(&'a T) -> Vec<f64> {
        f(param)
    }

    fn simulate<'a, F, T>(f: &'a F, param: &'a T, n: usize) -> Option<Vec<f64>>
    where
        F: Fn(&T) -> Vec<f64> {
        match n {
            0 => None,
            1 => Some(f(param)),
            _ => {
                let mut init: Vec<f64> = f(param);
                for _ in 1..n {
                    init = init.add_vec(&f(param))
                }
                Some(init.mul_scalar(1.0 / n as f64))
            }
        }
    }
}

impl MonteCarlo for FloatSimul {
    type RHS = f64;

    fn sample<'a, F, T>(f: &'a F, param: &'a T) -> f64
    where
        F: Fn(&'a T) -> f64 {
        f(param)
    }

    fn simulate<'a, F, T>(f: &'a F, param: &'a T, n: usize) -> Option<f64>
    where
        F: Fn(&T) -> f64 {
        match n {
            0 => None,
            1 => Some(f(param)),
            _ =>  Some((0..n).fold(0.0, |acc, _| acc + f(param)) / (n as f64))
        }
    }
}
