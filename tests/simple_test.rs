use leven_distance::Levenshtein;
use leven_distance::Results;

const SEQ1: &str = "some_sequence";
const SEQ2: &str = "other_sequence";

#[test]
fn default_test() {
    println!("[DEFAULT TEST]");
    let lev: Levenshtein = Levenshtein::new();
    let results: Results = lev.calculate(SEQ1, SEQ2);
    let distance: i32 = results.distance();
    let array: &Vec<Vec<i32>> = results.sequence();

    println!("Distance: {}", distance);
    println!("Array: {:?}", array);

    assert!(distance == 4, "calculated distance is incorrect");
}

#[test]
fn cost_test() {
    println!("[COST TEST]");
    let mut lev: Levenshtein = Levenshtein::new();
    lev.set_replace_cost(2);

    let results: Results = lev.calculate(SEQ1, SEQ2);
    let distance: i32 = results.distance();
    let array: &Vec<Vec<i32>> = results.sequence();

    println!("Distance: {}", distance);
    println!("Array: {:?}", array);
    assert!(distance == 5, "calculated distance is incorrect");
}
