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

impl Function {
    fn print(self) {
        match self.vertex {
            Some(vertex) => {
                println!(
                    "({}|{}) ---> ({}|{}) ---> ({}|{})",
                    self.start.x, self.start.x, vertex.x, vertex.y, self.end.x, self.end.y
                );
            }
            None => {
                println!(
                    "({}|{}) ---> ({}|{})",
                    self.start.x, self.start.x, self.end.x, self.end.y
                );
            }
        }
    }
}

impl PiecewiseDefinedFunction {
    fn print(self) {
        for function in self.function {
            function.print();
        }
    }
}

fn main() {
    let mut piecewise_defined_function = PiecewiseDefinedFunction { function: vec![] };
    let func1 = Function {
        start: Point2 { x: 0.0, y: 0.0 },
        vertex: None,
        end: Point2 { x: -20.0, y: 0.0 },
    };
    let func2 = Function {
        start: Point2 { x: 0.0, y: 0.0 },
        vertex: Some(Point2 { x: 10.0, y: 20.0 }),
        end: Point2 { x: 20.0, y: 40.0 },
    };
    let func3 = Function {
        start: Point2 { x: 0.0, y: 0.0 },
        vertex: None,
        end: Point2 { x: 1.0, y: 0.0 },
    };
    piecewise_defined_function.function.push(func1);
    piecewise_defined_function.function.push(func2);
    piecewise_defined_function.function.push(func3);
    piecewise_defined_function.print();
}
