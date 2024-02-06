/// A struct to hold the results of a Levenshtein distance calculation.
///
/// This struct contains the Levenshtein distance between two sequences and the sequence of
/// operations (represented as a matrix of integers) that describe how to transform the first sequence into the second.
pub struct Results {
    /// The Levenshtein distance between two sequences.
    distance: i32,
    /// The sequence of operations required to transform one sequence into another.
    /// This is represented as a matrix where each cell contains an operation code.
    sequence: Vec<Vec<i32>>,
}

impl Results {
    pub fn new(distance: i32, sequence: Vec<Vec<i32>>) -> Self {
        Self { distance, sequence }
    }

    /// Returns the Levenshtein distance.
    ///
    /// This is the minimum number of single-character edits (insertions, deletions, or substitutions)
    /// required to change one sequence into the other.
    ///
    /// # Returns
    ///
    /// Returns an `i32` representing the Levenshtein distance.
    ///
    /// # Examples
    ///
    /// ```
    /// let results = Results::new(3, vec![vec![0, 1, 2], vec![1, 2, 3]]);
    /// assert_eq!(results.distance(), 3);
    /// ```
    pub fn distance(&self) -> i32 {
        self.distance
    }

    /// Returns a reference to the sequence of operations matrix.
    ///
    /// Each element in the matrix represents an operation code corresponding to an edit operation.
    ///
    /// # Returns
    ///
    /// Returns a reference to a `Vec<Vec<i32>>` representing the sequence of operations.
    ///
    /// # Examples
    ///
    /// ```
    /// let results = Results::new(3, vec![vec![0, 1, 2], vec![1, 2, 3]]);
    /// assert_eq!(results.sequence(), &vec![vec![0, 1, 2], vec![1, 2, 3]]);
    /// ```
    pub fn sequence(&self) -> &Vec<Vec<i32>> {
        &self.sequence
    }
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub struct Costs {
    pub on_set: i32,
    pub on_match: i32,
    pub on_insert: i32,
    pub on_replace: i32,
    pub on_delete: i32,
}

impl Costs {
    pub fn new() -> Self {
        Self {
            on_set: 0,
            on_match: 0,
            on_insert: 1,
            on_replace: 1,
            on_delete: 1,
        }
    }

    pub fn as_slice(&self) -> [i32; 5] {
        let slice: [i32; 5] = [
            self.on_set,
            self.on_match,
            self.on_insert,
            self.on_replace,
            self.on_delete,
        ];
        slice
    }
}

pub struct Mapping {
    pub length: (usize, usize),
    pub sequence: Vec<Vec<i32>>,
    pub lookup: Vec<Vec<(usize, char)>>,
}

impl Mapping {
    fn proc_sequence(seq: &str) -> Vec<char> {
        std::iter::once('\0').chain(seq.chars()).collect()
    }

    fn proc_lookup(lookup: &mut Vec<Vec<(usize, char)>>, seq: &Vec<char>) {
        let mut char_lookup: Vec<(usize, char)> = Vec::with_capacity(seq.len());
        for (l_index, &letter) in seq.iter().enumerate() {
            char_lookup.push((l_index, letter));
        }
        lookup.push(char_lookup);
    }
}

impl Mapping {
    pub fn new(seq1: &str, seq2: &str) -> Self {
        let seq1: Vec<char> = Self::proc_sequence(seq1);
        let seq2: Vec<char> = Self::proc_sequence(seq2);
        let length: (usize, usize) = (seq1.len(), seq2.len());

        let sequence: Vec<Vec<i32>> = vec![vec![0; length.0]; length.1];
        let mut lookup: Vec<Vec<(usize, char)>> = Vec::with_capacity(2);

        Self::proc_lookup(&mut lookup, &seq1);
        Self::proc_lookup(&mut lookup, &seq2);

        Self {
            length,
            sequence,
            lookup,
        }
    }

    pub fn distance(&self) -> i32 {
        let (l1, l2): (usize, usize) = self.length;
        self.sequence[l2 - 1][l1 - 1]
    }

    pub fn value(&self, position: &Position) -> i32 {
        let (x, y) = (position.x, position.y);
        if x >= 0 && y >= 0 {
            return self.sequence[y as usize][x as usize];
        }
        0
    }

    pub fn insert_position(&self, x: i32, y: i32) -> Position {
        if self.length.0 < self.length.1 {
            return Position::new(x, y - 1);
        }
        Position::new(x - 1, y)
    }

    pub fn replace_position(&self, x: i32, y: i32) -> Position {
        Position::new(x - 1, y - 1)
    }

    pub fn delete_position(&self, x: i32, y: i32) -> Position {
        if self.length.0 < self.length.1 {
            return Position::new(x - 1, y);
        }
        Position::new(x, y - 1)
    }

    pub fn onset_array(&self) -> [i32; 4] {
        [0, 0, 0, 0]
    }

    pub fn match_array(&self, replace: &Position) -> [i32; 4] {
        let value: i32 = self.value(replace);
        [replace.x, replace.y, value, 1]
    }

    pub fn insert_array(&self, insert: &Position) -> [i32; 4] {
        let value: i32 = self.value(insert);
        [insert.x, insert.y, value, 2]
    }

    pub fn replace_array(&self, replace: &Position) -> [i32; 4] {
        let value: i32 = self.value(replace);
        [replace.x, replace.y, value, 3]
    }

    pub fn delete_array(&self, delete: &Position) -> [i32; 4] {
        let value: i32 = self.value(delete);
        [delete.x, delete.y, value, 4]
    }
}
