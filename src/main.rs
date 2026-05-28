#![no_std]
#![no_main]

mod game;

use game::state::player::Player;
use game::state::enemy::Enemy;
use crate::game::engine::display::GameDisplay;
use crate::game::input::controls::Controls;
use crate::game::engine::raycaster::cast_ray;
use micromath::F32Ext;

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
const ROT_SPEED: f32 = 0.15;
const DISPLAY_FREQ: u32 = 16_000_000;

use game::state::map::{WORLD_MAP, MAP_SIZE};

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

    let mut enemies = [const { None }; 10];
    let mut enemy_count = 0;
    
    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            if WORLD_MAP[x][y] == 5 && enemy_count < 10 {
                enemies[enemy_count] = Some(Enemy::new(x as f32 + 0.5, y as f32 + 0.5));
                enemy_count += 1;
            }
        }
    }

    loop {
        if controls.forward() { player.move_forward(MOVE_SPEED); }
        if controls.backward() { player.move_backward(MOVE_SPEED); }
        if controls.turn_left() { player.rotate(ROT_SPEED); }
        if controls.turn_right() { player.rotate(-ROT_SPEED); }
        if controls.shoot() { 
            let was_shooting = player.is_shooting();
            player.shoot(); 
            
            if !was_shooting {
                let mut hit_idx: Option<usize> = None;
                let mut min_dist = 1000.0;
                
                let wall_hit = cast_ray(&player, 0.0);
                
                for i in 0..enemy_count {
                    if let Some(enemy) = &enemies[i] {
                        let rx = enemy.x - player.x;
                        let ry = enemy.y - player.y;
                        let dist = (rx * rx + ry * ry).sqrt();
                        
                        let inv_det = 1.0 / (player.plane_x * player.dir_y - player.dir_x * player.plane_y);
                        let transform_x = inv_det * (player.dir_y * rx - player.dir_x * ry);
                        let transform_y = inv_det * (-player.plane_y * rx + player.plane_x * ry);
                        
                        if transform_y > 0.1 && transform_y < wall_hit.perp_dist {
                            let angle_off = (transform_x / transform_y).abs();
                            if angle_off < 0.2 && dist < min_dist {
                                min_dist = dist;
                                hit_idx = Some(i);
                            }
                        }
                    }
                }
                
                if let Some(idx) = hit_idx {
                    if let Some(ref mut enemy) = enemies[idx] {
                        enemy.hp -= 1;
                        if enemy.hp <= 0 {
                            enemies[idx] = None;
                        }
                    }
                }
            }
        }

        player.update();
        for i in 0..enemy_count {
            if let Some(ref mut enemy) = enemies[i] {
                enemy.update(&player);
            }
        }

        let mut active_enemies = [Enemy::new(0.0, 0.0); 10];
        let mut active_count = 0;
        for i in 0..enemy_count {
            if let Some(enemy) = &enemies[i] {
                active_enemies[active_count] = *enemy;
                active_count += 1;
            }
        }

        display.render(&player, &active_enemies[..active_count]).unwrap();

        embassy_time::Timer::after_millis(16).await;
    }
}
