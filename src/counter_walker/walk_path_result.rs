use std::ops::{Add, AddAssign};

#[derive(Debug)]
pub struct WalkPathResult {
    pub line_count: i64,
    pub folder_count: usize,
    pub file_count: usize,
    pub empty_file_count: usize,
    pub error_file_count: usize,
}

#[allow(dead_code)]
impl WalkPathResult {
    fn total_files(&self) -> usize {
        self.file_count + self.error_file_count + self.empty_file_count
    }

    fn total_entries(&self) -> usize {
        self.total_files() + self.folder_count
    }

    pub fn new() -> WalkPathResult {
        WalkPathResult {
            line_count: 0,
            folder_count: 0,
            file_count: 0,
            empty_file_count: 0,
            error_file_count: 0,
        }
    }
}

impl Add for WalkPathResult {
    type Output = WalkPathResult;

    fn add(self, rhs: Self) -> Self::Output {
        return WalkPathResult {
            line_count: self.line_count + rhs.line_count,
            folder_count: self.folder_count + rhs.folder_count,
            file_count: self.file_count + rhs.file_count,
            empty_file_count: self.empty_file_count + rhs.empty_file_count,
            error_file_count: self.error_file_count + rhs.error_file_count,
        };
    }
}

impl AddAssign for WalkPathResult {
    fn add_assign(&mut self, rhs: Self) {
        self.line_count += rhs.line_count;
        self.folder_count += rhs.folder_count;
        self.file_count += rhs.file_count;
        self.empty_file_count += rhs.empty_file_count;
        self.error_file_count += rhs.error_file_count;
    }
}