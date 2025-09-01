mod file_picker;
mod filters;
mod keybindings;
mod render;
mod tab_manager;

use crate::file_picker::{FileItem, FilePicker};
use crate::filters::{FileFilter, Filter, PlaybookFilter};
use crate::keybindings::{Keybinding, Keybindings};
use crate::tab_manager::TabManager;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::io::Read;
use std::{fs, path};
use zellij_tile::prelude::*;

const CONFIGURATION_IGNORE_COMMENTS: &str = "ignore_comments";
const CONFIGURATION_SORT_FILES: &str = "sort_files";
const CONFIGURATION_PIPE_MODE: &str = "pipe_mode";

const CWD: &str = "/host";

const BASE_COLOR: usize = 2;

const RESERVE_ROW_COUNT: usize = 6;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
struct PlaybookLine {
    id: usize,
    content: String,
}

impl PlaybookLine {
    fn new(id: usize, content: String) -> Self {
        Self { id, content }
    }
}

#[derive(Debug, Clone)]
struct State {
    mode: Mode,
    pipe_mode: bool,
    ignore_comments: bool,
    sort_files: bool,
    filter_mode: filters::Mode,
    filter: String,
    files_mgr: TabManager<FileItem>,
    playbook_mgr: TabManager<PlaybookLine>,
    file_picker: FilePicker,
    keybindings: Keybindings,
    crit_error_message: Option<String>,
    error_message: Option<String>,
    current_file: Option<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mode: Default::default(),
            pipe_mode: false,
            ignore_comments: true,
            sort_files: true,
            filter_mode: Default::default(),
            filter: "".to_string(),
            files_mgr: Default::default(),
            playbook_mgr: Default::default(),
            file_picker: Default::default(),
            keybindings: Default::default(),
            crit_error_message: None,
            error_message: None,
            current_file: None,
        }
    }
}

#[derive(Default, PartialEq, Debug, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u32)]
enum Mode {
    #[default]
    FilePicker = 1,
    Playbook = 2,
    Usage = 3,
}

trait Navigation {
    fn next(&self) -> Self;
    fn prev(&self) -> Self;
    fn iter() -> impl Iterator<Item = Self>;
}

impl Navigation for Mode {
    fn next(&self) -> Mode {
        let next = (*self as u32).saturating_add(1);
        Mode::try_from(next).unwrap_or(Mode::FilePicker)
    }

    fn prev(&self) -> Mode {
        let prev = (*self as u32).saturating_sub(1);
        Mode::try_from(prev).unwrap_or(Mode::Usage)
    }

    fn iter() -> impl Iterator<Item = Self> {
        (1..=3).filter_map(|v| Mode::try_from(v).ok())
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::FilePicker => "FilePicker",
            Self::Playbook => "Playbook",
            Self::Usage => "Usage",
        };
        write!(f, "{}", name)
    }
}

impl State {
    fn playbook_filter(&self) -> Box<dyn Filter<PlaybookLine>> {
        Box::new(PlaybookFilter::new(self.filter_mode, self.filter.clone()))
    }

    fn file_filter(&self) -> Box<dyn Filter<crate::file_picker::FileItem>> {
        Box::new(FileFilter::new(self.filter_mode, self.filter.clone()))
    }

    fn set_filter(&mut self) {
        match self.mode {
            Mode::FilePicker => self.files_mgr.with_filter(self.file_filter()),
            Mode::Playbook => self.playbook_mgr.with_filter(self.playbook_filter()),
            _ => {}
        }
    }

    fn get_cwd(&self) -> path::PathBuf {
        path::PathBuf::from(CWD)
    }

