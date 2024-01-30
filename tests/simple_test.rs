use levenshtein::Levenshtein;
use levenshtein::Results;

#[test]
fn simple_test() {
    let lev: Levenshtein = Levenshtein::new();
    let results: Results = lev.calculate("test", "text");
    let distance: i32 = results.distance();
    let array: &Vec<Vec<i32>> = results.sequence();

    println!("Distance: {}", distance);
    println!("Array: {:?}", array);
}
