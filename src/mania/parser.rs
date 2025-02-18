use ini::{Ini, ParseError, Properties};

use crate::structs::{
    Behavior, Colours, ColumnLayout, FlipConfig, HitImages, Images, KeyImages, Keys, LightingWidth,
    ManiaConfig, NoteBodyStyle, NoteFlipPerColumn, NoteImages, Positions, SpecialStyle,
    StageImages,
};
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::path::PathBuf;

pub fn open_ini(path: &str) -> Result<Ini, Box<dyn Error>> {
    let content = std::fs::read_to_string(path)?;
    let escaped_content = content
        .lines()
        .filter(|line| !line.trim().starts_with("//")) // Filtrer les lignes commençant par //
        .collect::<Vec<&str>>()
        .join("\n")
        .replace('\\', "\\\\");

    Ok(Ini::load_from_str(&escaped_content)?)
}

pub fn parse_color(color_str: &str) -> Option<[u8; 4]> {
    let parts: Vec<&str> = color_str.split(',').collect();
    if parts.len() >= 3 {
        let r = parts[0].trim().parse().ok()?;
        let g = parts[1].trim().parse().ok()?;
        let b = parts[2].trim().parse().ok()?;
        let a = parts
            .get(3)
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(255);
        Some([r, g, b, a])
    } else {
        None
    }
}

pub fn parse_color_rgb(color_str: &str) -> Option<[u8; 3]> {
    let parts: Vec<&str> = color_str.split(',').collect();
    if parts.len() >= 3 {
        let r = parts[0].trim().parse().ok()?;
        let g = parts[1].trim().parse().ok()?;
        let b = parts[2].trim().parse().ok()?;
        Some([r, g, b])
    } else {
        None
    }
}

pub fn parse_keys(props: &Properties, key_count: u32) -> Keys {
    Keys {
        count: key_count,
        keys_under_notes: props
            .get("KeysUnderNotes")
            .and_then(|v| v.parse::<u32>().ok())
            .map(|v| v != 0)
            .unwrap_or(false),
    }
}

pub fn parse_special_style(props: &Properties) -> SpecialStyle {
    SpecialStyle {
        style_type: props
            .get("SpecialStyle")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0),
        split_stages: props
            .get("SplitStages")
            .and_then(|v| v.parse::<u32>().ok())
            .map(|v| v != 0)
            .unwrap_or(false),
        stage_separation: props
            .get("StageSeparation")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0),
        separate_score: props
            .get("SeparateScore")
            .and_then(|v| v.parse::<u32>().ok())
            .map(|v| v != 0)
            .unwrap_or(false),
    }
}

pub fn parse_column_layout(props: &Properties, key_count: u32) -> ColumnLayout {
    ColumnLayout {
        column_start: props
            .get("ColumnStart")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        column_right: props
            .get("ColumnRight")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        column_width: parse_comma_separated_numbers(props.get("ColumnWidth")),
        column_spacing: parse_comma_separated_numbers(props.get("ColumnSpacing")),
        column_line_width: parse_comma_separated_numbers(props.get("ColumnLineWidth")),
        barline_height: props
            .get("BarlineHeight")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1.0),
        lighting_width: parse_lighting_width(props),
        width_for_note_height_scale: props
            .get("WidthForNoteHeightScale")
            .and_then(|s| s.parse().ok()),
    }
}

pub fn parse_lighting_width(props: &Properties) -> LightingWidth {
    LightingWidth {
        n_width: parse_comma_separated_numbers(props.get("LightingNWidth")),
        l_width: parse_comma_separated_numbers(props.get("LightingLWidth")),
    }
}

pub fn parse_positions(props: &Properties) -> Positions {
    Positions {
        hit_position: props
            .get("HitPosition")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        light_position: props
            .get("LightPosition")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        score_position: props
            .get("ScorePosition")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        combo_position: props
            .get("ComboPosition")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
    }
}

