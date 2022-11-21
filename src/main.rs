use factorial::factorial_calculator::FactorialCalculator;

mod digits;
mod factorial;

fn main() {
    let problem_number = 34;
    let mut answer_p34 = 0;

    let mut fac_calc = FactorialCalculator::new();

    for num in 1u128..fac_calc.factorial(9) * 8 {
        if fac_calc.is_curious_factorial(num) {
            answer_p34 += num;
            println!("Found courious factorial: {}", num);
        }
    }

    println!(
        "The answer to problem {} of project Euler is {}.",
        problem_number, answer_p34
    );
}
