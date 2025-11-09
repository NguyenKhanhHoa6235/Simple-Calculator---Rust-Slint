#[derive(Copy, Clone, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Default, Debug)]
pub struct CalculatorState {
    pub current: String,
    pub stored: f64,
    pub op: Option<Op>,
    pub last_operand: Option<f64>,
    pub last_op: Option<Op>, // để lặp lại phép toán
    pub expression: String,  // thêm dòng hiển thị biểu thức
    pub error: bool,
}

impl CalculatorState {
    const MAX_LEN: usize = 16; // Giới hạn độ dài đầu vào

    /// Nhập chữ số (0–9)
    pub fn input_digit(&mut self, digit: u8) {
        if self.error {
            return;
        }
        if self.current.len() >= Self::MAX_LEN {
            return; // bỏ qua nếu quá dài
        }
        if self.current == "0" {
            self.current.clear();
        }
        self.current.push(char::from(b'0' + digit));
        self.update_expression();
    }

    /// Nhập dấu thập phân
    pub fn input_decimal(&mut self) {
        if self.error {
            return;
        }
        if self.current.len() >= Self::MAX_LEN {
            return;
        }
        if !self.current.contains('.') {
            if self.current.is_empty() {
                self.current.push('0');
            }
            self.current.push('.');
            self.update_expression();
        }
    }

    /// Xóa ký tự cuối
    pub fn backspace(&mut self) {
        if self.error {
            return;
        }
        self.current.pop();
        if self.current.is_empty() {
            self.current.push('0');
        }
        self.update_expression();
    }

    /// Đổi dấu (±)
    pub fn toggle_sign(&mut self) {
        if self.error || self.current.is_empty() {
            return;
        }
        if self.current.starts_with('-') {
            self.current.remove(0);
        } else if self.current != "0" {
            self.current.insert(0, '-');
        }
        self.update_expression();
    }

    /// Cập nhật chuỗi biểu thức hiển thị
    fn update_expression(&mut self) {
        if let Some(op) = self.op {
            let op_str = match op {
                Op::Add => "+",
                Op::Sub => "-",
                Op::Mul => "×",
                Op::Div => "÷",
            };
            self.expression = format!("{} {} {}", self.stored, op_str, self.current);
        } else {
            self.expression = self.current.clone();
        }
    }

    /// Chọn phép toán
    pub fn set_op(&mut self, op: Op) {
        if self.error {
            return;
        }

        if self.current.is_empty() {
            if self.op.is_some() {
                self.op = Some(op);
            }
            return;
        }

        if let Ok(num) = self.current.parse::<f64>() {
            self.stored = num;
            self.current.clear();
            self.op = Some(op);
            self.update_expression();
        } else {
            self.error = true;
        }
    }

    /// Tính toán kết quả (=)
    pub fn evaluate(&mut self) {
        if self.error {
            return;
        }

        if let Some(op) = self.op {
            // Phép toán thông thường
            if let Ok(right) = self.current.parse::<f64>() {
                let res = self.apply_op(op, self.stored, right);
                if !self.error {
                    self.expression = format!(
                        "{} {} {} =",
                        self.stored,
                        Self::op_to_string(op),
                        right
                    );
                    self.current = res.to_string();
                    self.stored = res;
                    self.last_operand = Some(right);
                    self.last_op = Some(op);
                    self.op = None;
                }
            } else {
                self.error = true;
            }
        } else if let (Some(last_op), Some(last_operand)) = (self.last_op, self.last_operand) {
            // Lặp lại phép toán cuối
            let res = self.apply_op(last_op, self.stored, last_operand);
            if !self.error {
                self.expression = format!(
                    "{} {} {} =",
                    self.stored,
                    Self::op_to_string(last_op),
                    last_operand
                );
                self.current = res.to_string();
                self.stored = res;
            }
        }
    }

    fn op_to_string(op: Op) -> &'static str {
        match op {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "×",
            Op::Div => "÷",
        }
    }

    fn apply_op(&mut self, op: Op, left: f64, right: f64) -> f64 {
        match op {
            Op::Add => left + right,
            Op::Sub => left - right,
            Op::Mul => left * right,
            Op::Div => {
                if right == 0.0 {
                    self.error = true;
                    f64::NAN
                } else {
                    left / right
                }
            }
        }
    }

    /// Clear toàn bộ
    pub fn clear(&mut self) {
        *self = CalculatorState::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state() -> CalculatorState {
        CalculatorState::default()
    }

    #[test]
    fn test_addition_basic() {
        let mut calc = make_state();
        calc.input_digit(1);
        calc.input_digit(2);
        calc.set_op(Op::Add);
        calc.input_digit(3);
        calc.evaluate();
        assert_eq!(calc.current, "15");
    }

    #[test]
    fn test_multiply_by_zero() {
        let mut calc = make_state();
        calc.input_digit(5);
        calc.set_op(Op::Mul);
        calc.input_digit(0);
        calc.evaluate();
        assert_eq!(calc.current, "0");
    }

    #[test]
    fn test_divide_by_zero_error() {
        let mut calc = make_state();
        calc.input_digit(9);
        calc.set_op(Op::Div);
        calc.input_digit(0);
        calc.evaluate();
        assert!(calc.error, "Expected divide-by-zero to set error flag");
    }

    #[test]
    fn test_double_decimal_ignored() {
        let mut calc = make_state();
        calc.input_digit(1);
        calc.input_decimal();
        calc.input_decimal(); // should be ignored
        calc.input_digit(5);
        calc.set_op(Op::Add);
        calc.input_digit(2);
        calc.evaluate();
        assert_eq!(calc.current, "3.5");
    }

    #[test]
    fn test_repeat_evaluate() {
        let mut calc = make_state();
        calc.input_digit(5);
        calc.set_op(Op::Add);
        calc.input_digit(2);
        calc.evaluate(); // 7
        calc.evaluate(); // 7 + 2 again = 9
        assert_eq!(calc.current, "9");
    }

    #[test]
    fn test_backspace_behavior() {
        let mut calc = make_state();
        calc.input_digit(1);
        calc.input_digit(0);
        calc.backspace();
        calc.backspace();
        assert_eq!(calc.current, "0");
    }

    #[test]
    fn test_too_long_input_truncated() {
        let mut calc = make_state();
        for _ in 0..30 {
            calc.input_digit(9);
        }
        assert!(calc.current.len() <= CalculatorState::MAX_LEN);
        assert!(!calc.error);
    }
}
