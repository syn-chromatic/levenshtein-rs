//! # Levenshtein Distance Calculator
//!
//! `levenshtein` is a library for calculating the Levenshtein distance between two sequences.
//! It provides customization for the costs of insertion, deletion, and replacement operations.

use crate::structures::Costs;
use crate::structures::Mapping;
use crate::structures::Position;
use crate::structures::Results;

/// A struct for calculating Levenshtein distance.
///
/// This struct provides methods to set custom costs for insert, delete, and replace operations
/// and a method to calculate the Levenshtein distance between two sequences.
pub struct Levenshtein {
    costs: Costs,
}

impl Levenshtein {
    /// Constructs a new `Levenshtein`.
    ///
    /// Initializes with default costs for insertion, deletion, and replacement operations.
    ///
    /// # Examples
    ///
    /// ```
    /// let levenshtein = Levenshtein::new();
    /// ```
    pub fn new() -> Levenshtein {
        let costs: Costs = Costs::new();
        Levenshtein { costs }
    }

    /// Sets the cost of an insertion operation.
    ///
    /// # Arguments
    ///
    /// * `cost` - The cost of an insertion operation.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut levenshtein = Levenshtein::new();
    /// levenshtein.set_insert_cost(2);
    /// ```
    pub fn set_insert_cost(&mut self, cost: i32) {
        self.costs.on_insert = cost;
    }

    /// Sets the cost of a replacement operation.
    ///
    /// # Arguments
    ///
    /// * `cost` - The cost of a replacement operation.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut levenshtein = Levenshtein::new();
    /// levenshtein.set_replace_cost(1);
    /// ```
    pub fn set_replace_cost(&mut self, cost: i32) {
        self.costs.on_replace = cost;
    }

    /// Sets the cost of a deletion operation.
    ///
    /// # Arguments
    ///
    /// * `cost` - The cost of a deletion operation.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut levenshtein = Levenshtein::new();
    /// levenshtein.set_delete_cost(2);
    /// ```
    pub fn set_delete_cost(&mut self, cost: i32) {
        self.costs.on_delete = cost;
    }

    /// Calculates the Levenshtein distance between two sequences.
    ///
    /// Returns a `Results` struct containing the distance and the sequence of operations.
    ///
    /// # Arguments
    ///
    /// * `seq1` - The first sequence.
    /// * `seq2` - The second sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// let levenshtein = Levenshtein::new();
    /// let results = levenshtein.calculate("kitten", "sitting");
    /// ```
    pub fn calculate(&self, seq1: &str, seq2: &str) -> Results {
        let map: Mapping = self.calculate_map(seq1, seq2);
        let distance: i32 = map.distance();
        let sequence: Vec<Vec<i32>> = map.sequence;
        let results: Results = Results::new(distance, sequence);
        results
    }
}

impl Levenshtein {
    fn calculate_map(&self, seq1: &str, seq2: &str) -> Mapping {
        let mut map: Mapping = Mapping::new(seq1, seq2);
        let costs: [i32; 5] = self.costs.as_slice();

        for x in 0..(map.length.0) as i32 {
            for y in 0..(map.length.1) as i32 {
                let values: [i32; 4] = self.dynamic_operations(x, y, &map);
                let value: i32 = values[2];
                let key: i32 = values[3];
                let cost: i32 = costs[key as usize];
                map.sequence[y as usize][x as usize] = value + cost;
            }
        }

        map
    }

    fn get_min_ops(&self, ops: [[i32; 4]; 3]) -> [i32; 4] {
        let mut min_ops: [i32; 4] = ops[0];

        for op in ops[1..].iter() {
            if op[0] >= 0 && op[1] >= 0 {
                if min_ops[0] < 0 || min_ops[1] < 0 {
                    min_ops = *op;
                } else if op[2] < min_ops[2] {
                    min_ops = *op;
                }
            }
        }

        min_ops
    }

    fn get_operations_array(
        &self,
        char1: char,
        char2: char,
        map: &Mapping,
        insert: Position,
        replace: Position,
        delete: Position,
    ) -> [i32; 4] {
        if replace.x + replace.y == -2 {
            return map.onset_array();
        } else if char1 == char2 {
            return map.match_array(&replace);
        }

        let ops: [[i32; 4]; 3] = [
            map.insert_array(&insert),
            map.replace_array(&replace),
            map.delete_array(&delete),
        ];
        self.get_min_ops(ops)
    }

    fn dynamic_operations(&self, x: i32, y: i32, map: &Mapping) -> [i32; 4] {
        let (_, char1): (usize, char) = map.lookup[0][x as usize];
        let (_, char2): (usize, char) = map.lookup[1][y as usize];

        let insert: Position = map.insert_position(x, y);
        let replace: Position = map.replace_position(x, y);
        let delete: Position = map.delete_position(x, y);

        self.get_operations_array(char1, char2, map, insert, replace, delete)
    }
}
