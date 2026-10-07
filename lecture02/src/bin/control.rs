fn main() {
    // Reading from the standard input.
    {
        // util::input
    }

    // Branching with if.
    {
        // Do you want a beer?
    }

    // The condition must be a bool.
    // There is no "truthy" value in Rust.
    {
    }

    // if is an expression: it has a value.
    /*{
        let age: f64 = util::input("age: ");
        let messege = if age>18.0{
            "adult"
        }else{
            "Child"
        };

        println!{"{messege}"};
    }

    // Branching in more than two directions: else if.
    {
        // Quadratic equation solver.

        let a: f64 = util::input("a: ");
        let b: f64 = util::input("b: ");
        let c: f64 = util::input("c: ");

        let discriminate = b*b - 4.0*a*c;
        if discriminate == 0.0{
            println!("One solution");
        }
        else if discriminate > 0.0 {
            println!("two solution");
        }
        else{
            println!("have no solution");
        }
    }*/

    // Matching a value against patterns: match.
    {
        // literal, |, ..=, _, x if x < 0

        let n = -5;
        match n {
            0 => println!("n is Zero"),
            1 | 2 => println!("One or Two"),
            0..=10 => println!("zero to 10"),
            x if x<0 => println!("Is a negative value"),
            _ => println!("somethig else"),
        }
    }

    // match must be exhaustive.
    {
        // match n {
        //     0 => println!("zero"),
        // }
    }

    // match is an expression, too.
    {
        let n = 2;
        let messege = match n {
            1 => "one",
            2 => "two",
            _ => "Something else",
        };

        println!("{messege}");
    }

    // The simplest loop: loop.
    {
        // break, continue

        let mut n = 0;
        loop{
            n += 1;
            if n==3{
                continue;
            }
            println!("{n}");
            if n==5 {
                break;
            }
        }
    }

    // loop is an expression: break can return a value.
    {
        // The first square number over 100.
        let mut n = 1;
        
        loop {
            
            let square_number = n*n;
            if square_number > 100 {
                break;
            }
            n += 1;
        println!("{square_number}");
            
        }

        let mut m = 1;
        let result = loop {
            
            let square_number = m*m;
            if square_number > 100 {
                break square_number;
            }
            m += 1;
            
        };

        println!("First square over 100 : {}", result);
    }

    // while.
    {
        // How many Collatz steps does it take to get from 27 to 1?
        let mut n = 27;
        let mut step = 0;

        while n != 1 {
            if n % 2 == 0 {
                n/=2;
            }else{
                n = n*3+1;
            }
            step+=1;
        }
        println!("Total number of steps: {step}");
    }

    // for and ranges.
    {
        // a..b, a..=b, .rev(), .step_by()
    }

    // Example: the first n square numbers.
    {
        //let limit = util::input("Square of 1st :");
        let limit = 6;
        for n in 1..=limit {
            let result = n*n;
            println!("{result}");
        }
    }

    // Example: a triangle built from star characters.
    {
        // *
        // **
        // ***
        // ****

        for _n in 1..=4{
            for _m in 0.._n{
                print!("*"); // print without moving to new line
            }

            println!(); // print and go to next line
        }

        
    }

    // Leaving an outer loop: labels.
    {
        // The first pair whose product is over 20.
        //'outer: for
    }

    // Statements and expressions.
    {
        // An expression has a value, a statement does not.
        // In Rust, a block is an expression: its value is the last expression
        // inside it, written without a semicolon.

        // With a closing semicolon the block produces the unit value (),
        // the only value of the unit type. That is what `if` without `else`,
        // `while` and `for` produce as well.
    }
}
