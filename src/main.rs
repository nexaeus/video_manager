use std::path::PathBuf;
use video_manager::{
    resolve_abs_location,
    files_discovery,
    filter_files,
};

fn main() {
    let mut files_list: Vec<PathBuf> = Vec::new();

    let videos_location = resolve_abs_location("Videos").unwrap();
    let _ = files_discovery(videos_location, &mut files_list);

    println!("{:?}", filter_files(files_list, "jpg"));
}


