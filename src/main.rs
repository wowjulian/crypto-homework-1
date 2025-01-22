use clap::Parser;

/// Simple program to calculate gcd and stuff
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// An integer a, which is the modulus
    #[arg(short, long)]
    a: i64,

    /// non-negative integer b that is less than a.
    #[arg(short, long)]
    b: i64,
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

fn print_x_and_y(a: i64, b: i64) {
    let mut x_prev2: i64 = 1;
    let mut x_prev1: i64 = 0;
    let mut y_prev2: i64 = 0;
    let mut y_prev1: i64 = 1;

    let mut remainder_prev2 = a;
    let mut remainder_prev1 = b;
    let mut current_remainder: i64 = -1;

    while current_remainder != 0 {
        let floored_quotient = remainder_prev2 / remainder_prev1;
        current_remainder = remainder_prev2 - (remainder_prev1 * floored_quotient);
        if current_remainder == 0 {
            break;
        }

        let current_x = x_prev2 - (floored_quotient * x_prev1);
        let current_y = y_prev2 - (floored_quotient * y_prev1);

        x_prev2 = x_prev1;
        x_prev1 = current_x;

        y_prev2 = y_prev1;
        y_prev1 = current_y;

        remainder_prev2 = remainder_prev1;
        remainder_prev1 = current_remainder;
    }

    println!("x: {}", x_prev1);
    println!("y: {}", y_prev1);
}

fn main() {
    let args = Args::parse();
    if args.b < 0 {
        println!("b must be non-negative");
        return;
    }
    if args.b >= args.a {
        println!("b must be less than a");
        return;
    }

    let gcd_result = gcd(args.a, args.b);
    println!("gcd: {}", gcd_result);
    print_x_and_y(args.a, args.b);
}
