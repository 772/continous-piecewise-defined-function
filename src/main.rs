struct PiecewiseDefinedFunction {
    functions: Vec<Function>,
}

struct Function {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    vertex: f32,
}

#[allow(dead_code)]
enum Notation {
    IversonBracket,
    ZeroToThePowerOfZeroIsOne,
    ZeroToThePowerOfZeroIsOneDesmos,
    SVGPath,
    Poles,
}

fn knots(notation: &Notation, x1: f32, x2: f32) -> String {
    match notation {
        Notation::IversonBracket => {
            String::from("[".to_string() + &x1.to_string() + " <= x <= " + &x2.to_string() + "]")
        }
        Notation::ZeroToThePowerOfZeroIsOne => String::from(
            "(((1+(".to_owned()
                + "x"
                + "-"
                + &x1.to_string()
                + ")/(("
                + "x"
                + "-"
                + &x1.to_string()
                + ")^2+0^(("
                + "x"
                + "-"
                + &x1.to_string()
                + ")^2))^(1/2)*2)^2-1)/8) * (1-((1+("
                + "x"
                + "-"
                + &x2.to_string()
                + ")/(("
                + "x"
                + "-"
                + &x2.to_string()
                + ")^2+0^(("
                + "x"
                + "-"
                + &x2.to_string()
                + ")^2))^(1/2)*2)^2-1)/8)",
        ),
        Notation::ZeroToThePowerOfZeroIsOneDesmos => String::from(
            "(((1+(".to_owned()
                + "x"
                + "-"
                + &x1.to_string()
                + ")/(("
                + "x"
                + "-"
                + &x1.to_string()
                + ")^2+0^{(("
                + "x"
                + "-"
                + &x1.to_string()
                + ")^2)})^{(1/2)}*2)^2-1)/8) * (1-((1+("
                + "x"
                + "-"
                + &x2.to_string()
                + ")/(("
                + "x"
                + "-"
                + &x2.to_string()
                + ")^2+0^{(("
                + "x"
                + "-"
                + &x2.to_string()
                + ")^2)})^{(1/2)}*2)^2-1)/8)",
        ),
        Notation::SVGPath => String::from(""),
        Notation::Poles => String::from(""),
    }
}

impl Function {
    fn new(x1: f32, y1: f32, x2: f32, y2: f32, vertex: f32) -> Function {
        Function {
            x1,
            y1,
            x2,
            y2,
            vertex,
        }
    }

    fn print(self, notation: &Notation) {
		let x1 = self.x1;
		let y1 = self.y1;
		let x2 = self.x1 + (self.x2 - self.x1) / 2.0;
		let y2 = self.y1 + (self.y2 - self.y1) / 2.0 + self.vertex;
		let x3 = self.x2;
		let y3 = self.y2;
        match self.vertex {
			// Linear.
            0.0 => {
                let a = (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2))
                    / ((x1 - x2) * (x1 - x3) * (x3 - x2));
                let b = (x1 * x1 * (y2 - y3) + x2 * x2 * (y3 - y1) + x3 * x3 * (y1 - y2))
                    / ((x1 - x2) * (x1 - x3) * (x2 - x3));
                let c = (x1 * x1 * (x2 * y3 - x3 * y2)
                    + x1 * (x3 * x3 * y2 - x2 * x2 * y3)
                    + x2 * x3 * y1 * (x2 - x3))
                    / ((x1 - x2) * (x1 - x3) * (x2 - x3));
                print!(
                    "({} * x^2 + {} * x + {}) * {} + ",
                    a,
                    b,
                    c,
                    knots(notation, x1, x3)
                );
            }
            // Quadratic.
            _ => {
                let func = ((y3 - y1) / (x3 - x1)).to_string()
                    + " * (x - "
                    + &x1.to_string()
                    + ") + "
                    + &y1.to_string();
                print!(
                    "({}) * {} + ",
                    func,
                    knots(notation, x1, x3)
                );
            }
        }
    }
}

impl PiecewiseDefinedFunction {
    fn print(self, notation: &Notation) {
        for function in self.functions {
            function.print(notation);
        }
        println!("0");
    }

    fn append(mut self, piecewise_defined_function: &PiecewiseDefinedFunction) {
        let x = self.functions.last().unwrap().x2;
        for function in &piecewise_defined_function.functions {
            self.functions.push(Function::new(
                function.x1 + x,
                function.y1,
                function.x2 + x,
                function.y2,
                function.vertex,
            ));
        }
    }
}

fn main() {
    let a_different_step_function = PiecewiseDefinedFunction {
        functions: vec![
            Function::new(0.0, 0.0, 10.0, 0.0, 5.0),
            Function::new(10.0, 0.0, 20.0, 10.0, 0.0),
            Function::new(20.0, 10.0, 30.0, 10.0, 5.0),
            Function::new(30.0, 10.0, 40.0, 20.0, 0.0),
            Function::new(40.0, 20.0, 50.0, 20.0, 5.0),
            Function::new(50.0, 20.0, 60.0, 30.0, 0.0),
        ],
    };
    a_different_step_function.print(&Notation::ZeroToThePowerOfZeroIsOneDesmos);
}

#[cfg(test)]
mod tests {}
