fn main() {
    // Compute 7.0 / 3.0.
    {
    }

    // What is the type of 7.0 / 3.0?
    {
    }

    // Float data types.
    {
    }

    // Float literals.
    if false {
        let a = 1_000.5; // _ is only a visual separator
        let b = 3.;      // the fractional part may be empty, but .5 is not valid
        let c = 1.5e-3;  // scientific notation
        let e = 2.5f32;
        let f = 2.5_f64;
        println!("{a} {b} {c} {e} {f}");
    }

    // Computation with floats and integers.
    {
    }

    // The float to int conversion never produces undefined behavior.
    {
    }

    // Minimum and maximum values of the floating point types.
    if false {
        println!("{:e} {:e}", f32::MIN, f32::MAX);
        println!("{:e} {:e}", f64::MIN, f64::MAX);
        println!("{:e}", f64::MIN_POSITIVE); // smallest positive normal value
        println!("{:e}", f64::EPSILON);      // distance between 1.0 and the next value
    }

    // Division by zero gives infinity.
    // (._. )?
    {
    }

    // Not a Number (NaN).
    {
    }

    // Floating point numbers are not exact.
    {
        // 0.1 + 0.2
    }

    // Floats should be compared with a tolerance.
    {
    }

    // Rounding.
    {
    }

    // Powers and roots.
    {
        // powi, powf, sqrt
    }

    // Exponential and logarithm.
    {
        // exp, ln, log10
    }

    // Trigonometric functions work with radians.
    {
        // to_radians, to_degrees
    }

    // Mathematical constants.
    {
        // use std::f64::consts::{PI, E, SQRT_2};
    }

    // Printing with a given number of decimals.
    {
    }
}
