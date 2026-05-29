#![no_std]
#![no_main]

mod game;

use game::state::player::Player;
use game::state::enemy::Enemy;
use game::state::map::{WORLD_MAP, MAP_SIZE};
use game::engine::display::GameDisplay;
use game::engine::audio::MusicPlayer;
use game::engine::led::update_leds;
use game::input::controls::Controls;
use game::logic::{handle_input, update_game_state, collect_active_enemies};

use core::cell::RefCell;
use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::spi::{self, Spi};
use embassy_rp::pwm::{Pwm, Config};
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

const DISPLAY_FREQ: u32 = 16_000_000;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let controls = Controls::new(
        Input::new(p.PIN_18, Pull::Up),
        Input::new(p.PIN_19, Pull::Up),
        Input::new(p.PIN_17, Pull::Up),
        Input::new(p.PIN_16, Pull::Up),
        Input::new(p.PIN_2, Pull::Up),
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
        .color_order(mipidsi::options::ColorOrder::Bgr)
        .init(&mut Delay)
        .unwrap();

    let mut display = GameDisplay::new(screen);
    let mut player = Player::new();
    let mut music = MusicPlayer::new(230);
    let mut pwm = Pwm::new_output_a(p.PWM_SLICE2, p.PIN_20, Config::default());

    let mut led_red = Output::new(p.PIN_3, Level::Low);
    let mut led_green = Output::new(p.PIN_5, Level::Low);
    let mut led_blue = Output::new(p.PIN_4, Level::Low);
    let mut frame_count: u32 = 0;

    let mut enemies = [const { None }; 20];
    let mut enemy_count = 0;
    
    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            if WORLD_MAP[x][y] == 5 && enemy_count < 20 {
                enemies[enemy_count] = Some(Enemy::new(x as f32 + 0.5, y as f32 + 0.5));
                enemy_count += 1;
            }
        }
    }

    loop {
        handle_input(&controls, &mut player, &mut enemies, enemy_count);
        update_game_state(&mut player, &mut music, &mut pwm, &mut enemies, enemy_count);
        
        let active_enemies = collect_active_enemies(&enemies, enemy_count);
        display.render(&player, &active_enemies).unwrap();

        update_leds(&player, &mut led_red, &mut led_green, &mut led_blue, frame_count);
        
        frame_count = frame_count.wrapping_add(1);
        embassy_time::Timer::after_millis(16).await;
    }
}
