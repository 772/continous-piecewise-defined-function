struct Point2 {
    x: usize,
    y: usize,
}

struct PiecewiseDefinedFunction {
    function: Vec<Function>,
}

struct Function {
    start: Point2,
    stopover: Option<Point2>,
    end: Point2,
}

enum Notation {
    IversonBracket,
}

impl Function {
    fn print(self, _notation: &Notation) {
        match self.stopover {
            Some(stopover) => {
                let x1: f32 = self.start.x as f32;
                let x2: f32 = stopover.x as f32;
                let x3: f32 = self.end.x as f32;
                let y1: f32 = self.start.y as f32;
                let y2: f32 = stopover.y as f32;
                let y3: f32 = self.end.y as f32;
                let a: f32 = (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2))
                    / ((x1 - x2) * (x1 - x3) * (x3 - x2));
                let b: f32 = (x1 * x1 * (y2 - y3) + x2 * x2 * (y3 - y1) + x3 * x3 * (y1 - y2))
                    / ((x1 - x2) * (x1 - x3) * (x2 - x3));
                let c: f32 = (x1 * x1 * (x2 * y3 - x3 * y2)
                    + x1 * (x3 * x3 * y2 - x2 * x2 * y3)
                    + x2 * x3 * y1 * (x2 - x3))
                    / ((x1 - x2) * (x1 - x3) * (x2 - x3));
                println!(
                    "{} * x^2 + {} * x + {} * [{} <= x <= {}] +",
                    a, b, c, self.start.x, self.end.x
                );
            }
            None => {
                let func = ((self.end.y - self.start.y) / (self.end.x - self.start.x)).to_string()
                    + " * (x - "
                    + &self.start.x.to_string()
                    + ") + "
                    + &self.start.y.to_string();
                println!("{} * [{} <= x <= {}] +", func, self.start.x, self.end.x);
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
                start: Point2 { x: 0, y: 0 },
                stopover: Some(Point2 { x: 5, y: 5 }),
                end: Point2 { x: 10, y: 0 },
            },
            Function {
                start: Point2 { x: 10, y: 0 },
                stopover: None,
                end: Point2 { x: 20, y: 10 },
            },
            Function {
                start: Point2 { x: 20, y: 10 },
                stopover: Some(Point2 { x: 25, y: 15 }),
                end: Point2 { x: 30, y: 10 },
            },
            Function {
                start: Point2 { x: 30, y: 10 },
                stopover: None,
                end: Point2 { x: 40, y: 20 },
            },
            Function {
                start: Point2 { x: 40, y: 20 },
                stopover: Some(Point2 { x: 45, y: 25 }),
                end: Point2 { x: 50, y: 20 },
            },
            Function {
                start: Point2 { x: 50, y: 20 },
                stopover: None,
                end: Point2 { x: 60, y: 30 },
            },
        ],
    };
    a_different_step_function.print(&Notation::IversonBracket);
}
