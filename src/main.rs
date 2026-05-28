#![no_std]
#![no_main]

mod game;

use game::state::player::Player;
use crate::game::engine::display::GameDisplay;
use crate::game::input::controls::Controls;

use core::cell::RefCell;
use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::spi::{self, Spi};
use embassy_sync::blocking_mutex::{Mutex, raw::NoopRawMutex};
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice;
use embassy_time::Delay;
use display_interface_spi::SPIInterface;
use mipidsi::models::ILI9341Rgb565;

use panic_probe as _;
use defmt_rtt as _;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

const MOVE_SPEED: f32 = 0.35;
const ROT_SPEED: f32 = 0.08;
const DISPLAY_FREQ: u32 = 16_000_000;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let controls = Controls::new(
        Input::new(p.PIN_18, Pull::Up),
        Input::new(p.PIN_19, Pull::Up),
        Input::new(p.PIN_16, Pull::Up),
        Input::new(p.PIN_17, Pull::Up),
    );

    let mut spi_cfg = spi::Config::default();
    spi_cfg.frequency = DISPLAY_FREQ;
    spi_cfg.phase = spi::Phase::CaptureOnSecondTransition;
    spi_cfg.polarity = spi::Polarity::IdleHigh;

    let spi = Spi::new_blocking_txonly(p.SPI1, p.PIN_14, p.PIN_15, spi_cfg);
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
    let spi_dev = SpiDevice::new(&spi_bus, Output::new(p.PIN_13, Level::High));
    let di = SPIInterface::new(spi_dev, Output::new(p.PIN_11, Level::Low));

    let screen = mipidsi::Builder::new(ILI9341Rgb565, di)
        .reset_pin(Output::new(p.PIN_12, Level::Low))
        .init(&mut Delay)
        .unwrap();

    let mut display = GameDisplay::new(screen);
    let mut player = Player::new();

    loop {
        if controls.forward() { player.move_forward(MOVE_SPEED); }
        if controls.backward() { player.move_backward(MOVE_SPEED); }
        if controls.turn_left() { player.rotate(ROT_SPEED); }
        if controls.turn_right() { player.rotate(-ROT_SPEED); }

        display.render(&player).unwrap();

        embassy_time::Timer::after_millis(16).await;
    }
}