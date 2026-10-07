fn main() {
    // Printing to the standard output.
    {
        // {}, {x}, {:?}, print!
    }

    // Compute 1 + 1.
    {
    }

    // What is the type of 1 + 1?
    {
        // util::type_of
    }

    // Evaluate expression with parentheses.
    {
    }

    // Integer division.
    {
    }

    // Modulo operation.
    {
    }

    // Unsigned integer data types.
    {
    }

    // Signed integer data types.
    {
    }

    // Integer literals.
    if false {
        let a = 1_000_000;   // _ is only a visual separator
        let b = 0xff;        // hexadecimal
        let c = 0b1010_0011; // binary
        println!("{a} {b} {c}");
    }

    // The type of a literal can be given as a suffix.
    if false {
        let a = 42u8;
        let b = 42_i64;
        println!("{} {}", util::type_of(&a), util::type_of(&b));
    }

    // Type inference.
    {
    }

    // Variables are immutable by default.
    {
    }

    // Mutable variables.
    {
    }

    // Shadowing.
    {
        // let x = x + 1, inner block
    }

    // Constants vs immutable variables.
    {
        // Constant:
        // - Created at compile time.
        // - Type must be explicitly specified.
        // - Can be defined outside functions.

        // Variable:
        // - Created at runtime.
        // - Lives within a function.
    }


    // Minimum and maximum values of the integer types.
    {
        // ::{MIN, MAX}
    }

    // Size of the integer types in bytes.
    // The size of usize and isize depends on the platform.
    {
        // size_of::<>
    }

    // The types of operands must be the same.
    {
    }

    // Conversion with the as operator.
    {
    }

    // Integer overflow (only allowed in release mode).
    {
    }

    // Saturating addition.
    {
    }

    // Bitwise operations.
    {
        // 0b, "{:08b}"
    }

    // Shift operations.
    {
    }

    // There is no exponentiation operator, the pow method is used instead.
    {
    }

    // Absolute value and sign.
    {
        // .abs(), .signum()
    }

    // Minimum and maximum of two values.
    {
    }
}
