struct Point2 {
    x: f32,
    y: f32,
}

struct PiecewiseDefinedFunction {
    function: Vec<Function>,
}

struct Function {
    start: Point2,
    stopover: Option<Point2>,
    end: Point2,
}

#[allow(dead_code)]
enum Notation {
    IversonBracket,
    ZeroToThePowerOfZeroIsOne,
    ZeroToThePowerOfZeroIsOneDesmos,
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
    }
}

impl Function {
    fn new(x1: f32, y1: f32, x2: f32, y2: f32, vertex: f32) -> Function {
        if hgt == 0.0 {
            Function {
                start: Point2 { x: x1, y: y1 },
                stopover: None,
                end: Point2 { x: x2, y: y2 },
            }
        } else {
            Function {
                start: Point2 { x: x1, y: y1 },
                stopover: Some(Point2 {
                    x: x1 + (x2 - x1) / 2.0,
                    y: y1 + (y2 - y1) / 2.0 + hgt,
                }),
                end: Point2 { x: x2, y: y2 },
            }
        }
    }

    fn print(self, notation: &Notation) {
        match self.stopover {
            Some(stopover) => {
                let x1 = self.start.x;
                let x2 = stopover.x;
                let x3 = self.end.x;
                let y1 = self.start.y;
                let y2 = stopover.y;
                let y3 = self.end.y;
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
            None => {
                let func = ((self.end.y - self.start.y) / (self.end.x - self.start.x)).to_string()
                    + " * (x - "
                    + &self.start.x.to_string()
                    + ") + "
                    + &self.start.y.to_string();
                print!(
                    "({}) * {} + ",
                    func,
                    knots(notation, self.start.x, self.end.x)
                );
            }
        }
    }
}

impl PiecewiseDefinedFunction {
    fn print(self, notation: &Notation) {
        for function in self.function {
            function.print(notation);
        }
        println!("0");
    }
}

fn main() {
    let a_different_step_function = PiecewiseDefinedFunction {
        function: vec![
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
