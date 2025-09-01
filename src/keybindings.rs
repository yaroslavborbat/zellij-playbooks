use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use zellij_tile::prelude::*;

const BIND_EDIT: &str = "bind_edit";
const BIND_RELOAD: &str = "bind_reload";
const BIND_SWITCH_FILTER_ID: &str = "bind_switch_filter_id";

#[derive(Clone, Debug)]
pub(crate) struct Keybindings {
    pub edit: Keybinding,
    pub reload: Keybinding,
    pub switch_filter_id: Keybinding,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            edit: Keybinding::new(KeyModifier::Ctrl, 'e'),
            reload: Keybinding::new(KeyModifier::Ctrl, 'r'),
            switch_filter_id: Keybinding::new(KeyModifier::Ctrl, 'i'),
        }
    }
}

impl Keybindings {
    pub fn new(conf: BTreeMap<String, String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut default = Self::default();

        if let Some(value) = conf.get(BIND_EDIT) {
            default.edit = parse_key_info(value)?
        }
        if let Some(value) = conf.get(BIND_RELOAD) {
            default.reload = parse_key_info(value)?
        }
        if let Some(value) = conf.get(BIND_SWITCH_FILTER_ID) {
            default.switch_filter_id = parse_key_info(value)?
        }

        Ok(default)
    }
}

fn parse_key_info(binding: &String) -> Result<Keybinding, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = binding.split_whitespace().collect();
    if parts.len() != 2 {
        return Err(format!("Invalid keybinding format: {}", binding).into());
    }
    let modifier = KeyModifier::from_str(parts[0])?;
    let key_char = parts[1].chars().next().ok_or("Missing key character")?;
    Ok(Keybinding::new(modifier, key_char))
}

#[derive(Clone, Debug)]
pub(crate) struct Keybinding {
    key_with_modifier: KeyWithModifier,
}

impl Keybinding {
    pub(crate) fn new(modifier: KeyModifier, key: char) -> Self {
        Self {
            key_with_modifier: KeyWithModifier::new_with_modifiers(
                BareKey::Char(key),
                BTreeSet::from([modifier]),
            ),
        }
    }

    pub(crate) fn matches(&self, key: &KeyWithModifier) -> bool {
        self.key_with_modifier.eq(key)
    }
}

impl fmt::Display for Keybinding {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.key_with_modifier.fmt(f)
    }
}
