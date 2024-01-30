use levenshtein::Levenshtein;
use levenshtein::Results;

const SEQ1: &str = "test";
const SEQ2: &str = "text";

#[test]
fn simple_test() {
    let lev: Levenshtein = Levenshtein::new();
    let results: Results = lev.calculate(SEQ1, SEQ2);
    let distance: i32 = results.distance();
    let array: &Vec<Vec<i32>> = results.sequence();

    println!("Distance: {}", distance);
    println!("Array: {:?}", array);
}
