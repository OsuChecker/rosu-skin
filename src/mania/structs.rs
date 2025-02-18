#[derive(Debug)]
pub struct ManiaConfig {
    pub keys: Keys,
    pub special_style: SpecialStyle,
    pub column_layout: ColumnLayout,
    pub positions: Positions,
    pub colours: Colours,
    pub images: Images,
    pub behavior: Behavior,
}

#[derive(Debug)]
pub struct Keys {
    pub count: u32,
    pub keys_under_notes: bool,
}

#[derive(Debug)]
pub struct SpecialStyle {
    pub style_type: u8, // 0, 1, or 2
    pub split_stages: bool,
    pub stage_separation: u32,
    pub separate_score: bool,
}

#[derive(Debug)]
pub struct ColumnLayout {
    pub column_start: u32,
    pub column_right: u32,
    pub column_width: Vec<u32>,
    pub column_spacing: Vec<u32>,
    pub column_line_width: Vec<u32>,
    pub barline_height: f32,
    pub lighting_width: LightingWidth,
    pub width_for_note_height_scale: Option<u32>,
}

#[derive(Debug)]
pub struct LightingWidth {
    pub n_width: Vec<u32>,
    pub l_width: Vec<u32>,
}

#[derive(Debug)]
pub struct Positions {
    pub hit_position: u32,
    pub light_position: u32,
    pub score_position: u32,
    pub combo_position: u32,
}

#[derive(Debug)]
pub struct Colours {
    pub columns: Vec<[u8; 4]>,
    pub lights: Vec<[u8; 4]>,
    pub column_line: [u8; 4],
    pub barline: [u8; 4],
    pub judgement_line: [u8; 3],
    pub key_warning: [u8; 3],
    pub hold: [u8; 4],
    pub break_colour: [u8; 3],
}

#[derive(Debug)]
pub struct Images {
    pub keys: KeyImages,
    pub notes: NoteImages,
    pub stage: StageImages,
    pub hits: HitImages,
}

#[derive(Debug)]
pub struct KeyImages {
    pub normal: Vec<String>,    // KeyImage#
    pub pressed: Vec<String>,   // KeyImage#D
}

#[derive(Debug)]
pub struct NoteImages {
    pub regular: Vec<String>,   // NoteImage#
    pub hold_head: Vec<String>, // NoteImage#H
    pub hold_body: Vec<String>, // NoteImage#L
    pub hold_tail: Vec<String>, // NoteImage#T
}

#[derive(Debug)]
pub struct StageImages {
    pub left: String,
    pub right: String,
    pub bottom: String,
    pub hint: String,
    pub light: String,
    pub lighting_n: String,
    pub lighting_l: String,
    pub warning_arrow: String,
}

#[derive(Debug)]
pub struct HitImages {
    pub hit_0: String,
    pub hit_50: String,
    pub hit_100: String,
    pub hit_200: String,
    pub hit_300: String,
    pub hit_300g: String,
}

#[derive(Debug)]
pub struct Behavior {
    pub judgement_line: bool,
    pub light_frame_per_second: u32,
    pub upside_down: bool,
    pub note_body_style: NoteBodyStyle,
    pub flip_config: FlipConfig,
    pub combo_burst_style: u8,
}

#[derive(Debug)]
pub struct NoteBodyStyle {
    pub global: u8,             // 0, 1, or 2
    pub per_column: Vec<u8>,
}

#[derive(Debug)]
pub struct FlipConfig {
    pub key_flip: bool,
    pub note_flip: bool,
    pub per_column_key_flip: Vec<bool>,
    pub per_column_note_flip: NoteFlipPerColumn,
}

#[derive(Debug)]
pub struct NoteFlipPerColumn {
    pub note: Vec<bool>,
    pub hold_head: Vec<bool>,
    pub hold_body: Vec<bool>,
    pub hold_tail: Vec<bool>,
}