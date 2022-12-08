use std::collections::HashMap;

pub struct FileSystem {
    current_path: Vec<String>,
    pub root: Directory,
}

impl FileSystem {
    fn add_to_current_directory(&mut self, first_part: &str, second_part: &str) {
        let dir = self.get_current_directory();
        match first_part {
            "dir" => {
                dir.subdirectories
                    .insert(String::from(second_part), Directory::new());
            }
            bytes => {
                if let Ok(filesize) = bytes.parse() {
                    dir.files.insert(String::from(second_part), filesize);
                }
            }
        }
    }

    fn change_directory(&mut self, dir: &str) {
        match dir {
            "/" => self.current_path = vec![String::from("/")],
            ".." => {
                self.current_path.pop();
            }
            _ => {
                self.current_path.push(String::from(dir));
            }
        }
    }

    fn get_current_directory(&mut self) -> &mut Directory {
        let mut dir = &mut self.root;
        for segment in self.current_path.iter().skip(1) {
            dir = dir
                .subdirectories
                .get_mut(segment)
                .expect("Should have been able to step into dir");
        }
        dir
    }

    pub fn from_commands(commands: &str) -> FileSystem {
        let mut fs = FileSystem {
            current_path: vec![String::from("/")],
            root: Directory::new(),
        };

        for row in commands.split("\r\n") {
            let mut parts = row.split(' ');
            if let Some(first) = parts.next() {
                match first {
                    "$" => {
                        if let Some(command) = parts.next() {
                            match command {
                                "cd" => {
                                    if let Some(dir) = parts.next() {
                                        fs.change_directory(dir);
                                    }
                                }
                                _ => {}
                            };
                        }
                    }
                    _ => fs.add_to_current_directory(first, parts.next().unwrap()),
                }
            }
        }
        fs
    }
}

#[derive(Debug)]
pub struct Directory {
    files: HashMap<String, usize>,
    subdirectories: HashMap<String, Directory>,
}

impl Directory {
    pub fn new() -> Directory {
        Directory {
            files: HashMap::new(),
            subdirectories: HashMap::new(),
        }
    }

    pub fn bytesize(&self) -> usize {
        let files_total = self.files.values().fold(0, |acc, f| acc + f);
        let total_size = self.subdirectories.values().fold(files_total, |acc, dir| {
            return acc + dir.bytesize();
        });
        total_size
    }
}

pub fn sum_bytesizes(directory: &Directory, max_size: usize, total_size: usize) -> usize {
    let mut output = total_size;
    let size = directory.bytesize();
    if size <= max_size {
        output = output + size;
    }
    for sub in directory.subdirectories.values() {
        output = output + sum_bytesizes(&sub, max_size, total_size);
    }

    output
}

pub fn find_directory_to_delete(
    directory: &Directory,
    disk_size: usize,
    required_space: usize,
) -> usize {
    let available_space = disk_size - directory.bytesize();
    let mut closest_match = directory.bytesize();
    find_directory_with_size_of_at_least(
        &directory,
        required_space - available_space,
        &mut closest_match,
    );
    closest_match
}

fn find_directory_with_size_of_at_least(
    directory: &Directory,
    size: usize,
    current_match: &mut usize,
) {
    let dir_size = directory.bytesize();
    if dir_size < *current_match && dir_size >= size {
        *current_match = dir_size;
    }
    for sub in directory.subdirectories.values() {
        find_directory_with_size_of_at_least(sub, size, current_match);
    }
}
