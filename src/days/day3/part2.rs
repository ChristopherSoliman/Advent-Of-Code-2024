use std::iter::Peekable;

pub struct Program<I: Iterator<Item = char>> {
    data: Peekable<I>,
    do_command: bool,
}

impl<I: Iterator<Item = char>> Program<I> {
    pub fn new(data: I) -> Program<I> {
        Program {
            data: data.peekable(),
            do_command: true,
        }
    }

    pub fn parse(&mut self) -> i32 {
        let mut sum: i32 = 0;
        while self.find_next_mul() {
            if let Some(p) = self.get_product() {
                sum += p;
            }
        }
        sum
    }

    fn get_product(&mut self) -> Option<i32> {
        let a: i32;
        let b: i32;

        if self.data.next()? != '(' {
            return None;
        }
        match self.data.peek() {
            Some('0'..='9') => {
                if let Some(n) = self.parse_number() {
                    a = n.parse().expect("Can't parse number");
                } else {
                    return None;
                }
            }
            Some(_) => return None,
            None => return None,
        }
        if self.data.next()? != ',' {
            return None;
        }
        match self.data.peek() {
            Some('0'..='9') => {
                if let Some(n) = self.parse_number() {
                    b = n.parse().expect("Can't parse number");
                } else {
                    return None;
                }
            }
            Some(_) => return None,
            None => return None,
        }
        if self.data.next()? != ')' {
            return None;
        }

        Some(a * b)
    }

    fn parse_number(&mut self) -> Option<String> {
        let mut num = String::new();
        num += &self.data.next().unwrap().to_string();
        while let Some(c) = self.data.peek() {
            match c {
                &('0'..='9') => {
                    if let Some(n) = self.parse_number() {
                        num += &n;
                    } else {
                        return None;
                    }
                }
                _ => return Some(num),
            }
        }
        None
    }

    fn find_next_mul(&mut self) -> bool {
        while let Some(c) = self.data.next() {
            match c {
                'm' => {
                    if self.data.next() == Some('u') && self.data.next() == Some('l') {
                        return true;
                    }
                }
                'd' => {
                    if self.data.next() == Some('o')
                        && self.data.next() == Some('n')
                        && self.data.next() == Some('\'')
                        && self.data.next() == Some('t')
                    {
                        if !self.find_next_do() {
                            return false;
                        }
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn find_next_do(&mut self) -> bool {
        while let Some(c) = self.data.next() {
            match c {
                'd' => {
                    if self.data.next() == Some('o')
                        && self.data.next() == Some('(')
                        && self.data.next() == Some(')')
                    {
                        self.do_command = true;
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
}
