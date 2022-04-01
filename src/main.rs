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
            Function {
                start: Point2 { x: 0.0, y: 0.0 },
                stopover: Some(Point2 { x: 5.0, y: 5.0 }),
                end: Point2 { x: 10.0, y: 0.0 },
            },
            Function {
                start: Point2 { x: 10.0, y: 0.0 },
                stopover: None,
                end: Point2 { x: 20.0, y: 10.0 },
            },
            Function {
                start: Point2 { x: 20.0, y: 10.0 },
                stopover: Some(Point2 { x: 25.0, y: 15.0 }),
                end: Point2 { x: 30.0, y: 10.0 },
            },
            Function {
                start: Point2 { x: 30.0, y: 10.0 },
                stopover: None,
                end: Point2 { x: 40.0, y: 20.0 },
            },
            Function {
                start: Point2 { x: 40.0, y: 20.0 },
                stopover: Some(Point2 { x: 45.0, y: 25.0 }),
                end: Point2 { x: 50.0, y: 20.0 },
            },
            Function {
                start: Point2 { x: 50.0, y: 20.0 },
                stopover: None,
                end: Point2 { x: 60.0, y: 30.0 },
            },
        ],
    };
    a_different_step_function.print(&Notation::ZeroToThePowerOfZeroIsOneDesmos);
}
