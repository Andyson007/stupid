use fizzbuzzmacro::fizzbuzz;

fn main() {
    for i in 1..=300 {
        // println!(
        //     "{}",
        //     fizzbuzz!(i, ("Fizz", 3), ("Buzz", 5), ("Fuzz", 7), ("Fozz", 11))
        // );
        fizzbuzz!(i, ("Fizz", 3), ("Buzz", 5), ("Fuzz", 7));
    }
}
