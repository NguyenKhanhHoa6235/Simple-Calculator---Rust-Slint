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
    /// Nhập chữ số (0–9)
    pub fn input_digit(&mut self, digit: u8) {
        if self.error {
            return;
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
