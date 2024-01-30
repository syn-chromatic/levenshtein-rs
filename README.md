## `⌽` Levenshtein
Compute operational differences between two sequences using the Levenshtein algorithm. 


___
### `➢` Usage:
#### `⤷` Basic Usage
```rust
use levenshtein::Levenshtein;
use levenshtein::Results;

fn main() {
    let levenshtein: Levenshtein = Levenshtein::new();

    let results: Results = levenshtein.calculate("test", "text");

    let distance: i32 = results.distance();
    let sequence: &Vec<Vec<i32>> = results.sequence();

    println!("Distance: {}", distance);
    println!("Sequence: {:?}", sequence);
}
```


#### `⤷` With replace operation cost of 2:
```rust
use levenshtein::Levenshtein;
use levenshtein::Results;

fn main() {
    let mut levenshtein: Levenshtein = Levenshtein::new();
    levenshtein.costs().set_replace(2);

    let results: Results = levenshtein.calculate("test", "text");

    let distance: i32 = results.distance();
    let sequence: &Vec<Vec<i32>> = results.sequence();

    println!("Distance: {}", distance);
    println!("Sequence: {:?}", sequence);
}
```