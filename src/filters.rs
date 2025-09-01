use crate::{file_picker::FileItem, PlaybookLine};
use std::fmt;
use std::fmt::Formatter;

pub(crate) trait Filter<T> {
    fn keep(&self, t: &T) -> bool;
}

#[derive(Default, PartialEq, Copy, Clone, Debug)]
pub(crate) enum Mode {
    #[default]
    Name,
    ID,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Name => "Name",
            Self::ID => "ID",
        };
        write!(f, "{}", name)
    }
}

impl Mode {
    pub(crate) fn switch_to(&self, mode: Mode) -> Self {
        if *self == mode {
            Mode::default()
        } else {
            mode
        }
    }
}

pub(crate) struct PlaybookFilter {
    mode: Mode,
    filter: String,
}

impl PlaybookFilter {
    pub(crate) fn new(mode: Mode, filter: String) -> Self {
        PlaybookFilter { mode, filter }
    }

    fn keep_by_name(&self, line: &PlaybookLine) -> bool {
        if self.filter.is_empty() {
            return true;
        }
        line.content.contains(&self.filter)
    }

    fn keep_by_id(&self, line: &PlaybookLine) -> bool {
        line.id.to_string().starts_with(&self.filter.to_string())
    }
}

impl Filter<PlaybookLine> for PlaybookFilter {
    fn keep(&self, line: &PlaybookLine) -> bool {
        match self.mode {
            Mode::ID => self.keep_by_id(line),
            _ => self.keep_by_name(line),
        }
    }
}

pub(crate) struct FileFilter {
    mode: Mode,
    filter: String,
}

impl FileFilter {
    pub(crate) fn new(mode: Mode, filter: String) -> Self {
        FileFilter { mode, filter }
    }

    fn keep_by_name(&self, file: &FileItem) -> bool {
        if self.filter.is_empty() {
            return true;
        }
        file.name.contains(&self.filter)
    }

    fn keep_by_id(&self, file: &FileItem) -> bool {
        file.id.to_string().starts_with(&self.filter.to_string())
    }
}

impl Filter<FileItem> for FileFilter {
    fn keep(&self, file: &FileItem) -> bool {
        match self.mode {
            Mode::ID => self.keep_by_id(file),
            _ => self.keep_by_name(file),
        }
    }
}
