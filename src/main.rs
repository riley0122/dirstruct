use std::fs;

struct File {
    is_folder: bool,
    name: String,
    contents: Vec<File>
}

fn read_folder(folder: File, start_folder_path: String) -> Result<File, std::io::Error> {
    if !folder.is_folder {
        println!("Something went wrong!");
    }

    let mut contents = Vec::new();

    let cwd = std::env::current_dir()?;
    let folder_path = cwd.join(format!("{}/{}", start_folder_path, folder.name.clone()));

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let name = entry_path.file_name().unwrap().to_string_lossy().to_string();
        let is_folder = entry_path.is_dir();
        if is_folder {
            let sub_directory_file = read_folder(File {
                is_folder: true,
                name,
                contents: Vec::new(),
            }, format!("{}/{}", start_folder_path, folder.name.clone()))?;
            contents.push(sub_directory_file);
        } else {
            contents.push(File {
                is_folder: false,
                name: name.clone(),
                contents: Vec::new(),
            });
        }
    }

    Ok(File {
        is_folder: true,
        name: folder.name,
        contents,
    })
}

fn draw_files(files: &Vec<File>, depth: u8) {
    for i in 0..files.len() {
        let file = &files[i.clone()];

        let mut start_char = "├";

        if i == files.len() - 1 {
            start_char = "└";
        } else if i == 0 && depth == 0{
            start_char = "┬";
        }
        println!("{}{} {}", "│ ".repeat(depth.clone().try_into().unwrap()), start_char ,file.name.clone());

        if file.is_folder.clone() {
            draw_files(&file.contents, depth + 1);
        }
    }
}

fn main() {
    let paths = fs::read_dir("./").unwrap();

    let mut files: Vec<File> = Vec::new();

    for path in paths {
        let file = File {
            is_folder: *(&path.as_ref().unwrap().metadata().unwrap().is_dir()),
            name: path.as_ref().unwrap().file_name().into_string().unwrap(),
            contents: Vec::new()
        };

        files.push(file);
    }

    for file in files.iter_mut() {
        if file.is_folder {
            if let Ok(updated_file) = read_folder(File {
                is_folder:true,
                name: file.name.clone(),
                contents: vec![]
            }, ".".to_string()) {
                file.contents = updated_file.contents;
            }
        }
    }

    draw_files(&files, 0);
}
