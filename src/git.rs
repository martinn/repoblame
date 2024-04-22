use bytelines::ByteLines;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Child, ChildStdout, Command, Stdio};

pub struct GitTree {
    pub child: Child,
}

impl GitTree {
    pub fn new(repo_path: &Path) -> Self {
        let cmd = Command::new("git")
            .current_dir(repo_path)
            .arg("ls-tree")
            .arg("-r")
            .arg("--name-only")
            .arg("HEAD")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute git command. Is this a git repository?");

        GitTree { child: cmd }
    }

    pub fn iter(&mut self) -> GitTreeIter<BufReader<&mut ChildStdout>> {
        GitTreeIter {
            source: BufReader::new(self.child.stdout.as_mut().unwrap()),
        }
    }
}

impl Drop for GitTree {
    fn drop(&mut self) {
        self.child
            .wait()
            .expect("Failed to wait for git ls-tree command.");
    }
}

pub struct GitTreeIter<T>
where
    T: BufRead,
{
    pub source: T,
}

impl<T> Iterator for GitTreeIter<T>
where
    T: BufRead,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.source.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    line.pop(); // Remove '\n'
                    Some(line)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}

pub struct GitBlame {
    pub child: Child,
}

impl GitBlame {
    pub fn new(repo_path: &Path, file_path: &Path) -> Self {
        let cmd = Command::new("git")
            .current_dir(repo_path)
            .arg("blame")
            .arg("--line-porcelain")
            .arg("-M")
            .arg("-C")
            .arg("HEAD")
            .arg(file_path)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute git blame.");

        GitBlame { child: cmd }
    }

    pub fn iter(&mut self) -> GitBlameIter {
        GitBlameIter {
            source: ByteLines::new(BufReader::new(self.child.stdout.as_mut().unwrap())),
        }
    }
}

impl Drop for GitBlame {
    fn drop(&mut self) {
        self.child
            .wait()
            .expect("Failed to wait for git blame command.");
    }
}

pub struct GitBlameIter<'a> {
    pub source: ByteLines<BufReader<&'a mut ChildStdout>>,
}

impl Iterator for GitBlameIter<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.next() {
            Some(bytes_read) => match bytes_read {
                Ok(byte_line) => {
                    let line = String::from_utf8_lossy(byte_line).to_string();
                    Some(line)
                }
                Err(_) => None,
            },
            None => None,
        }
    }
}
