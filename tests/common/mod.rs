use insta_cmd::get_cargo_bin;
use std::{ffi::OsStr, path::PathBuf, process::Command};
use tempfile::{TempDir, tempdir};

pub struct Fixture {
    _bin_path: PathBuf,
    _temp_dir: TempDir,
    data_file_path: String,
}

#[cfg(test)]
#[allow(unused)]
impl Fixture {
    pub fn new() -> Self {
        let bin_path = get_cargo_bin("tomo");
        let temp_dir = tempdir().expect("temporary directory should've been created");
        let data_file_path = temp_dir
            .path()
            .join(".tomo")
            .to_str()
            .expect("temporary directory path is not valid utf-8")
            .to_string();

        Self {
            _bin_path: bin_path,
            _temp_dir: temp_dir,
            data_file_path,
        }
    }

    pub fn base_cmd(&self) -> Command {
        let mut cmd = Command::new(&self._bin_path);
        cmd.args(["--data-file", &self.data_file_path]);
        cmd
    }

    pub fn cmd<I, S>(&self, args: I) -> Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut command = Command::new(&self._bin_path);
        command.args(args);
        command.args(["--data-file", &self.data_file_path]);
        command
    }
}