pub fn parse_colours(props: &Properties, key_count: u32) -> Colours {
    Colours {
        columns: parse_numbered_colors(props, "Colour", key_count),
        lights: parse_numbered_colors(props, "ColourLight", key_count),
        column_line: props
            .get("ColourColumnLine")
            .and_then(parse_color)
            .unwrap_or([255, 255, 255, 255]),
        barline: props
            .get("ColourBarline")
            .and_then(parse_color)
            .unwrap_or([255, 255, 255, 255]),
        judgement_line: props
            .get("ColourJudgementLine")
            .and_then(parse_color_rgb)
            .unwrap_or([255, 255, 255]),
        key_warning: props
            .get("ColourKeyWarning")
            .and_then(parse_color_rgb)
            .unwrap_or([255, 0, 0]),
        hold: props
            .get("ColourHold")
            .and_then(parse_color)
            .unwrap_or([255, 230, 0, 255]),
        break_colour: props
            .get("ColourBreak")
            .and_then(parse_color_rgb)
            .unwrap_or([255, 0, 0]),
    }
}

pub fn parse_images(props: &Properties, key_count: u32) -> Images {
    Images {
        keys: parse_key_images(props, key_count),
        notes: parse_note_images(props, key_count),
        stage: parse_stage_images(props),
        hits: parse_hit_images(props),
    }
}

pub fn parse_key_images(props: &Properties, key_count: u32) -> KeyImages {
    KeyImages {
        normal: parse_numbered_strings(props, "KeyImage", key_count,Some("")),
        pressed: parse_numbered_strings(props, "KeyImage", key_count, Some("D")),
    }
}

pub fn parse_note_images(props: &Properties, key_count: u32) -> NoteImages {
    NoteImages {
        regular: parse_numbered_strings(props, "NoteImage", key_count,Some("")),
        hold_head: parse_numbered_strings(props, "NoteImage", key_count, Some("H")),
        hold_body: parse_numbered_strings(props, "NoteImage", key_count, Some("L")),
        hold_tail: parse_numbered_strings(props, "NoteImage", key_count, Some("T")),
    }
}

pub fn parse_stage_images(props: &Properties) -> StageImages {
    StageImages {
        left: props.get("StageLeft").map(String::from).unwrap_or_default(),
        right: props
            .get("StageRight")
            .map(String::from)
            .unwrap_or_default(),
        bottom: props
            .get("StageBottom")
            .map(String::from)
            .unwrap_or_default(),
        hint: props.get("StageHint").map(String::from).unwrap_or_default(),
        light: props
            .get("StageLight")
            .map(String::from)
            .unwrap_or_default(),
        lighting_n: props
            .get("StageLightingN")
            .map(String::from)
            .unwrap_or_default(),
        lighting_l: props
            .get("StageLightingL")
            .map(String::from)
            .unwrap_or_default(),
        warning_arrow: props
            .get("WarningArrow")
            .map(String::from)
            .unwrap_or_default(),
    }
}

pub fn parse_hit_images(props: &Properties) -> HitImages {
    HitImages {
        hit_0: props.get("Hit0").map(String::from).unwrap_or_default(),
        hit_50: props.get("Hit50").map(String::from).unwrap_or_default(),
        hit_100: props.get("Hit100").map(String::from).unwrap_or_default(),
        hit_200: props.get("Hit200").map(String::from).unwrap_or_default(),
        hit_300: props.get("Hit300").map(String::from).unwrap_or_default(),
        hit_300g: props.get("Hit300g").map(String::from).unwrap_or_default(),
    }
}

pub fn parse_behavior(props: &Properties, key_count: u32) -> Behavior {
    Behavior {
        judgement_line: props
            .get("JudgementLine")
            .and_then(|v| v.parse::<u32>().ok())
            .map(|v| v != 0)
            .unwrap_or(false),
        light_frame_per_second: props
            .get("LightFramePerSecond")
            .and_then(|v| v.parse().ok())
            .unwrap_or(24),
        upside_down: props
            .get("UpsideDown")
            .and_then(|v| v.parse::<u32>().ok())
            .map(|v| v != 0)
            .unwrap_or(false),
        note_body_style: parse_note_body_style(props, key_count),
        flip_config: parse_flip_config(props, key_count),
        combo_burst_style: props
            .get("ComboBurstStyle")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0),
    }
}

