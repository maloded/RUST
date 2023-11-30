use std::{ffi::OsStr, io::Write, path::PathBuf, time::Instant};

const FILE_SIZE_BASE: f64 = 1e6;

fn get_input(query: &str) -> std::io::Result<String> {
    print!("{}", query);
    std::io::stdout().flush()?;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

fn get_search_data() -> Option<(String, String, Vec<String>)> {
    let search_path = match get_input("Enter path to the dir to search for file in: ") {
        Ok(path) => path,
        Err(err) => {
            println!("{err}");
            return None;
        }
    };

    let search_name = match get_input("Enter filename to search without extension: ") {
        Ok(name) => name,
        Err(err) => {
            println!("{err}");
            return None;
        }
    };

    let search_extensions = match get_input("Enter file extensions separated by space: ") {
        Ok(ext) => get_extensions(ext),
        Err(err) => {
            println!("{err}");
            return None;
        }
    };

    if search_path.is_empty() || (search_name.is_empty() && search_extensions.is_empty()) {
        println!("Enter something");
        return None;
    }

    Some((
        search_path.to_lowercase(),
        search_name.to_lowercase(),
        search_extensions,
    ))
}

fn search_files(
    search_path: &str,
    filename: &str,
    extensions: &Vec<String>,
    now: Instant,
    results_count: &mut i32,
) {
    let is_no_extensions = extensions.is_empty();
    let is_empty_filename = filename.is_empty();

    let files = match std::fs::read_dir(search_path) {
        Ok(files) => files,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    for entry in files {
        if let Ok(entry) = entry {
            let path = entry.path();
            let file_name = convert_os_string(path.file_stem());
            let file_extension = convert_os_string(path.extension());

            if path.is_dir() {
                if is_no_extensions && file_name.contains(&filename) {
                    file_found(&path, &now, results_count)
                }

                search_files(
                    path.to_str().unwrap_or_default(),
                    filename,
                    extensions,
                    now,
                    results_count,
                )
            } else if is_empty_filename && extensions.contains(&file_extension) {
                file_found(&path, &now, results_count);
            } else if path.is_file() && file_name.contains(filename) {
                if (!is_no_extensions && extensions.contains(&file_extension)) || is_no_extensions {
                    file_found(&path, &now, results_count);
                }
            }
        }
    }
}

fn convert_os_string(os_str: Option<&OsStr>) -> String {
    os_str
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_lowercase()
}

fn get_extensions(extensions_string: String) -> Vec<String> {
    extensions_string
        .split_whitespace()
        .map(|ext| ext.to_lowercase())
        .collect()
}

fn file_found(path: &PathBuf, now: &Instant, results_count: &mut i32) {
    *results_count += 1;
    print_path_info(path, now);
}

fn print_path_info(path: &PathBuf, now: &Instant) {
    print!(
        "{} - Found in {} seconds",
        path.display(),
        now.elapsed().as_secs_f64()
    );

    match std::fs::metadata(path) {
        Ok(metadata) => {
            print!(" - {} MB\n", metadata.len() as f64 / FILE_SIZE_BASE);
        }
        Err(_) => println!(),
    }
}

fn main() {
    loop {
        let (path, filename, extensions) = match get_search_data() {
            Some(data) => data,
            None => continue,
        };
        println!();

        let now = Instant::now();
        let mut results_count = 0;

        search_files(&path, &filename, &extensions, now, &mut results_count);

        println!(
            "Total time: {}s, {} result found",
            now.elapsed().as_secs_f64(),
            results_count
        );
    }
}
