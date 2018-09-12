use std::fs;
use std::ffi::OsString;

const POSSIBLE_DISKS: [&str; 10] = ["C", "D", "E", "F", "G", "H", "I", "J", "K", "L"];
const CAMERA_DIR: &str = r"PRIVATE\AVCHD\BDMV\STREAM";
const USER_DIR: &str = r"C:\Users\niedzwiedz\Documents\camera_videos";


fn files<'a>(directory: &String) -> std::result::Result<std::vec::Vec<std::fs::DirEntry>, std::io::Error>
{
    let mut files_ = vec![];
    let reader = fs::read_dir(directory)?;
    for file in reader {
        files_.push(file?);
    }

    Ok(files_)
}

fn is_camera_dir(disk: &String) -> bool {
    match files(disk) {
        Ok(files_) => {
            let required_files = ["DCIM", "MISC", "PRIVATE"];

            let file_present = |filename: &str| -> bool {
                for file in &files_ {
                    if file.file_name() == filename { return true }
                }
                false
            };

            required_files.iter().all(|elem| file_present(elem))
        },
        Err(_) => { return false }
    }

}

fn camera_disk<'a>() -> Option<String> {
    let disks: Vec<String> = POSSIBLE_DISKS.iter().map(|elem| {
        format!("{}:\\", elem)
    }).collect();

    for disk in disks {
        if is_camera_dir(&disk) {
            return Some(disk)
        }
    }
    None
}


fn contains(element: &std::fs::DirEntry, vector: &Vec<std::fs::DirEntry>) -> bool {
    for el in vector.iter() {
        if element.file_name() == el.file_name() {  // TODO: Implement equality using hash system
            return true
        }
    }

    false
}


fn main() -> Result<(), std::io::Error> {
    match camera_disk() {
        Some(disk) => {
            let camera_dir = format!("{}{}", disk, CAMERA_DIR);

            let to_copy = files(&camera_dir)?;
            let current = files(&String::from(USER_DIR))?;

            println!("Copying {} to {}", camera_dir, USER_DIR);
            for file in to_copy {
                if !contains(&file, &current) {
                    let source = format!("{}\\{}", &camera_dir, file.file_name().into_string().unwrap());
                    let destination = format!("{}\\{}", &USER_DIR, file.file_name().into_string().unwrap());
                    println!(
                        "{} -> {}",
                        &source,
                        &destination,
                    );

                    fs::copy(source, destination).expect("Failed to copy file.");
                }
            }
        },
        None => {println!("Camera SD card is not connected.")}
    }

    Ok(())
}
