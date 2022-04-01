# piecewise-defined-function

This is an experimental tool to create piecewise-defined functions that contain only linear and quadratic equations.

## Example

```
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
```

Output function:

```
(-0.2 * x^2 + 2 * x + -0) * (((1+(x-0)/((x-0)^2+0^{((x-0)^2)})^{(1/2)}*2)^2-1)/8) * (1-((1+(x-10)/((x-10)^2+0^{((x-10)^2)})^{(1/2)}*2)^2-1)/8) + (1 * (x - 10) + 0) * (((1+(x-10)/((x-10)^2+0^{((x-10)^2)})^{(1/2)}*2)^2-1)/8) * (1-((1+(x-20)/((x-20)^2+0^{((x-20)^2)})^{(1/2)}*2)^2-1)/8) + (-0.2 * x^2 + 10 * x + -110) * (((1+(x-20)/((x-20)^2+0^{((x-20)^2)})^{(1/2)}*2)^2-1)/8) * (1-((1+(x-30)/((x-30)^2+0^{((x-30)^2)})^{(1/2)}*2)^2-1)/8) + (1 * (x - 30) + 10) * (((1+(x-30)/((x-30)^2+0^{((x-30)^2)})^{(1/2)}*2)^2-1)/8) * (1-((1+(x-40)/((x-40)^2+0^{((x-40)^2)})^{(1/2)}*2)^2-1)/8) + (-0.2 * x^2 + 18 * x + -380) * (((1+(x-40)/((x-40)^2+0^{((x-40)^2)})^{(1/2)}*2)^2-1)/8) * (1-((1+(x-50)/((x-50)^2+0^{((x-50)^2)})^{(1/2)}*2)^2-1)/8) + (1 * (x - 50) + 20) * (((1+(x-50)/((x-50)^2+0^{((x-50)^2)})^{(1/2)}*2)^2-1)/8) * (1-((1+(x-60)/((x-60)^2+0^{((x-60)^2)})^{(1/2)}*2)^2-1)/8) + 0
```

Function plotted in Desmos.

![Example](example.png)

## Available notations

- Iverson brackets
- ZeroToThePowerOfZeroIsOne notation from https://github.com/772/text-to-graph-of-a-function
- ZeroToThePowerOfZeroIsOne notation (optimized for Desmos)