use std::fs;

const DOT: &str = ".";

fn has_extension(file_name: &str, ext: &str) -> bool {
    let mut sections: Vec<&str> = file_name.split(DOT).collect();
    if let Some(extension) = sections.pop() {
        extension == ext
    } else {
        false
    }
}

fn get_file_name(entry: &fs::DirEntry) -> String {
    let file_name = entry.file_name();
    String::from(file_name.to_str().unwrap())
}

fn get_path_to_file(entry: &fs::DirEntry) -> String {
    let entry_path = entry.path();
    String::from(entry_path.to_str().unwrap())
}

pub fn by_extension(path: &str, extension: &str) -> Vec<String> {
    let mut files_by_extension: Vec<String> = vec![];
    match fs::read_dir(path) {
        Ok(files) => {
            files_by_extension = files.fold(files_by_extension, |mut results, result| {
                if let Ok(entry) = result {
                    let file_type = entry.file_type().unwrap();
                    let file_name = get_file_name(&entry);
                    let file_path = get_path_to_file(&entry);
                    if file_type.is_dir() {
                        let files_with_extension: Vec<String> = by_extension(&file_path, extension);
                        results.extend_from_slice(&files_with_extension);
                    } else if has_extension(&file_name, extension) {
                        results.push(file_path);
                    }
                }
                results
            })
        }
        Err(error) => panic!("No files found: {:?}", error),
    };
    files_by_extension
}

#[cfg(test)]
pub mod migration_tests {
    use super::*;

    #[test]
    fn get_files_by_extension() {
        let dir_path = String::from("/home/moon/web/byThePeoples/lib/db/src");
        let file_name = String::from("sql_file.sql");
        let file_path = format!("{}/{}", dir_path, file_name);
        fs::File::create(&file_path);
        let files = by_extension(&dir_path, "sql");
        fs::remove_file(&file_path);
        assert!(files.contains(&file_path));
    }
}
