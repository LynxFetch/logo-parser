// SPDX-FileCopyrightText: 2023 CELESTIFYX Team
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fs,
    path::Path,
    str::FromStr
};

use chromakitx::{
    AnsiColor,
    CssColor,
    XtermColor,
    AnyColor
};

use super::{
    error::Error,

    config::{
        LogoConfig,
        ColorConfig,
        ColorDef
    }
};

pub struct LogoParser;

impl LogoParser {
    pub fn from(path: &Path) -> Result<(Vec<String>, Vec<String>, ColorConfig), Error> {
        if !path.exists() {
            return Err(Error::FileNotFound(path.to_path_buf()));
        }

        let content: String = fs::read_to_string(path)?;

        let config: LogoConfig = json5::from_str(&content)?;
        config.validate().map_err(Error::ValidationError)?;

        let large: Vec<String> = config.get_large_logo().unwrap_or_else(|| Vec::new());
        let small: Vec<String> = config.get_small_logo().unwrap_or_else(|| Vec::new());

        if large.is_empty() && small.is_empty() {
            return Err(Error::ValidationError("No valid logo found".to_string()));
        }

        let color_config: ColorConfig = Self::parse_colors(&config)?;
        Ok((large, small, color_config))
    }

    fn parse_colors(config: &LogoConfig) -> Result<ColorConfig, Error> {
        let color_defs: Vec<ColorDef> = config.get_color_defs().ok_or_else(|| Error::ValidationError("Color definitions not found".to_string()))?;

        if color_defs.is_empty() {
            return Err(Error::ValidationError("Color definitions array is empty".to_string()));
        }

        let title_ref: String = config.get_title_color_ref().ok_or_else(|| Error::ValidationError("Title color reference not found".to_string()))?;
        let title_color: AnyColor = Self::resolve_color_reference(&title_ref, &color_defs)?;

        let keys_ref: String = config.get_keys_color_ref().ok_or_else(|| Error::ValidationError("Keys color reference not found".to_string()))?;
        let keys_color: AnyColor = Self::resolve_color_reference(&keys_ref, &color_defs)?;

        let mut ascii_colors: Vec<AnyColor> = Vec::new();

        for (idx, _) in color_defs.iter().enumerate() {
            let color_ref: String = format!("%{}", idx);
            let color: AnyColor = Self::resolve_color_reference(&color_ref, &color_defs)?;

            (&mut ascii_colors).push(color);
        }

        Ok(ColorConfig {
            title_color,
            keys_color,
            ascii_colors
        })
    }

    fn resolve_color_reference(reference: &str, color_defs: &[ColorDef]) -> Result<AnyColor, Error> {
        let reference: &str = reference.trim();

        if reference.starts_with('%') {
            let idx_str: &str = &reference[1..];
            let idx: usize = idx_str.parse().map_err(|_| Error::ColorParseError(format!("Invalid color index: {}", idx_str)))?;

            if idx >= color_defs.len() {
                return Err(Error::ColorParseError(format!("Color index out of bounds: {}", idx), ));
            }

            return Self::parse_color_def(&color_defs[idx]);
        }

        Self::parse_color_name(reference)
    }

    fn parse_color_def(color_def: &ColorDef) -> Result<AnyColor, Error> {
        match color_def {
            ColorDef::Simple(name) => Self::parse_color_name(name),

            ColorDef::Named(map) => {
                if map.len() != 1 {
                    return Err(Error::ColorParseError("Color definition object must have exactly one entry".to_string()));
                }

                let (color_name, color_type): (&String, &String) = map.iter().next().ok_or_else(|| Error::ColorParseError("Empty color definition".to_string()))?;

                match color_type.to_lowercase().as_str() {
                    "ansi" => Self::parse_ansi_color(color_name),
                    "xterm" => Self::parse_xterm_color(color_name),
                    "css" => Self::parse_css_color(color_name),

                    _ => Err(Error::ColorParseError(format!("Unknown color type: {}", color_type)))
                }
            }
        }
    }

    fn parse_color_name(name: &str) -> Result<AnyColor, Error> {
        AnsiColor::from_str(name).map(AnyColor::Ansi).or_else(|_| {
            CssColor::from_str(name).map(AnyColor::Css)
        }).or_else(|_| {
            XtermColor::from_str(name).map(AnyColor::Xterm)
        }).map_err(|_| Error::ColorParseError(format!("Unknown color name: {}", name)))
    }

    fn parse_ansi_color(name: &str) -> Result<AnyColor, Error> {
        AnsiColor::from_str(name).map(AnyColor::Ansi).map_err(|_| Error::ColorParseError(format!("Unknown ANSI color: {}", name)))
    }

    fn parse_xterm_color(name: &str) -> Result<AnyColor, Error> {
        XtermColor::from_str(name).map(AnyColor::Xterm).map_err(|_| Error::ColorParseError(format!("Unknown Xterm color: {}", name)))
    }

    fn parse_css_color(name: &str) -> Result<AnyColor, Error> {
        CssColor::from_str(name).map(AnyColor::Css).map_err(|_| Error::ColorParseError(format!("Unknown CSS color: {}", name)))
    }
}
