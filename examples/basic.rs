// SPDX-FileCopyrightText: 2023 CELESTIFYX Team
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    error::Error,
    path::Path
};

use logo_parser::{
    ColorConfig,
    LogoParser
};

fn main() -> Result<(), Box<dyn Error>> {
    let path: &Path = Path::new("examples/tux.json5");

    println!("Loading logo from file...");
    let (large, _, colors): (Vec<String>, Vec<String>, ColorConfig) = LogoParser::from(path)?;

    println!("Logo lines: {}", large.len());
    println!("Title color: {:?}", colors.title_color);
    println!("Keys color: {:?}", colors.keys_color);
    println!("ASCII colors: {}", colors.ascii_colors.len());

    for (i, line) in large.iter().enumerate() {
        println!("[{}] {}", i, line);
    }

    Ok(())
}
