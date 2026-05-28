use embassy_rp::gpio::Input;

pub struct Controls<'a> {
    forward: Input<'a>,
    backward: Input<'a>,
    turn_left: Input<'a>,
    turn_right: Input<'a>,
}

impl<'a> Controls<'a> {
    pub fn new(
        forward: Input<'a>,
        backward: Input<'a>,
        turn_left: Input<'a>,
        turn_right: Input<'a>,
    ) -> Self {
        Self { forward, backward, turn_left, turn_right }
    }

    pub fn forward(&self) -> bool { self.forward.is_low() }
    pub fn backward(&self) -> bool { self.backward.is_low() }
    pub fn turn_left(&self) -> bool { self.turn_left.is_low() }
    pub fn turn_right(&self) -> bool { self.turn_right.is_low() }
}