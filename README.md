# levenshtein-rs
Compute operational differences between two sequences using the Levenshtein algorithm. 

___
## Usage:
```rust
let lev = Levenshtein::new("test", "text");
let distance: i32 = lev.distance();
let array: Vec<Vec<i32>> = lev.sequence_array();

println!("Distance: {}", distance);
println!("Array: {:?}", array);
```