    fn load_file(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = self.get_cwd().join(file_path);

        let mut file = fs::File::open(&path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut playbook_lines = Vec::new();
        let mut id = 1;

        for line in lines {
            // Skip empty lines
            if line.trim().is_empty() {
                continue;
            }

            // Skip comments if ignore_comments is enabled
            if self.ignore_comments && line.trim().starts_with('#') {
                continue;
            }

            playbook_lines.push(PlaybookLine::new(id, line));
            id += 1;
        }

        self.playbook_mgr = TabManager::new(playbook_lines);
        self.current_file = Some(file_path.to_string());

        Ok(())
    }

    fn handle_error(&mut self, error_message: String) {
        self.error_message = Some(error_message.clone());
        eprintln!("Error: {}", error_message);
    }

    fn render_errors(&mut self) -> bool {
        if let Some(e) = self.crit_error_message.as_ref() {
            let text = Text::new(format!("ERROR: {}", e.red()));
            print_text_with_coordinates(text, 1, 1, None, None);
            return true;
        }
        if let Some(e) = self.error_message.take() {
            let text = Text::new(format!("ERROR: {}", e.red()));
            print_text_with_coordinates(text, 1, 1, None, None);
            return true;
        }
        false
    }

    fn render_usage(&self) {
        render::render_mode(0, 0, Mode::Usage);

        let mut table = Table::new();

        table = table.add_row(vec!["KeyBinding", "Action", "Mode", "Configurable"]);
        // Non configurable
        table = table.add_row(vec![
            format!(
                "{}|{}",
                BareKey::Esc,
                Keybinding::new(KeyModifier::Ctrl, 'c')
            )
            .as_str(),
            "Exit the zellij-playbooks.",
            "*",
            "False",
        ]);
        table = table.add_row(vec![
            format!("{}|{} {}", BareKey::Tab, BareKey::Down, BareKey::Up).as_str(),
            "Navigate through the list of files or lines.",
            format!("{}|{}", Mode::FilePicker, Mode::Playbook).as_str(),
            "False",
        ]);
        table = table.add_row(vec![
            format!("{} {}", BareKey::Left, BareKey::Right).as_str(),
            "Switch between modes.",
            "*",
            "False",
        ]);
        table = table.add_row(vec![
            BareKey::Backspace.to_string().as_str(),
            "Remove the last character from the filter.",
            format!("{}|{}", Mode::FilePicker, Mode::Playbook).as_str(),
            "False",
        ]);
        table = table.add_row(vec![
            BareKey::Enter.to_string().as_str(),
            "Select file or paste the selected line into the terminal.",
            "*",
            "False",
        ]);
        table = table.add_row(vec![
            format!("{:?} {}", KeyModifier::Ctrl, Mode::FilePicker as u32).as_str(),
            "Switch to File Picker mode.",
            "*",
            "False",
        ]);
        table = table.add_row(vec![
            format!("{:?} {}", KeyModifier::Ctrl, Mode::Playbook as u32).as_str(),
            "Switch to Playbook mode.",
            "*",
            "False",
        ]);
        table = table.add_row(vec![
            format!("{:?} {}", KeyModifier::Ctrl, Mode::Usage as u32).as_str(),
            "Switch to Usage mode to view instructions.",
            "*",
            "False",
        ]);

        // Configurable
        table = table.add_row(vec![
            self.keybindings.edit.to_string().as_str(),
            "Open the selected file in an editor.",
            format!("{}|{}", Mode::FilePicker, Mode::Playbook).as_str(),
            "True",
        ]);
        table = table.add_row(vec![
            self.keybindings.reload.to_string().as_str(),
            "Reload files from current directory.",
            "*",
            "True",
        ]);
        table = table.add_row(vec![
            self.keybindings.switch_filter_id.to_string().as_str(),
            "Switch to id filtering mode.",
            format!("{}|{}", Mode::FilePicker, Mode::Playbook).as_str(),
            "True",
        ]);

        print_table_with_coordinates(table, 2, 2, None, None);
    }

    fn render_file_picker(&self, rows: usize, cols: usize) {
        let iter = self.files_mgr.iter().map(|(i, f)| (i, f.id, &f.name));
        render::render_main_menu(
            rows,
            cols,
            self.files_mgr.get_position(),
            self.files_mgr.len(),
            Mode::FilePicker,
            self.filter.clone(),
            self.filter_mode.to_string(),
            iter,
        );
    }

    fn render_playbook(&self, rows: usize, cols: usize) {
        let iter = self.playbook_mgr.iter().map(|(i, l)| (i, l.id, &l.content));
        render::render_main_menu(
            rows,
            cols,
            self.playbook_mgr.get_position(),
            self.playbook_mgr.len(),
            Mode::Playbook,
            self.filter.clone(),
            self.filter_mode.to_string(),
            iter,
        );
    }
}

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
            PermissionType::WriteToStdin,
            PermissionType::OpenFiles,
        ]);

        if let Some(value) = configuration.get(CONFIGURATION_PIPE_MODE) {
            self.pipe_mode = value.trim().parse::<bool>().unwrap_or_else(|_| {
                self.handle_error(
                    format!("'{CONFIGURATION_PIPE_MODE}' config value must be 'true' or 'false', but it's '{value}'. The true is used.")
                );
                false
            })
        }

        if self.pipe_mode {
            return;
        }

        if let Some(value) = configuration.get(CONFIGURATION_IGNORE_COMMENTS) {
            self.ignore_comments = value.trim().parse::<bool>().unwrap_or_else(|_| {
                self.handle_error(
                    format!("'{CONFIGURATION_IGNORE_COMMENTS}' config value must be 'true' or 'false', but it's '{value}'. The true is used.")
                );
                true
            })
        }

        if let Some(value) = configuration.get(CONFIGURATION_SORT_FILES) {
            self.sort_files = value.trim().parse::<bool>().unwrap_or_else(|_| {
                self.handle_error(
                    format!("'{CONFIGURATION_SORT_FILES}' config value must be 'true' or 'false', but it's '{value}'. The true is used.")
                );
                true
            })
        }

        match Keybindings::new(configuration) {
            Ok(kb) => self.keybindings = kb,
            Err(e) => {
                self.handle_error(format!(
                    "Failed to parse zellij-playbooks keybindings, check your config: {}. Default is used.", e
                ));
            }
        }

        // Load files from current directory
        if let Err(e) = self
            .file_picker
            .load_files(&self.get_cwd(), self.sort_files)
        {
            self.handle_error(format!("Failed to load files: {}", e));
        }
        self.files_mgr = self.file_picker.manager();

        subscribe(&[EventType::Key]);
    }

    fn update(&mut self, event: Event) -> bool {
        if self.pipe_mode {
            return false;
        }

        let mut should_render = false;

        if let Event::Key(key) = event {
            match key.bare_key {
                // Not configurable keys
                BareKey::Esc => close_focus(),
                BareKey::Char('c') if key.has_modifiers(&[KeyModifier::Ctrl]) => {
                    close_focus();
                }
                BareKey::Down | BareKey::Tab => match self.mode {
                    Mode::FilePicker => {
                        self.files_mgr.select_down();
                        should_render = true;
                    }
                    Mode::Playbook => {
                        self.playbook_mgr.select_down();
                        should_render = true;
                    }
                    _ => {}
                },
                BareKey::Up => match self.mode {
                    Mode::FilePicker => {
                        self.files_mgr.select_up();
                        should_render = true;
                    }
                    Mode::Playbook => {
                        self.playbook_mgr.select_up();
                        should_render = true;
                    }
                    _ => {}
                },
                BareKey::Right => {
                    self.mode = self.mode.next();
                    self.filter_mode = filters::Mode::default();
                    self.set_filter();
                    should_render = true;
                }
                BareKey::Left => {
                    self.mode = self.mode.prev();
                    self.filter_mode = filters::Mode::default();
                    self.set_filter();
                    should_render = true;
                }
                BareKey::Char(c)
                    if key.has_modifiers(&[KeyModifier::Ctrl]) && c.is_ascii_digit() =>
                {
                    if let Some(digit) = c.to_digit(10) {
                        if let Ok(mode) = Mode::try_from(digit) {
                            if self.mode != mode {
                                self.mode = mode;
                                self.filter_mode = filters::Mode::default();
                                self.set_filter();
                                should_render = true;
                            }
                        }
                    }
                }
                BareKey::Char(c) if key.has_no_modifiers() => match self.mode {
                    Mode::FilePicker | Mode::Playbook => {
                        if self.filter.is_empty() {
                            if c.is_ascii_digit() {
                                self.filter_mode = filters::Mode::ID
                            } else {
                                self.filter_mode = filters::Mode::Name
                            }
                        }
                        match self.filter_mode {
                            filters::Mode::ID => {
                                if let Some(digit) = c.to_digit(10) {
                                    if !self.filter.is_empty() || digit > 0 {
                                        self.filter.push(c);
                                        self.set_filter();
                                        should_render = true;
                                    }
                                }
                            }
                            _ => {
                                self.filter.push(c);
                                self.set_filter();
                                should_render = true;
                            }
                        }
                    }
                    _ => {}
                },
                BareKey::Backspace => match self.mode {
                    Mode::FilePicker | Mode::Playbook => {
                        self.filter.pop();
                        self.set_filter();
                        should_render = true;
                    }
                    _ => {}
                },
                BareKey::Enter => match self.mode {
                    Mode::FilePicker => {
                        if let Some(file) = self.files_mgr.get_selected() {
                            let file_name = file.name.clone();
                            if let Err(e) = self.load_file(&file_name) {
                                self.handle_error(format!(
                                    "Failed to load file '{}': {}",
                                    file_name, e
                                ));
                            } else {
                                self.mode = Mode::Playbook;
                                self.filter = "".to_string();
                                self.set_filter();
                            }
                            should_render = true;
                        }
                    }
                    Mode::Playbook => {
                        if let Some(line) = self.playbook_mgr.get_selected() {
                            focus_previous_pane();
                            write_chars(&format!("{}\n", line.content));
                            focus_previous_pane();
                        }
                    }
                    _ => {}
                },
                _ => {
                    // Handle configurable keybindings
                    if self.keybindings.edit.matches(&key) {
                        match self.mode {
                            Mode::FilePicker => {
                                if let Some(f) = self.files_mgr.get_selected() {
                                    let file =
                                        FileToOpen::new(f.name.as_str()).with_cwd(self.get_cwd());
                                    open_file_in_place(file, Default::default());
                                }
                            }
                            Mode::Playbook => {
                                if let Some(f) = self.current_file.clone() {
                                    let file = FileToOpen::new(f.as_str()).with_cwd(self.get_cwd());
                                    open_file_in_place(file, Default::default());
                                }
                            }
                            _ => {}
                        }
                    } else if self.keybindings.reload.matches(&key) {
                        if let Err(e) = self
                            .file_picker
                            .load_files(&self.get_cwd(), self.sort_files)
                        {
                            self.handle_error(format!("Failed to reload files: {}", e));
                        }
                        self.files_mgr = self.file_picker.manager();

                        if let Some(f) = self.current_file.clone() {
                            if let Err(e) = self.load_file(f.as_str()) {
                                self.handle_error(format!("Failed to reload file: {}", e));
                            }
                        }

                        should_render = true;
                    } else if self.keybindings.switch_filter_id.matches(&key) {
                        self.filter_mode = self.filter_mode.switch_to(filters::Mode::ID);
                        self.set_filter();
                        should_render = true;
                    }
                }
            }
        }

        should_render
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        if let PipeSource::Cli(_) = pipe_message.source {
            if let Some(payload) = pipe_message.payload {
                self.handle_error(payload.clone());

                focus_previous_pane();
                write_chars(&format!("{}\n", payload));
                focus_previous_pane();
            }
        }
        false
    }

    fn render(&mut self, rows: usize, cols: usize) {
        if self.pipe_mode {
            return;
        }
        if self.render_errors() {
            return;
        }

        match self.mode {
            Mode::FilePicker => self.render_file_picker(rows, cols),
            Mode::Playbook => self.render_playbook(rows, cols),
            Mode::Usage => self.render_usage(),
        }
    }
}

register_plugin!(State);
