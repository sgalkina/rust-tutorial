use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

// Display: 3.3 + 7.2i
// Debug: Complex { real: 3.3, imag: 7.2 }

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0} + {1}i", self.real, self.imag)
    }
}

fn main() {
    let complex = Complex{real: 3.3, imag: 7.2};
    println!("Compare Complex");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}