pub struct General {
    pub metadata: SkinMetadata,
    pub cursor: CursorConfig,
    pub spinner: SpinnerConfig,
    pub gameplay: GameplayConfig,
}

pub struct SkinMetadata {
    pub name: String,
    pub author: String,
    pub version: String,
    pub animation_framerate: i32,
}

pub struct CursorConfig {
    pub centre: bool,
    pub expand: bool,
    pub rotate: bool,
    pub trail_rotate: bool,
}

pub struct SpinnerConfig {
    pub fade_playfield: bool,
    pub frequency_modulate: bool,
    pub no_blink: bool,
}

pub struct GameplayConfig {
    pub allow_slider_ball_tint: bool,
    pub combo_burst_random: bool,
    pub custom_combo_burst_sounds: Vec<u32>,
    pub hit_circle_overlay_above_number: bool,
    pub layered_hit_sounds: bool,
    pub slider_ball_flip: bool,
}

impl Default for General {
    fn default() -> Self {
        Self {
            metadata: SkinMetadata::default(),
            cursor: CursorConfig::default(),
            spinner: SpinnerConfig::default(),
            gameplay: GameplayConfig::default(),
        }
    }
}

impl Default for SkinMetadata {
    fn default() -> Self {
        Self {
            name: String::from("Unknown"),
            author: String::new(),
            version: String::from("latest"),
            animation_framerate: -1,
        }
    }
}

impl Default for CursorConfig {
    fn default() -> Self {
        Self {
            centre: true,
            expand: true,
            rotate: true,
            trail_rotate: true,
        }
    }
}

impl Default for SpinnerConfig {
    fn default() -> Self {
        Self {
            fade_playfield: false,
            frequency_modulate: true,
            no_blink: false,
        }
    }
}

impl Default for GameplayConfig {
    fn default() -> Self {
        Self {
            allow_slider_ball_tint: false,
            combo_burst_random: false,
            custom_combo_burst_sounds: Vec::new(),
            hit_circle_overlay_above_number: true,
            layered_hit_sounds: true,
            slider_ball_flip: true,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub struct Colours {
    pub combo: ComboColours,
    pub interface: InterfaceColours,
    pub gameplay: GameplayColours,
}

pub struct ComboColours {
    pub combo1: RgbColor,
    pub combo2: RgbColor,
    pub combo3: RgbColor,
    pub combo4: RgbColor,
    pub combo5: Option<RgbColor>,
    pub combo6: Option<RgbColor>,
    pub combo7: Option<RgbColor>,
    pub combo8: Option<RgbColor>,
}

pub struct InterfaceColours {
    pub menu_glow: RgbColor,
    pub song_select_active_text: RgbColor,
    pub song_select_inactive_text: RgbColor,
    pub input_overlay_text: RgbColor,
}

pub struct GameplayColours {
    pub slider_ball: RgbColor,
    pub slider_border: RgbColor,
    pub slider_track_override: Option<RgbColor>,
    pub spinner_background: RgbColor,
    pub star_break_additive: RgbColor,
}

impl Default for Colours {
    fn default() -> Self {
        Self {
            combo: ComboColours::default(),
            interface: InterfaceColours::default(),
            gameplay: GameplayColours::default(),
        }
    }
}

impl Default for ComboColours {
    fn default() -> Self {
        Self {
            combo1: RgbColor::new(255, 192, 0),    // Orange
            combo2: RgbColor::new(0, 202, 0),      // Vert
            combo3: RgbColor::new(18, 124, 255),   // Bleu
            combo4: RgbColor::new(242, 24, 57),    // Rouge
            combo5: None,
            combo6: None,
            combo7: None,
            combo8: None,
        }
    }
}

impl Default for InterfaceColours {
    fn default() -> Self {
        Self {
            menu_glow: RgbColor::new(0, 78, 155),
            song_select_active_text: RgbColor::new(0, 0, 0),
            song_select_inactive_text: RgbColor::new(255, 255, 255),
            input_overlay_text: RgbColor::new(0, 0, 0),
        }
    }
}

impl Default for GameplayColours {
    fn default() -> Self {
        Self {
            slider_ball: RgbColor::new(2, 170, 255),
            slider_border: RgbColor::new(255, 255, 255),
            slider_track_override: None,
            spinner_background: RgbColor::new(100, 100, 100),
            star_break_additive: RgbColor::new(255, 182, 193),
        }
    }
}

pub struct Fonts {
    pub hit_circle: FontConfig,
    pub score: FontConfig,
    pub combo: FontConfig,
}

pub struct FontConfig {
    pub prefix: String,
    pub overlap: i32,
}

impl Default for Fonts {
    fn default() -> Self {
        Self {
            hit_circle: FontConfig {
                prefix: String::from("default"),
                overlap: -2,
            },
            score: FontConfig {
                prefix: String::from("score"),
                overlap: 0,
            },
            combo: FontConfig {
                prefix: String::from("score"),
                overlap: 0,
            },
        }
    }
}

impl FontConfig {
    pub fn new(prefix: &str, overlap: i32) -> Self {
        Self {
            prefix: String::from(prefix),
            overlap,
        }
    }
}
pub struct CatchTheBeat {
    pub hyper_dash_colors: HyperDashColors,
}

pub struct HyperDashColors {
    pub main: RgbColor,
    pub fruit: Option<RgbColor>,
    pub after_image: Option<RgbColor>,
}

impl Default for CatchTheBeat {
    fn default() -> Self {
        Self {
            hyper_dash_colors: HyperDashColors::default(),
        }
    }
}

impl Default for HyperDashColors {
    fn default() -> Self {
        let default_color = RgbColor::new(255, 0, 0);
        Self {
            main: default_color,
            fruit: None,
            after_image: None,
        }
    }
}

impl HyperDashColors {
    pub fn get_fruit_color(&self) -> RgbColor {
        self.fruit.unwrap_or(self.main)
    }

    pub fn get_after_image_color(&self) -> RgbColor {
        self.after_image.unwrap_or(self.main)
    }
}

