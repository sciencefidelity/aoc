use bytes::Buf;
use std::{fs, io::Cursor, path::Path};

fn mull_it_over(path: &str, conditionals: bool) -> i32 {
    let path = Path::new(&path);
    let file = fs::read(path).unwrap();
    let contents = Cursor::new(file);
    let mut scanner = Scanner::new(contents, conditionals);
    scanner.scan_tokens();
    scanner.product
}

pub struct Scanner {
    src: Cursor<Vec<u8>>,
    enabled: bool,
    conditionals: bool,
    start: usize,
    current: usize,
    product: i32,
}

impl Scanner {
    pub const fn new(src: Cursor<Vec<u8>>, conditionals: bool) -> Self {
        Self {
            src,
            enabled: true,
            conditionals,
            start: 0,
            current: 0,
            product: 0,
        }
    }

    pub fn scan_tokens(&mut self) {
        while self.src.has_remaining() {
            self.start = self.current;
            self.scan_token();
        }
    }

    pub fn scan_token(&mut self) {
        match self.pop() {
            b'm' => self.match_pattern(),
            b'd' if self.conditionals => self.match_dos(),
            _ => {}
        }
    }

    fn match_pattern(&mut self) {
        let (start_pattern, end_pattern) = (b"mul(", b')');
        if !self.src.has_remaining() {
            return;
        }
        for b in start_pattern.iter().skip(1) {
            if self.pop() != *b {
                return;
            }
        }
        self.start = self.current;
        let left_side = self.number();
        if self.pop() != b',' {
            return;
        }
        self.start = self.current;
        let right_side = self.number();
        if self.pop() != end_pattern {
            return;
        }
        if self.enabled {
            self.product += left_side * right_side;
        }
    }

    fn match_dos(&mut self) {
        let disable_pattern = b"don't()";
        if self.pop() != b'o' {
            return;
        }
        if self.peek() == b'(' {
            self.pop();
            if self.pop() == b')' {
                self.enabled = true;
            }
        } else if self.pop() == b'n' {
            for b in disable_pattern.iter().skip(3) {
                if self.pop() != *b {
                    return;
                }
            }
            self.enabled = false;
        } else {
            return;
        }
    }

    fn number(&mut self) -> i32 {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        let text = self.string_from_bytes(self.start, self.current);
        text.parse().unwrap()
    }

    fn advance(&mut self) {
        self.current += 1;
        self.src.advance(1);
    }

    fn peek(&self) -> u8 {
        if self.src.has_remaining() {
            self.src.chunk()[0]
        } else {
            b'\0'
        }
    }

    fn pop(&mut self) -> u8 {
        self.current += 1;
        self.src.get_u8()
    }

    fn string_from_bytes(&self, start: usize, end: usize) -> String {
        let bytes = &self.src.get_ref()[start..end];
        String::from_utf8(bytes.into()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mull_it_over_part_1_a() {
        assert_eq!(mull_it_over("data/day_03_a.txt", false), 161);
    }

    #[test]
    fn test_mull_it_over_part_1_b() {
        assert_eq!(mull_it_over("data/day_03_b.txt", false), 153_469_856);
    }

    #[test]
    fn test_mull_it_over_part_2_a() {
        assert_eq!(mull_it_over("data/day_03_c.txt", true), 48);
    }

    #[test]
    fn test_mull_it_over_part_2_b() {
        assert_eq!(mull_it_over("data/day_03_b.txt", true), 77_055_967);
    }
}
