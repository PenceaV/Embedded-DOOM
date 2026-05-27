use embedded_graphics::{
    prelude::*,
    primitives::{Rectangle, PrimitiveStyle, PrimitiveStyleBuilder},
    pixelcolor::Rgb565,
    draw_target::DrawTarget,
};
use crate::game::player::Player;

pub struct GameDisplay<T> {
    screen: T,
    player_style: PrimitiveStyle<Rgb565>,
    erase_style: PrimitiveStyle<Rgb565>,
}

impl<T, E> GameDisplay<T>
where
    T: DrawTarget<Color = Rgb565, Error = E>,
{
    pub fn new(mut screen: T) -> Self {
        let _ = screen.clear(Rgb565::BLACK);

        let player_style = PrimitiveStyleBuilder::new().fill_color(Rgb565::GREEN).build();
        let erase_style = PrimitiveStyleBuilder::new().fill_color(Rgb565::BLACK).build();

        Self {
            screen,
            player_style,
            erase_style,
        }
    }

    pub fn draw_player(&mut self, player: &Player) -> Result<(), E> {
        Rectangle::new(Point::new(player.last_x, player.last_y), Size::new(player.size, player.size))
            .into_styled(self.erase_style)
            .draw(&mut self.screen)?;

        Rectangle::new(Point::new(player.x, player.y), Size::new(player.size, player.size))
            .into_styled(self.player_style)
            .draw(&mut self.screen)?;

        Ok(())
    }
    
    pub fn draw_initial_player(&mut self, player: &Player) -> Result<(), E> {
        Rectangle::new(Point::new(player.x, player.y), Size::new(player.size, player.size))
            .into_styled(self.player_style)
            .draw(&mut self.screen)?;
        Ok(())
    }
}