pub fn parse_note_body_style(props: &Properties, key_count: u32) -> NoteBodyStyle {
    NoteBodyStyle {
        global: props
            .get("NoteBodyStyle")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0),
        per_column: parse_numbered_values(props, "NoteBodyStyle", key_count, 0),
    }
}

pub fn parse_flip_config(props: &Properties, key_count: u32) -> FlipConfig {
    FlipConfig {
        key_flip: props
            .get("KeyFlip")
            .and_then(|v| v.parse::<u32>().ok())
            .map(|v| v != 0)
            .unwrap_or(false),
        note_flip: props
            .get("NoteFlip")
            .and_then(|v| v.parse::<u32>().ok())
            .map(|v| v != 0)
            .unwrap_or(false),
        per_column_key_flip: parse_numbered_bools(props, "KeyFlip", key_count),
        per_column_note_flip: parse_note_flip_per_column(props, key_count),
    }
}

pub fn parse_note_flip_per_column(props: &Properties, key_count: u32) -> NoteFlipPerColumn {
    NoteFlipPerColumn {
        note: parse_numbered_bools(props, "NoteFlip", key_count),
        hold_head: parse_numbered_bools(props, "NoteFlipH", key_count),
        hold_body: parse_numbered_bools(props, "NoteFlipL", key_count),
        hold_tail: parse_numbered_bools(props, "NoteFlipT", key_count),
    }
}

pub fn parse_comma_separated_numbers(value: Option<&str>) -> Vec<u32> {
    value
        .map(|s| s.split(',').filter_map(|n| n.trim().parse().ok()).collect())
        .unwrap_or_default()
}

pub fn parse_numbered_strings(
    props: &Properties,
    prefix: &str,
    count: u32,
    suffix: Option<&str>,
) -> Vec<String> {
    (0..count)
        .map(|i| {
            let key = if let Some(suffix) = suffix {
                format!("{}{}{}", prefix, i, suffix)
            } else {
                format!("{}{}", prefix, i)
            };
            props.get(&key).map(String::from).unwrap_or_default()
        })
        .collect()
}

pub fn parse_numbered_colors(props: &Properties, prefix: &str, count: u32) -> Vec<[u8; 4]> {
    (1..=count)
        .filter_map(|i| props.get(&format!("{}{}", prefix, i)).and_then(parse_color))
        .collect()
}

pub fn parse_numbered_values<T: std::str::FromStr + Clone>(
    props: &Properties,
    prefix: &str,
    count: u32,
    default: T,
) -> Vec<T> {
    (0..count)
        .map(|i| {
            props
                .get(&format!("{}{}", prefix, i))
                .and_then(|v| v.parse().ok())
                .unwrap_or_else(|| default.clone())
        })
        .collect()
}


pub fn parse_numbered_bools(props: &Properties, prefix: &str, count: u32) -> Vec<bool> {
    parse_numbered_values(props, prefix, count, false)
}

