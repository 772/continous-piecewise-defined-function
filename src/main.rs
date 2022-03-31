#[allow(dead_code)]
struct Point2 {
    x: f32,
    y: f32,
}

struct PiecewiseDefinedFunction {
    function: Vec<Function>,
}

struct Function {
    start: Point2,
    vertex: Option<Point2>,
    end: Point2,
}

enum Notation {
    IversonBracket,
}

impl Function {
    fn print(self, notation: &Notation) {
        match notation {
            Notation::IversonBracket => match self.vertex {
                Some(_vertex) => {
                    println!("quadratic_function * [{} <= x <= {}] +", self.start.x, self.end.x);
                }
                None => {
                    println!("linerar_function * [{} <= x <= {}] +", self.start.x, self.end.x);
                }
            },
        }
    }
}

impl PiecewiseDefinedFunction {
    fn print(self, notation: &Notation) {
        for function in self.function {
            function.print(notation);
        }
    }
}

fn main() {
    let a_different_step_function = PiecewiseDefinedFunction {
        function: vec![
            Function {
                start: Point2 { x: 0.0, y: 0.0 },
                vertex: Some(Point2 { x: 5.0, y: 20.0 }),
                end: Point2 { x: 10.0, y: 0.0 },
            },
            Function {
                start: Point2 { x: 10.0, y: 0.0 },
                vertex: None,
                end: Point2 { x: 20.0, y: 10.0 },
            },
            Function {
                start: Point2 { x: 20.0, y: 10.0 },
                vertex: Some(Point2 { x: 25.0, y: 15.0 }),
                end: Point2 { x: 30.0, y: 10.0 },
            },
            Function {
                start: Point2 { x: 30.0, y: 10.0 },
                vertex: None,
                end: Point2 { x: 40.0, y: 20.0 },
            },
            Function {
                start: Point2 { x: 40.0, y: 20.0 },
                vertex: Some(Point2 { x: 45.0, y: 25.0 }),
                end: Point2 { x: 50.0, y: 20.0 },
            },
            Function {
                start: Point2 { x: 50.0, y: 20.0 },
                vertex: None,
                end: Point2 { x: 60.0, y: 30.0 },
            },
        ],
    };
    a_different_step_function.print(&Notation::IversonBracket);
}
