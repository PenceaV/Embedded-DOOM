use embassy_rp::gpio::Input;

pub struct Controls<'a> {
    pub left_btn: Input<'a>,
    pub right_btn: Input<'a>,
    pub up_btn: Input<'a>,
    pub down_btn: Input<'a>,
}

impl<'a> Controls<'a> {
    pub fn new(left: Input<'a>, right: Input<'a>, up: Input<'a>, down: Input<'a>) -> Self {
        Self {
            left_btn: left,
            right_btn: right,
            up_btn: up,
            down_btn: down,
        }
    }

    pub fn is_left_pressed(&self) -> bool {
        self.left_btn.is_low()
    }

    pub fn is_right_pressed(&self) -> bool {
        self.right_btn.is_low()
    }

    pub fn is_up_pressed(&self) -> bool {
        self.up_btn.is_low()
    }

    pub fn is_down_pressed(&self) -> bool {
        self.down_btn.is_low()
    }
}