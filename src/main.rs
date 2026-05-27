#![no_std]
#![no_main]

mod game;

use game::{Player, GameDisplay, Controls};

use core::cell::RefCell;
use embassy_sync::blocking_mutex::{Mutex, raw::NoopRawMutex};
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice;
use embassy_time::Delay;
use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::spi;
use embassy_rp::spi::Spi;
use display_interface_spi::SPIInterface;
use mipidsi::models::ILI9341Rgb565;
use mipidsi::options::{Orientation, Rotation};

use embedded_graphics::pixelcolor::Rgb565;

use panic_probe as _;
use defmt_rtt as _;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

const DISPLAY_FREQ: u32 = 16_000_000;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let screen_cs = Output::new(p.PIN_13, Level::High);
    let screen_reset = Output::new(p.PIN_12, Level::Low);
    let screen_dc = Output::new(p.PIN_11, Level::Low);
    let mosi = p.PIN_15;
    let clk = p.PIN_14;

    let controls = Controls::new(
        Input::new(p.PIN_17, Pull::Up),
        Input::new(p.PIN_16, Pull::Up), 
        Input::new(p.PIN_18, Pull::Up), 
        Input::new(p.PIN_19, Pull::Up), 
    );

    let mut display_config = spi::Config::default();
    display_config.frequency = DISPLAY_FREQ;
    display_config.phase = spi::Phase::CaptureOnSecondTransition;
    display_config.polarity = spi::Polarity::IdleHigh;

    let spi = Spi::new_blocking_txonly(p.SPI1, clk, mosi, display_config);
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
    let display_spi = SpiDevice::new(&spi_bus, screen_cs);
    let di = SPIInterface::new(display_spi, screen_dc);

    let screen = mipidsi::Builder::new(ILI9341Rgb565, di)
        .reset_pin(screen_reset)
        .orientation(Orientation::new().rotate(Rotation::Deg270).flip_horizontal())
        .init(&mut Delay)
        .unwrap();

    let mut display = GameDisplay::new(screen); // Fixed: changed from raw_screen to screen
    let mut player = Player::new(50, 320, 240);

    display.draw_initial_player(&player).unwrap();

    loop {
        if controls.is_left_pressed() { player.move_left(5); }
        if controls.is_right_pressed() { player.move_right(5); }
        if controls.is_up_pressed() { player.move_up(5); }
        if controls.is_down_pressed() { player.move_down(5); }

        if player.has_moved() {
            display.draw_player(&player).unwrap();
            player.update_last_position();
        }

        embassy_time::Timer::after_millis(16).await;
    }
}