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

use panic_probe as _;
use defmt_rtt as _;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

const DISPLAY_FREQ: u32 = 16_000_000;

const WORLD_MAP: [[i32; 24]; 24] = [
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
    [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
    [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,4,4,0,1],
    [1,4,0,0,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,4,0,1],
    [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,3,0,3,0,1],
    [1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,4,0,0,0,3,0,1],
    [1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,4,3,3,3,0,1],
    [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let controls = Controls::new(
        Input::new(p.PIN_17, Pull::Up), 
        Input::new(p.PIN_16, Pull::Up), 
        Input::new(p.PIN_18, Pull::Up), 
        Input::new(p.PIN_19, Pull::Up), 
    );

    let screen_cs = Output::new(p.PIN_13, Level::High);
    let screen_reset = Output::new(p.PIN_12, Level::Low);
    let screen_dc = Output::new(p.PIN_11, Level::Low);

    let mut display_config = spi::Config::default();
    display_config.frequency = DISPLAY_FREQ;
    display_config.phase = spi::Phase::CaptureOnSecondTransition;
    display_config.polarity = spi::Polarity::IdleHigh;

    let spi = Spi::new_blocking_txonly(p.SPI1, p.PIN_14, p.PIN_15, display_config);
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
    let display_spi = SpiDevice::new(&spi_bus, screen_cs);
    let di = SPIInterface::new(display_spi, screen_dc);

    let screen = mipidsi::Builder::new(ILI9341Rgb565, di)
        .reset_pin(screen_reset)
        .init(&mut Delay)
        .unwrap();

    let mut display = GameDisplay::new(screen);
    let mut player = Player::new();

    let move_speed: f32 = 0.35;
    let rot_speed: f32 = 0.08; 

    loop {
        if controls.is_forward_pressed() {
            player.move_forward(move_speed, &WORLD_MAP);
        }
        if controls.is_backward_pressed() {
            player.move_backward(move_speed, &WORLD_MAP);
        }
        if controls.is_turn_left_pressed() {
            player.rotate(rot_speed);
        }
        if controls.is_turn_right_pressed() {
            player.rotate(-rot_speed);
        }

        display.render_scene(&player, &WORLD_MAP).unwrap();

        embassy_time::Timer::after_millis(16).await;
    }
}