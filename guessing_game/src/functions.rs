fn main() {
    let f = {
        let x = 1;
        let y = 2;
        x * y
    };

    let bmi = calc_bmi(1.76, 75.3);
    println!("{:.3}", bmi);
}

fn calc_bmi(height: f64, weight: f64) -> f64 {
    //In Rust, any line that doesnt end with semicolon is treated as a value instead of a stement,
    // that is why Im not explictely returning since rust takes that last line as the
    // return value of my function.
    weight / (height * height)
}
