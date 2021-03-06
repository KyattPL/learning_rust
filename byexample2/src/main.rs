use std::fmt;

#[derive(Debug)]
struct Matrix(f64, f64, f64, f64);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    let test = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", test);
    println!("Debug: {:?}", test);
    println!("Transposed:");
    println!("{}", transpose(&test));
}

fn transpose(mat: &Matrix) -> Matrix {
    Matrix(mat.0, mat.2, mat.1, mat.3)
}
