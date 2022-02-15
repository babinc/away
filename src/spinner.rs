pub struct Spinner {
    chars: Vec<char>,
    count: i32
}

impl Spinner {
    pub fn new() -> Self {
        Spinner {
            chars: vec!['|', '/', '-', '\\'],
            count: 0
        }
    }

    pub fn next_char(&mut self) -> char {
        self.count += 1;

        let output = self.chars[self.count as usize];
        let length = self.chars.len() as i32;

        if self.count == length - 1 {
            self.count = 0;
        }

        output
    }
}