struct Levenshtein {
    seq1: String,
    seq2: String,
}

impl Levenshtein {
    pub fn new(seq1: &str, seq2: &str) -> Levenshtein {
        let seq1: String = seq1.to_string();
        let seq2: String = seq2.to_string();
        Levenshtein { seq1, seq2 }
    }

    pub fn distance(&self) -> i32 {
        let seq1: &str = self.seq1.as_str();
        let seq2: &str = self.seq2.as_str();
        let (_, distance): (Vec<Vec<i32>>, i32) = self.calculate_distance(seq1, seq2);
        distance
    }

    pub fn sequence_array(&self) -> Vec<Vec<i32>> {
        let seq1: &str = self.seq1.as_str();
        let seq2: &str = self.seq2.as_str();
        let (seq_array, _): (Vec<Vec<i32>>, i32) = self.calculate_distance(seq1, seq2);
        seq_array
    }

    fn calculate_distance(&self, seq1: &str, seq2: &str) -> (Vec<Vec<i32>>, i32) {
        let (mut seq_array, seq_lookup): (Vec<Vec<i32>>, Vec<Vec<(usize, char)>>) =
            self.get_sequence_array(&seq1, &seq2);

        let ops_costs: [i32; 5] = [0, 0, 1, 1, 1];

        let seq1_len: usize = seq1.len() + 1;
        let seq2_len: usize = seq2.len() + 1;

        for x in 0..(seq1_len) as i32 {
            for y in 0..(seq2_len) as i32 {
                let op_values: [i32; 4] =
                    self.dynamic_operations(x, y, &seq_array, &seq_lookup, seq1_len, seq2_len);
                let op_value: i32 = op_values[2];
                let op_key: i32 = op_values[3];
                let op_cost: i32 = ops_costs[op_key as usize];
                seq_array[y as usize][x as usize] = op_value + op_cost;
            }
        }
        let dist_val: i32 = seq_array[seq2_len - 1][seq1_len - 1];
        (seq_array, dist_val)
    }

    fn get_sequence_array(
        &self,
        seq1: &str,
        seq2: &str,
    ) -> (Vec<Vec<i32>>, Vec<Vec<(usize, char)>>) {
        let seq1_c: Vec<char> = std::iter::once('\0').chain(seq1.chars()).collect();
        let seq2_c: Vec<char> = std::iter::once('\0').chain(seq2.chars()).collect();

        let seq_arr: Vec<Vec<i32>> = vec![vec![0; seq1_c.len()]; seq2_c.len()];
        let mut seq_lookup: Vec<Vec<(usize, char)>> = Vec::with_capacity(2);

        for s_index in 0..2 {
            let seq: &Vec<char> = if s_index == 0 { &seq1_c } else { &seq2_c };
            let mut lookup = Vec::with_capacity(seq.len());
            for (l_index, &letter) in seq.iter().enumerate() {
                lookup.push((l_index, letter));
            }
            seq_lookup.push(lookup);
        }

        (seq_arr, seq_lookup)
    }

    fn get_min_ops(&self, ops_array: [[i32; 4]; 3]) -> [i32; 4] {
        let mut min_ops: [i32; 4] = [0, 0, 0, 0];
        let mut min_ops_initialized = false;

        for op in ops_array.iter() {
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

    fn get_insert(&self, x: i32, y: i32, seq1_len: usize, seq2_len: usize) -> (i32, i32) {
        if seq1_len < seq2_len {
            return (x, y - 1);
        }
        (x - 1, y)
    }

    fn get_replace(&self, x: i32, y: i32) -> (i32, i32) {
        (x - 1, y - 1)
    }

    fn get_delete(&self, x: i32, y: i32, seq1_len: usize, seq2_len: usize) -> (i32, i32) {
        if seq1_len < seq2_len {
            return (x - 1, y);
        }
        (x, y - 1)
    }

    fn get_value(&self, x: i32, y: i32, seq_array: &Vec<Vec<i32>>) -> i32 {
        if x >= 0 && y >= 0 {
            return seq_array[y as usize][x as usize];
        }
        0
    }

    fn dynamic_operations(
        &self,
        x: i32,
        y: i32,
        seq_array: &Vec<Vec<i32>>,
        seq_lookup: &Vec<Vec<(usize, char)>>,
        seq1_len: usize,
        seq2_len: usize,
    ) -> [i32; 4] {
        let (_, char1): (usize, char) = seq_lookup[0][x as usize];
        let (_, char2): (usize, char) = seq_lookup[1][y as usize];

        let (x_ins, y_ins): (i32, i32) = self.get_insert(x, y, seq1_len, seq2_len);
        let (x_rep, y_rep): (i32, i32) = self.get_replace(x, y);
        let (x_del, y_del): (i32, i32) = self.get_delete(x, y, seq1_len, seq2_len);

        let ins_val: i32 = self.get_value(x_ins, y_ins, seq_array);
        let rep_val: i32 = self.get_value(x_rep, y_rep, seq_array);
        let del_val: i32 = self.get_value(x_del, y_del, seq_array);

        let onset_state: bool = x_rep + y_rep == -2;
        let match_state: bool = char1 == char2;
        let op_array: [i32; 4];

        if onset_state {
            op_array = [0, 0, 0, 0] // x, y, val, onset;
        } else if match_state {
            op_array = [x_rep, y_rep, rep_val, 1] // x, y, val, match;
        } else {
            let ops_array: [[i32; 4]; 3] = [
                [x_ins, y_ins, ins_val, 2], // x, y, val, insert
                [x_rep, y_rep, rep_val, 3], // x, y, val, replace
                [x_del, y_del, del_val, 4], // x, y, val, delete
            ];

            op_array = self.get_min_ops(ops_array);
        }
        op_array
    }
}

fn main() {
    let lev = Levenshtein::new("test", "text");
    let distance: i32 = lev.distance();
    let array: Vec<Vec<i32>> = lev.sequence_array();
    println!("Distance: {}", distance);
    println!("Array: {:?}", array);
}
