fn f(x1: f64, x2: f64) -> f64 {
    x1 * x1 - 2.0 * x1 * x2 + 10.0 * x2 * x2 + x1 - 2.0 * x2
}

fn gradient(x1: f64, x2: f64) -> (f64, f64) {
    let df_dx1 = 2.0 * x1 - 2.0 * x2 + 1.0;
    let df_dx2 = -2.0 * x1 + 20.0 * x2 - 2.0;

    (df_dx1, df_dx2)
}

fn steepest_descent(alpha: f64, iterations: usize) {
    // Starting point x^0 = (0, 1)
    let mut x1 = 0.0;
    let mut x2 = 1.0;

    println!("\n========================================");
    println!("Stepsize alpha = {}", alpha);
    println!("========================================");

    println!(
        "{:<8} {:>14} {:>14} {:>16}",
        "k", "x1", "x2", "f(x)"
    );

    for k in 1..=iterations {
        // Compute gradient at current point
        let (g1, g2) = gradient(x1, x2);

        // Steepest descent:
        // x^(k+1) = x^k - alpha * gradient(f(x^k))
        x1 = x1 - alpha * g1;
        x2 = x2 - alpha * g2;

        // Print only the iterations requested in the exercise
        if k == 1 || k == 10 || k == 20 || k == 30 || k == 40 || k == 50 {
            println!("{:<8} {:>14.6} {:>14.6} {:>16.6}", k, x1, x2, f(x1, x2));
        }
    }
}

fn main() {
    let iterations = 50;

    steepest_descent(0.01, iterations);
    steepest_descent(0.05, iterations);
    steepest_descent(0.1, iterations);

    println!("=================================================================");
    println!("Comparing with x*=(-0.444444, 0.055556) and f(x*) = -0.277778 we can see that in case of alpha= 0.05 we are very close at 50 iteration");
    println!("So we can say : alpha = 0.05 is convergence");
}


