use std::cell::RefCell;
use std::rc::{Rc, Weak};

mod calc;
use calc::{CalculatorState, Op};

slint::include_modules!();

fn main() {
    let state: Rc<RefCell<CalculatorState>> = Rc::new(RefCell::new(CalculatorState::default()));
    let ui = CalculatorView::new().unwrap();
    let ui_weak = ui.as_weak();
    let state_weak = Rc::downgrade(&state);

    let invalidate_view = move |state_weak: Weak<RefCell<CalculatorState>>, ui_weak: slint::Weak<CalculatorView>| {
        if let (Some(state_rc), Some(ui)) = (state_weak.upgrade(), ui_weak.upgrade()) {
            let state = state_rc.borrow();
            ui.set_current_value(state.current.clone().into());
            ui.set_expression(state.expression.clone().into()); // cập nhật dòng trên
        }
    };

    {
        let state_weak = state_weak.clone();
        let ui_weak = ui_weak.clone();
        ui.on_input(move |digit: slint::SharedString| {
            if let Some(state_rc) = state_weak.upgrade() {
                let mut state = state_rc.borrow_mut();
                let s = digit.as_str();
                if s == "." {
                    state.input_decimal();
                } else if let Some(ch) = s.chars().next() {
                    if let Some(d) = ch.to_digit(10) {
                        state.input_digit(d as u8);
                    }
                }
            }
            invalidate_view(state_weak.clone(), ui_weak.clone());
        });
    }

    {
        let state_weak = state_weak.clone();
        let ui_weak = ui_weak.clone();
        ui.on_operation(move |op| {
            if let Some(state_rc) = state_weak.upgrade() {
                let mut state = state_rc.borrow_mut();
                match op {
                    CalculatorViewOp::Add => state.set_op(Op::Add),
                    CalculatorViewOp::Subtract => state.set_op(Op::Sub),
                    CalculatorViewOp::Multiply => state.set_op(Op::Mul),
                    CalculatorViewOp::Divide => state.set_op(Op::Div),
                    CalculatorViewOp::Evaluate => state.evaluate(),
                    CalculatorViewOp::Clear => state.clear(),
                    CalculatorViewOp::Backspace => state.backspace(),
                    CalculatorViewOp::ToggleSign => state.toggle_sign(),
                }
            }
            invalidate_view(state_weak.clone(), ui_weak.clone());
        });
    }

    ui.run().unwrap();
}
