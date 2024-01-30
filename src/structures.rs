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
    on_set: i32,
    on_match: i32,
    on_insert: i32,
    on_replace: i32,
    on_delete: i32,
}

impl Costs {
    pub fn new() -> Self {
        let on_set: i32 = 0;
        let on_match: i32 = 0;
        let on_insert: i32 = 1;
        let on_replace: i32 = 1;
        let on_delete: i32 = 1;

        Self {
            on_set,
            on_match,
            on_insert,
            on_replace,
            on_delete,
        }
    }

    pub fn set_insert(&mut self, value: i32) {
        self.on_insert = value;
    }

    pub fn set_replace(&mut self, value: i32) {
        self.on_replace = value;
    }

    pub fn set_delete(&mut self, value: i32) {
        self.on_delete = value;
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

pub struct Results {
    distance: i32,
    sequence: Vec<Vec<i32>>,
}

impl Results {
    pub fn new(distance: i32, sequence: Vec<Vec<i32>>) -> Self {
        Self { distance, sequence }
    }

    pub fn distance(&self) -> i32 {
        self.distance
    }

    pub fn sequence(&self) -> &Vec<Vec<i32>> {
        &self.sequence
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
