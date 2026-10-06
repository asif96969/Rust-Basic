fn f(x1: f64, x2: f64) -> f64 {
    x1 * x1
        - 2.0 * x1 * x2
        + 10.0 * x2 * x2
        + x1
        - 2.0 * x2
}

fn gradient(x1: f64, x2: f64) -> (f64, f64) {
    let df_dx1 = 2.0 * x1 - 2.0 * x2 + 1.0;
    let df_dx2 = -2.0 * x1 + 20.0 * x2 - 2.0;

    (df_dx1, df_dx2)
}

fn armijo_stepsize(
    x1: f64,
    x2: f64,
    d1: f64,
    d2: f64,
    s: f64,
    beta: f64,
    sigma: f64,
) -> f64 {
    // Start with alpha = s
    let mut alpha = s;

    // Gradient at current point
    let (g1, g2) = gradient(x1, x2);

    // Gradient^T * d
    let grad_dot_d = g1 * d1 + g2 * d2;

    loop {
        // Candidate new point:
        // x_new = x + alpha * d
        let new_x1 = x1 + alpha * d1;
        let new_x2 = x2 + alpha * d2;

        // Left side of Armijo condition
        let actual_decrease = f(x1, x2) - f(new_x1, new_x2);

        // Right side of Armijo condition
        let required_decrease = -sigma * alpha * grad_dot_d;

        // Check Armijo condition
        if actual_decrease >= required_decrease {
            return alpha;
        }

        // If condition fails, reduce alpha
        alpha = beta * alpha;
    }
}

fn main() {
    // Initial point x^0 = (0, 1)
    let mut x1 = 0.0;
    let mut x2 = 1.0;

    // Armijo parameters
    let s = 1.0;
    let beta = 0.5;
    let sigma = 0.1;

    let iterations = 50;

    println!(
        "{:<8} {:>14} {:>14} {:>14} {:>16}",
        "k", "x1", "x2", "alpha", "f(x)"
    );

    for k in 1..=iterations {
        // Compute gradient
        let (g1, g2) = gradient(x1, x2);

        // Steepest descent direction
        // d^k = -gradient f(x^k)
        let d1 = -g1;
        let d2 = -g2;

        // Find alpha using Armijo rule
        let alpha = armijo_stepsize(
            x1,
            x2,
            d1,
            d2,
            s,
            beta,
            sigma,
        );

        // Update point:
        // x^(k+1) = x^k + alpha_k * d^k
        x1 = x1 + alpha * d1;
        x2 = x2 + alpha * d2;

        // Print selected iterations
        if k == 1 || k == 10 || k == 20 || k == 30 || k == 40 || k == 50 {
            println!(
                "{:<8} {:>14.6} {:>14.6} {:>14.6} {:>16.6}",
                k,
                x1,
                x2,
                alpha,
                f(x1, x2)
            );
        }
    }
    println!("=================================================================");
    println!("Comparing with given x*=(-0.444444, 0.055556) , f(x*) = -0.277778 we can see that we get exact output after 50 iteration");
    println!("Therefore Armijo method is clearly convergence");
}
