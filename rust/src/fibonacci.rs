use std::mem;

/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    Vec::new()
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    Fib::new().into_iter().take(5).collect()
}

/// Implement Fib as an iterator that can be collected into an iterator
#[derive(Debug)]
struct Fib {
    first: bool,
    prev: u8,
    curr: u8,
}

impl Fib {
    fn new() -> Self {
        Fib {
            first: true,
            prev: 0,
            curr: 1,
        }
    }
}

impl Iterator for Fib {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(1);
        }

        mem::swap(&mut self.curr, &mut self.prev);
        self.curr.checked_add(self.prev).map(|n| {
            self.curr = n;
            n
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_five_fib_numbers() {
        assert_eq!(vec![1, 1, 2, 3, 5], fibonacci())
    }
}
