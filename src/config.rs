// SPDX-FileCopyrightText: 2023 CELESTIFYX Team
// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

use serde::{
    Deserialize,
    Serialize
};

use chromakitx::AnyColor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct LogoEntry {
    #[serde(rename = "type")]
    pub(super) logo_type: String,
    pub(super) ascii: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(super) enum ColorValue {
    Reference(String),
    Array(Vec<ColorDef>)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(super) enum ColorDef {
    Simple(String),
    Named(HashMap<String, String>)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct LogoConfig {
    pub(super) logos: Vec<LogoEntry>,
    pub(super) colors: Vec<ColorValue>
}

impl LogoConfig {
    pub(super) fn validate(&self) -> Result<(), String> {
        if self.logos.is_empty() {
            return Err("At least one logo is required".to_string());
        }

        let logo_type: String = self.logos[0].logo_type.to_lowercase();

        if (logo_type != "large") && (logo_type != "small") {
            return Err(format!("Logo type must be 'large' or 'small', found '{}'", self.logos[0].logo_type));
        }

        if self.colors.len() != 3 {
            return Err(format!("Expected exactly 3 color elements, found {}", self.colors.len()));
        }

        if !matches!(&self.colors[0], ColorValue::Reference(_)) {
            return Err("First color must be a reference string (e.g., '%0')".to_string());
        }

        if !matches!(&self.colors[1], ColorValue::Reference(_)) {
            return Err("Second color must be a reference string (e.g., '%0')".to_string());
        }

        if !matches!(&self.colors[2], ColorValue::Array(_)) {
            return Err("Third color element must be an array of color definitions".to_string());
        }

        Ok(())
    }

    pub(super) fn get_large_logo(&self) -> Option<Vec<String>> {
        self.logos.iter().find(|l: &&LogoEntry| l.logo_type.to_lowercase() == "large").map(|l: &LogoEntry| l.ascii.clone())
    }

    pub(super) fn get_small_logo(&self) -> Option<Vec<String>> {
        self.logos.iter().find(|l: &&LogoEntry| l.logo_type.to_lowercase() == "small").map(|l: &LogoEntry| l.ascii.clone())
    }

    pub(super) fn get_title_color_ref(&self) -> Option<String> {
        if let ColorValue::Reference(s) = &self.colors[0] {
            Some(s.clone())
        } else {
            None
        }
    }

    pub(super) fn get_keys_color_ref(&self) -> Option<String> {
        if let ColorValue::Reference(s) = &self.colors[1] {
            Some(s.clone())
        } else {
            None
        }
    }

    pub(super) fn get_color_defs(&self) -> Option<Vec<ColorDef>> {
        if let ColorValue::Array(arr) = &self.colors[2] {
            Some(arr.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColorConfig {
    pub title_color: AnyColor,
    pub keys_color: AnyColor,
    pub ascii_colors: Vec<AnyColor>
}
