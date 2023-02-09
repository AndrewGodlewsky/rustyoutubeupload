use std::time::Duration;

use fs_extra::file::*;
use terminal_color_builder::OutputFormatter as tcb;
use thirtyfour::prelude::*;

mod chromedriver;
mod cookies;
mod python;
mod upload;
mod video;

use chromedriver::ChromeDriver;
use video::Video;

fn clean_up(video_files: &[std::fs::DirEntry]) {
    // Archive the input video
    let copy_options = &CopyOptions::new().overwrite(true);

    for v in video_files {
        let path = v.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        move_file(
            "Input/".to_string() + file_name,
            "Input/Archive/".to_string() + file_name,
            copy_options,
        )
        .unwrap();
    }

    // Archive the clips
    let video_clip_files = std::fs::read_dir("Clips/")
        .unwrap()
        .map(|d| d.unwrap())
        .filter(|d| d.file_type().unwrap().is_file())
        .filter(|d| d.path().extension().unwrap() == "mp4")
        .collect::<Vec<std::fs::DirEntry>>();

    for v in video_clip_files {
        let path = v.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        move_file(
            "Clips/".to_string() + file_name,
            "Clips/Archive/".to_string() + file_name,
            copy_options,
        )
        .unwrap();
    }

    // Archive the clips data
    move_file("Clips/data.json", "Clips/Archive/data.json", copy_options).unwrap();
}

// #[tokio::main]
// async fn main() {
fn main() {
    python::install_requirements();

    loop {
        std::fs::create_dir_all("Input/Archive").unwrap();
        std::fs::create_dir_all("Clips/Archive").unwrap();
        std::fs::create_dir_all("Music").unwrap();

        std::thread::sleep(Duration::from_secs_f32(1.0));

        let video_files = std::fs::read_dir("Input/")
            .unwrap()
            .map(|d| d.unwrap())
            .filter(|d| d.file_type().unwrap().is_file())
            .filter(|d| d.path().extension().unwrap() == "mp4")
            .collect::<Vec<std::fs::DirEntry>>();

        if video_files.len() == 0 {
            continue;
        }

        println!(
            "Videos Found: {:?}",
            video_files
                .iter()
                .map(|v| v.path().into_os_string())
                .collect::<Vec<std::ffi::OsString>>()
        );

        python::run();

        let json_result = std::fs::read_to_string("Clips/data.json");

        if json_result.is_err() {
            continue;
        }

        let videos: Vec<Video> = serde_json::from_str(&json_result.unwrap()).unwrap();

        let rt = tokio::runtime::Runtime::new().unwrap();
        // rt.spawn(async {
        //     // upload::start(videos).await.unwrap();
        // });

        clean_up(&video_files);
    }
}
