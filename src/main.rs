use std::{
    collections::HashSet,
    ffi::OsStr,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let project_name = current_dir.file_name().unwrap().to_string_lossy();

    let files = search_files(&PathBuf::from("."));

    let mut uses = HashSet::new();
    let mut content = Vec::new();

    files.into_iter().for_each(|file| {
        let s = fs::read_to_string(&file).unwrap();
        let mut content_start = None;

        for (line_no, line) in s.lines().enumerate() {
            if line.starts_with("using ") {
                uses.insert(line.to_string());
                continue;
            }

            if !line.chars().all(|c| c.is_whitespace()) {
                content_start = Some(line_no);
                break;
            }
        }

        let path_string = file.to_string_lossy();
        let bars = "-".repeat(path_string.len());
        let mut non_use_content = s
            .lines()
            .skip(content_start.unwrap_or(s.lines().count()))
            .collect::<Vec<&str>>()
            .join("\n")
            .trim()
            .to_string();

        if non_use_content.chars().all(|c| !c.is_ascii()) {
            non_use_content = "// [empty file]".to_string()
        }

        content.push(format!(
            "// {bars}\n// {path_string}\n// {bars}\n{non_use_content}"
        ));
    });

    let mut uses = uses.into_iter().collect::<Vec<_>>();
    uses.sort();

    content.sort();

    let out = format!(
        "{}\n\nnamespace {project_name}Singular {{\n{}\n}}",
        uses.join("\n"),
        content.join("\n\n")
    );

    if !PathBuf::from("out").exists() {
        fs::create_dir("out").unwrap()
    }

    let out_path = PathBuf::from("out").join(format!("{project_name}.cs"));

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&out_path)
        .unwrap();

    file.write_all(out.as_bytes()).unwrap();

    println!("Output can be found in {out_path:?}")
}

fn search_files(dir: &Path) -> HashSet<PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .filter_map(|file| {
            let file = file.unwrap();

            if !file
                .file_name()
                .to_string_lossy()
                .chars()
                .next()
                .unwrap()
                .is_ascii_uppercase()
            {
                return None;
            }

            if file.metadata().unwrap().is_file() {
                if file.path().extension() == Some(&OsStr::new("cs")) {
                    Some(vec![file.path()])
                } else {
                    None
                }
            } else {
                Some(search_files(file.path().as_path()).into_iter().collect())
            }
        })
        .flatten()
        .collect()
}
