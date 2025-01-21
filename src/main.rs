use clap::Parser;

/// Simple program to greet a person
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
    let reminder = a % b;
    if a % b == 0 {
        return b;
    }
    return gcd(b, reminder);
}

fn main() {
    let args = Args::parse();
    if args.b < 0 {
        println!("b must be non-negative integer");
        return;
    }
    if args.b >= args.a {
        println!("b must be less than a");
        return;
    }

    let gcd_result = gcd(args.a, args.b);
    println!("gcd: {}", gcd_result);
}