pub fn read_mania_config(ini: &Ini) -> Vec<ManiaConfig> {
    let mut configs = Vec::new();

    for section in ini.sections() {
        if section.as_deref() == Some("Mania") {
            if let Some(props) = ini.section(section) {
                if let Some(keys_str) = props.get("Keys") {
                    if let Ok(key_count) = keys_str.parse::<u32>() {
                        configs.push(ManiaConfig {
                            keys: parse_keys(props, key_count),
                            special_style: parse_special_style(props),
                            column_layout: parse_column_layout(props, key_count),
                            positions: parse_positions(props),
                            colours: parse_colours(props, key_count),
                            images: parse_images(props, key_count),
                            behavior: parse_behavior(props, key_count),
                        });
                    }
                }
            }
        }
    }

    configs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_ini_test() -> Result<(), Box<dyn Error>> {
        let ini = open_ini("./ressources/test.ini")?;
        let section = ini.section(Some("General")).unwrap();
        assert_eq!(section.get("Name"), Some("Aggro"));
        assert_eq!(section.get("Author"), Some("virtual"));
        assert_eq!(section.get("Version"), Some("2.7"));

        let ini = open_ini("./ressources/test2.ini")?;
        let section = ini.section(Some("General")).unwrap();
        assert_eq!(section.get("Name"), Some("YUGEN ❯REMASTERED v1.2"));
        assert_eq!(section.get("Author"), Some("Garin"));
        assert_eq!(section.get("Version"), Some("2.5"));
        Ok(())
    }
    #[test]
    fn test_parse_color() {
        assert_eq!(parse_color("255,128,64"), Some([255, 128, 64, 255]));
        assert_eq!(parse_color("255,128,64,192"), Some([255, 128, 64, 192]));
        assert_eq!(parse_color("255, 128, 64"), Some([255, 128, 64, 255]));
        assert_eq!(parse_color("255,128"), None);
        assert_eq!(parse_color("invalid"), None);
    }

    #[test]
    fn test_parse_color_rgb() {
        assert_eq!(parse_color_rgb("255,128,64"), Some([255, 128, 64]));
        assert_eq!(parse_color_rgb("255, 128, 64"), Some([255, 128, 64]));
        assert_eq!(parse_color_rgb("invalid"), None);
    }

    #[test]
    fn test_parse_comma_separated_numbers() {
        assert_eq!(parse_comma_separated_numbers(Some("1,2,3")), vec![1, 2, 3]);
        assert_eq!(
            parse_comma_separated_numbers(Some("1, 2, 3")),
            vec![1, 2, 3]
        );
        assert_eq!(parse_comma_separated_numbers(None), Vec::<u32>::new());
    }

    #[test]
    fn test_parse_keys() {
        let mut props = Properties::new();
        props.insert("KeysUnderNotes".to_string(), "1".to_string());

        let keys = parse_keys(&props, 4);
        assert_eq!(keys.count, 4);
        assert!(keys.keys_under_notes);

        let empty_props = Properties::new();
        let default_keys = parse_keys(&empty_props, 4);
        assert_eq!(default_keys.count, 4);
        assert!(!default_keys.keys_under_notes);
    }

    #[test]
    fn test_parse_special_style() {
        let mut props = Properties::new();
        props.insert("SpecialStyle".to_string(), "2".to_string());
        props.insert("SplitStages".to_string(), "1".to_string());
        props.insert("StageSeparation".to_string(), "10".to_string());
        props.insert("SeparateScore".to_string(), "1".to_string());

        let style = parse_special_style(&props);
        assert_eq!(style.style_type, 2);
        assert!(style.split_stages);
        assert_eq!(style.stage_separation, 10);
        assert!(style.separate_score);
    }

    #[test]
    fn test_integration() -> Result<(), Box<dyn Error>> {
        let ini = open_ini("./ressources/test.ini")?;
        let configs = read_mania_config(&ini);

        assert!(
            !configs.is_empty(),
            "Le fichier de configuration devrait contenir au moins une configuration mania"
        );

        let first_config = &configs[0];
        assert!(
            first_config.keys.count > 0,
            "Le nombre de touches devrait être supérieur à 0"
        );

        Ok(())
    }

    #[test]
    fn test_parse_lighting_width() {
        let mut props = Properties::new();
        props.insert("LightingNWidth".to_string(), "2,3,4".to_string());
        props.insert("LightingLWidth".to_string(), "5,6,7".to_string());

        let lighting = parse_lighting_width(&props);
        assert_eq!(lighting.n_width, vec![2, 3, 4]);
        assert_eq!(lighting.l_width, vec![5, 6, 7]);
    }

    #[test]
    fn test_parse_column_layout() {
        let mut props = Properties::new();
        props.insert("ColumnStart".to_string(), "10".to_string());
        props.insert("ColumnRight".to_string(), "20".to_string());
        props.insert("ColumnWidth".to_string(), "32,32,32".to_string());
        props.insert("ColumnSpacing".to_string(), "2,2".to_string());
        props.insert("ColumnLineWidth".to_string(), "1,1,1".to_string());
        props.insert("BarlineHeight".to_string(), "1.5".to_string());
        props.insert("WidthForNoteHeightScale".to_string(), "100".to_string());

        let layout = parse_column_layout(&props, 3);
        assert_eq!(layout.column_start, 10);
        assert_eq!(layout.column_right, 20);
        assert_eq!(layout.column_width, vec![32, 32, 32]);
        assert_eq!(layout.column_spacing, vec![2, 2]);
        assert_eq!(layout.column_line_width, vec![1, 1, 1]);
        assert_eq!(layout.barline_height, 1.5);
        assert_eq!(layout.width_for_note_height_scale, Some(100));
    }

    #[test]
    fn test_parse_positions() {
        let mut props = Properties::new();
        props.insert("HitPosition".to_string(), "400".to_string());
        props.insert("LightPosition".to_string(), "200".to_string());
        props.insert("ScorePosition".to_string(), "300".to_string());
        props.insert("ComboPosition".to_string(), "350".to_string());

        let positions = parse_positions(&props);
        assert_eq!(positions.hit_position, 400);
        assert_eq!(positions.light_position, 200);
        assert_eq!(positions.score_position, 300);
        assert_eq!(positions.combo_position, 350);
    }

    // Tests pour les fonctions liées aux images
    #[test]
    fn test_parse_stage_images() {
        let mut props = Properties::new();
        props.insert("StageLeft".to_string(), "left.png".to_string());
        props.insert("StageRight".to_string(), "right.png".to_string());
        props.insert("StageBottom".to_string(), "bottom.png".to_string());
        props.insert("StageHint".to_string(), "hint.png".to_string());
        props.insert("StageLight".to_string(), "light.png".to_string());
        props.insert("StageLightingN".to_string(), "lighting_n.png".to_string());
        props.insert("StageLightingL".to_string(), "lighting_l.png".to_string());
        props.insert("WarningArrow".to_string(), "warning.png".to_string());

        let images = parse_stage_images(&props);
        assert_eq!(images.left, "left.png");
        assert_eq!(images.right, "right.png");
        assert_eq!(images.bottom, "bottom.png");
        assert_eq!(images.hint, "hint.png");
        assert_eq!(images.light, "light.png");
        assert_eq!(images.lighting_n, "lighting_n.png");
        assert_eq!(images.lighting_l, "lighting_l.png");
        assert_eq!(images.warning_arrow, "warning.png");
    }

    #[test]
    fn test_parse_hit_images() {
        let mut props = Properties::new();
        props.insert("Hit0".to_string(), "hit0.png".to_string());
        props.insert("Hit50".to_string(), "hit50.png".to_string());
        props.insert("Hit100".to_string(), "hit100.png".to_string());
        props.insert("Hit200".to_string(), "hit200.png".to_string());
        props.insert("Hit300".to_string(), "hit300.png".to_string());
        props.insert("Hit300g".to_string(), "hit300g.png".to_string());

        let images = parse_hit_images(&props);
        assert_eq!(images.hit_0, "hit0.png");
        assert_eq!(images.hit_50, "hit50.png");
        assert_eq!(images.hit_100, "hit100.png");
        assert_eq!(images.hit_200, "hit200.png");
        assert_eq!(images.hit_300, "hit300.png");
        assert_eq!(images.hit_300g, "hit300g.png");
    }

    // Tests pour les fonctions liées aux couleurs
    #[test]
    fn test_parse_colours() {
        let mut props = Properties::new();
        props.insert("Colour1".to_string(), "255,0,0,255".to_string());
        props.insert("Colour2".to_string(), "0,255,0,255".to_string());
        props.insert("ColourLight1".to_string(), "255,255,0,255".to_string());
        props.insert("ColourLight2".to_string(), "0,255,255,255".to_string());
        props.insert(
            "ColourColumnLine".to_string(),
            "255,255,255,255".to_string(),
        );
        props.insert("ColourBarline".to_string(), "200,200,200,255".to_string());
        props.insert("ColourJudgementLine".to_string(), "150,150,150".to_string());
        props.insert("ColourKeyWarning".to_string(), "255,0,0".to_string());
        props.insert("ColourHold".to_string(), "255,230,0,255".to_string());
        props.insert("ColourBreak".to_string(), "255,0,0".to_string());

        let colours = parse_colours(&props, 2);
        assert_eq!(colours.columns.len(), 2);
        assert_eq!(colours.lights.len(), 2);
        assert_eq!(colours.column_line, [255, 255, 255, 255]);
        assert_eq!(colours.barline, [200, 200, 200, 255]);
        assert_eq!(colours.judgement_line, [150, 150, 150]);
        assert_eq!(colours.key_warning, [255, 0, 0]);
        assert_eq!(colours.hold, [255, 230, 0, 255]);
        assert_eq!(colours.break_colour, [255, 0, 0]);
    }

    // Tests pour les fonctions liées au comportement
    #[test]
    fn test_parse_behavior() {
        let mut props = Properties::new();
        props.insert("JudgementLine".to_string(), "1".to_string());
        props.insert("LightFramePerSecond".to_string(), "60".to_string());
        props.insert("UpsideDown".to_string(), "0".to_string());
        props.insert("ComboBurstStyle".to_string(), "2".to_string());

        let behavior = parse_behavior(&props, 4);
        assert!(behavior.judgement_line);
        assert_eq!(behavior.light_frame_per_second, 60);
        assert!(!behavior.upside_down);
        assert_eq!(behavior.combo_burst_style, 2);
    }

    #[test]
    fn test_parse_note_body_style() {
        let mut props = Properties::new();
        props.insert("NoteBodyStyle".to_string(), "1".to_string());
        props.insert("NoteBodyStyle0".to_string(), "2".to_string());
        props.insert("NoteBodyStyle1".to_string(), "3".to_string());

        let style = parse_note_body_style(&props, 2);
        assert_eq!(style.global, 1);
        assert_eq!(style.per_column, vec![2, 3]);
    }

    #[test]
    fn test_parse_flip_config() {
        let mut props = Properties::new();
        props.insert("KeyFlip".to_string(), "1".to_string());
        props.insert("NoteFlip".to_string(), "1".to_string());
        props.insert("KeyFlip0".to_string(), "1".to_string());
        props.insert("KeyFlip1".to_string(), "0".to_string());
        props.insert("NoteFlip0".to_string(), "1".to_string());
        props.insert("NoteFlip1".to_string(), "0".to_string());

        let config = parse_flip_config(&props, 2);
        assert!(config.key_flip);
        assert!(config.note_flip);
        assert_eq!(config.per_column_key_flip, vec![true, false]);
    }


    #[test]
    fn test_parse_numbered_strings() {
        let mut props = Properties::new();
        props.insert("Test0".to_string(), "value0".to_string());
        props.insert("Test1".to_string(), "value1".to_string());
        props.insert("Test0D".to_string(), "value0D".to_string());
        props.insert("Test1D".to_string(), "value1D".to_string());

        let strings = parse_numbered_strings(&props, "Test", 2, None);
        assert_eq!(strings, vec!["value0", "value1"]);

        let strings_with_suffix = parse_numbered_strings(&props, "Test", 2, Some("D"));
        assert_eq!(strings_with_suffix, vec!["value0D", "value1D"]);
    }


    #[test]
    fn test_parse_numbered_values() {
        let mut props = Properties::new();
        props.insert("Test0".to_string(), "1".to_string());
        props.insert("Test1".to_string(), "2".to_string());

        let values: Vec<i32> = parse_numbered_values(&props, "Test", 3, 0);
        assert_eq!(values, vec![1, 2, 0]);
    }

    #[test]
    fn test_parse_numbered_bools() {
        let mut props = Properties::new();
        props.insert("Test0".to_string(), "1".to_string());
        props.insert("Test1".to_string(), "0".to_string());

        let bools = parse_numbered_bools(&props, "Test", 3);
        assert_eq!(bools, vec![true, false, false]);
    }
}