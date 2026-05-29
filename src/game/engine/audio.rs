use embassy_rp::pwm::{Pwm, Config};

const NOTE_E2:  u32 = 82;
const NOTE_E3:  u32 = 164;
const NOTE_D3:  u32 = 146;
const NOTE_C3:  u32 = 130;
const NOTE_B2:  u32 = 123;
const NOTE_A2:  u32 = 110;
const NOTE_A3:  u32 = 220;
const NOTE_G3:  u32 = 196;
const NOTE_F3:  u32 = 174;
const NOTE_FS3: u32 = 185;
const REST:     u32 = 0;

pub const DOOM_THEME: &[(u32, u32)] = &[
    (NOTE_E2, 8), (NOTE_E2, 8), (NOTE_E3, 8), (NOTE_E2, 8), 
    (NOTE_E2, 8), (NOTE_D3, 8), (NOTE_E2, 8), (NOTE_E2, 8),
    (NOTE_C3, 8), (NOTE_E2, 8), (NOTE_E2, 8), (NOTE_B2, 8), 
    (NOTE_E2, 8), (NOTE_E2, 8), (NOTE_C3, 8), (NOTE_C3, 8),
    
    (NOTE_E2, 8), (NOTE_E2, 8), (NOTE_E3, 8), (NOTE_E2, 8), 
    (NOTE_E2, 8), (NOTE_D3, 8), (NOTE_E2, 8), (NOTE_E2, 8),
    (NOTE_C3, 8), (NOTE_E2, 8), (NOTE_E2, 8), (NOTE_B2, 8), 
    (NOTE_E2, 16), (REST, 4),

    (NOTE_A2, 8), (NOTE_A2, 8), (NOTE_A3, 8), (NOTE_A2, 8), 
    (NOTE_A2, 8), (NOTE_G3, 8), (NOTE_A2, 8), (NOTE_A2, 8),
    (NOTE_F3, 8), (NOTE_A2, 8), (NOTE_A2, 8), (NOTE_E3, 8), 
    (NOTE_A2, 8), (NOTE_A2, 8), (NOTE_F3, 8), (NOTE_FS3, 8),

    (NOTE_E2, 8), (NOTE_E2, 8), (NOTE_E3, 8), (NOTE_E2, 8), 
    (NOTE_E2, 8), (NOTE_D3, 8), (NOTE_E2, 8), (NOTE_E2, 8),
    (NOTE_C3, 8), (NOTE_E2, 8), (NOTE_E2, 8), (NOTE_B2, 8), 
    (NOTE_E2, 8), (NOTE_E2, 8), (NOTE_C3, 8), (NOTE_C3, 8),
    
    (NOTE_E2, 8), (NOTE_E2, 8), (NOTE_E3, 8), (NOTE_E2, 8), 
    (NOTE_E2, 8), (NOTE_D3, 8), (NOTE_E2, 8), (NOTE_E2, 8),
    (NOTE_C3, 8), (NOTE_E2, 8), (NOTE_E2, 8), (NOTE_B2, 8), 
    (NOTE_E2, 16), (REST, 4),

    (NOTE_E3, 8), (NOTE_D3, 8), (NOTE_B2, 8), (NOTE_C3, 8),
    (NOTE_E3, 8), (NOTE_D3, 8), (NOTE_B2, 8), (NOTE_C3, 8),
    (NOTE_E3, 8), (NOTE_D3, 8), (NOTE_B2, 8), (NOTE_C3, 8),
    (NOTE_E3, 16), (NOTE_D3, 16),
];

pub struct MusicPlayer {
    current_note: usize,
    ticks: u32,
    pub tempo_divider: u32, 
}

impl MusicPlayer {
    pub fn new(tempo_divider: u32) -> Self {
        Self { 
            current_note: 0, 
            ticks: 0,
            tempo_divider: tempo_divider.max(1), // zero-division
        }
    }

    pub fn update(&mut self, pwm: &mut Pwm) {
        let (_freq, duration) = DOOM_THEME[self.current_note];
        let target_duration = (duration / self.tempo_divider).max(1);

        if self.ticks >= target_duration {
            self.ticks = 0;
            
            self.current_note = (self.current_note + 1) % DOOM_THEME.len();
            let (new_freq, _) = DOOM_THEME[self.current_note];
            
            let mut cfg = Config::default();
            if new_freq > 0 {
                cfg.divider = 64.into();
                cfg.top = ((125_000_000 / 64) / new_freq) as u16;
                cfg.compare_a = cfg.top / 2;
            } else {
                cfg.top = 0;
                cfg.compare_a = 0;
            }
            pwm.set_config(&cfg);
        }
        self.ticks += 1;
    }
}