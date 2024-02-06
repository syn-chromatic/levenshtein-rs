## `⌽` Levenshtein
Compute operational differences between two sequences using the Levenshtein algorithm. 


___
### `➢` Usage:
#### `⤷` Basic Usage
```rust
use levenshtein::Levenshtein;
use levenshtein::Results;

fn main() {
    let mut levenshtein: Levenshtein = Levenshtein::new();

    // Specify costs as you see fit, the default is 1 for all parameters.
    levenshtein.set_insert_cost(2);
    levenshtein.set_replace_cost(2);
    levenshtein.set_delete_cost(2);

    let results: Results = levenshtein.calculate("test", "text");

    let distance: i32 = results.distance();
    let sequence: &Vec<Vec<i32>> = results.sequence();

    println!("Distance: {}", distance);
    println!("Sequence: {:#?}", sequence);
}
```

