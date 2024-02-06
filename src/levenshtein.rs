use crate::structures::Costs;
use crate::structures::Mapping;
use crate::structures::Position;
use crate::structures::Results;

pub struct Levenshtein {
    costs: Costs,
}

impl Levenshtein {
    pub fn new() -> Levenshtein {
        let costs: Costs = Costs::new();
        Levenshtein { costs }
    }

    pub fn set_insert_cost(&mut self, cost: i32) {
        self.costs.on_insert = cost;
    }

    pub fn set_replace_cost(&mut self, cost: i32) {
        self.costs.on_replace = cost;
    }

    pub fn set_delete_cost(&mut self, cost: i32) {
        self.costs.on_delete = cost;
    }

    pub fn calculate(&self, seq1: &str, seq2: &str) -> Results {
        let ops: Mapping = self.calculate_distance(seq1, seq2);
        let distance: i32 = ops.distance();
        let sequence: Vec<Vec<i32>> = ops.sequence;
        let results: Results = Results::new(distance, sequence);
        results
    }
}

impl Levenshtein {
    fn calculate_distance(&self, seq1: &str, seq2: &str) -> Mapping {
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
        let mut min_ops: [i32; 4] = [0; 4];
        let mut min_ops_initialized: bool = false;

        for op in ops.iter() {
            if op[0] >= 0 && op[1] >= 0 {
                if !min_ops_initialized {
                    min_ops = *op;
                    min_ops_initialized = true;
                } else if op[2] < min_ops[2] {
                    min_ops = *op;
                }
            }
        }

        if !min_ops_initialized {
            panic!("Failed to retrieve Minimum Operation.");
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
        let onset_state: bool = replace.x + replace.y == -2;
        let match_state: bool = char1 == char2;

        if onset_state {
            return map.onset_array();
        } else if match_state {
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
