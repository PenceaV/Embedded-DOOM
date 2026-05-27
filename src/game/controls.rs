use embassy_rp::gpio::Input;

pub struct Controls<'a> {
    pub left_btn: Input<'a>,
    pub right_btn: Input<'a>,
    pub forward_btn: Input<'a>,
    pub backward_btn: Input<'a>,
}

impl<'a> Controls<'a> {
    pub fn new(left: Input<'a>, right: Input<'a>, forward: Input<'a>, backward: Input<'a>) -> Self {
        Self {
            left_btn: left,
            right_btn: right,
            forward_btn: forward,
            backward_btn: backward,
        }
    }

    pub fn is_turn_left_pressed(&self) -> bool { self.left_btn.is_low() }
    pub fn is_turn_right_pressed(&self) -> bool { self.right_btn.is_low() }
    pub fn is_forward_pressed(&self) -> bool { self.forward_btn.is_low() }
    pub fn is_backward_pressed(&self) -> bool { self.backward_btn.is_low() }
}