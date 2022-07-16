#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

impl CalculatorInput {
    fn eval(&self, l: i32, r: i32) -> i32 {
        match self {
            CalculatorInput::Add => l + r,
            CalculatorInput::Subtract => l - r,
            CalculatorInput::Multiply => l * r,
            CalculatorInput::Divide => l / r,
            _ => panic!("should not happen"),
        }
    }
}

pub struct Rpn {
    stack: Vec<i32>,
}

impl Rpn {
    fn new() -> Self {
        Rpn { stack: Vec::new() }
    }

    fn step(&mut self, op: &CalculatorInput) -> Result<i32, &str> {
        if let CalculatorInput::Value(v) = op {
            self.stack.push(*v);
            return Ok(*v);
        }

        if self.stack.len() >= 2 {
            let r = self.stack.pop().unwrap();
            let l = self.stack.pop().unwrap();
            let res = op.eval(l, r);
            self.stack.push(res);
            Ok(res)
        } else {
            Err("stack does not have 2 elements left")
        }
    }

    fn result(&self) -> Result<i32, String> {
        match self.stack.len() {
            1 => Ok(self.stack[0]),
            _ => Err(format!(
                "unexpected number of args on stack: {}",
                self.stack.len()
            )),
        }
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut rpn = Rpn::new();

    for i in inputs {
        if rpn.step(i).is_err() {
            return None;
        }
    }
    rpn.result().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calculator_input(s: &str) -> Vec<CalculatorInput> {
        s.split_whitespace()
            .map(|s| match s {
                "+" => CalculatorInput::Add,
                "-" => CalculatorInput::Subtract,
                "*" => CalculatorInput::Multiply,
                "/" => CalculatorInput::Divide,
                n => CalculatorInput::Value(n.parse().unwrap()),
            })
            .collect()
    }

    #[test]
    fn test_simple_subtraction() {
        let input = calculator_input("7 11 -");
        assert_eq!(evaluate(&input), Some(-4));
    }

    #[test]
    fn test_simple_division() {
        let input = calculator_input("57 19 /");
        assert_eq!(evaluate(&input), Some(3));
    }

    #[test]
    fn test_complex_operation() {
        let input = calculator_input("4 8 + 7 5 - /");
        assert_eq!(evaluate(&input), Some(6));
    }
}
