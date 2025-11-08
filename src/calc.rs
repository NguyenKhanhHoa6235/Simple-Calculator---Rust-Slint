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
    pub last_op: Option<Op>,  // Lưu toán tử cuối cùng
    pub error: bool,
}

impl CalculatorState {
    /// Nhập một chữ số (0–9)
    pub fn input_digit(&mut self, digit: u8) {
        if self.error {
            return;
        }
        if self.current == "0" {
            self.current.clear();
        }
        self.current.push(char::from(b'0' + digit));
    }

    /// Nhập dấu thập phân (.)
    pub fn input_decimal(&mut self) {
        if self.error {
            return;
        }
        if !self.current.contains('.') {
            if self.current.is_empty() {
                self.current.push('0');
            }
            self.current.push('.');
        }
    }

    /// Xóa ký tự cuối (Backspace)
    pub fn backspace(&mut self) {
        if self.error {
            return;
        }
        self.current.pop();
        if self.current.is_empty() {
            self.current.push('0');
        }
    }

    /// Đổi dấu số hiện tại (±)
    pub fn toggle_sign(&mut self) {
        if self.error || self.current.is_empty() {
            return;
        }
        if self.current.starts_with('-') {
            self.current.remove(0);
        } else if self.current != "0" {
            self.current.insert(0, '-');
        }
    }

    /// Chọn phép toán (+ - * /)
    pub fn set_op(&mut self, op: Op) {
        if self.error {
            return;
        }

        if self.current.is_empty() {
            // Nếu đã có toán tử cũ, thay toán tử mới
            if self.op.is_some() {
                self.op = Some(op);
            }
            return;
        }

        if let Ok(num) = self.current.parse::<f64>() {
            self.stored = num;
            self.current.clear();
            self.op = Some(op);
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
            // Phép toán bình thường
            if let Ok(right) = self.current.parse::<f64>() {
                let res = match op {
                    Op::Add => self.stored + right,
                    Op::Sub => self.stored - right,
                    Op::Mul => self.stored * right,
                    Op::Div => {
                        if right == 0.0 {
                            self.error = true;
                            f64::NAN
                        } else {
                            self.stored / right
                        }
                    }
                };
                self.current = if self.error { "Error".into() } else { res.to_string() };
                self.stored = res;
                self.last_operand = Some(right);
                self.last_op = Some(op);
                self.op = None;
            } else {
                self.error = true;
            }
        } else if let (Some(last_op), Some(last_operand)) = (self.last_op, self.last_operand) {
            // Lặp lại phép toán cuối cùng
            let res = match last_op {
                Op::Add => self.stored + last_operand,
                Op::Sub => self.stored - last_operand,
                Op::Mul => self.stored * last_operand,
                Op::Div => {
                    if last_operand == 0.0 {
                        self.error = true;
                        f64::NAN
                    } else {
                        self.stored / last_operand
                    }
                }
            };
            self.current = if self.error { "Error".into() } else { res.to_string() };
            self.stored = res;
            // last_op và last_operand giữ nguyên để tiếp tục lặp
        } else {
            self.error = true;
        }
    }

    /// Xóa toàn bộ trạng thái (C)
    pub fn clear(&mut self) {
        *self = CalculatorState::default();
    }
}
