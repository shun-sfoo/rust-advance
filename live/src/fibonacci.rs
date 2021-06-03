pub struct Fibonacci {
    a: u64,
    b: u64,
    cur: u8,
    total: u8,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.total {
            return None;
        }

        if self.a == 0 {
            self.a = 1;
            self.b = 1;
        } else {
            let c = self.a + self.b;
            self.a = self.b;
            self.b = c;
        }

        self.cur += 1;
        Some(self.a)
    }
}

impl Fibonacci {
    pub fn new(total: u8) -> Self {
        Self {
            a: 0,
            b: 0,
            cur: 0,
            total,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        let fib = Fibonacci::new(10);
        for item in fib {
            println!("item: {}", item);
        }
    }
}
