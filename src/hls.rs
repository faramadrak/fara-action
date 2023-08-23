use std::env;
use std::f32::consts::E;
use std::fs;
use std::fs::metadata;
use std::fs::File;
use std::io;
use std::io::stdout;
use std::io::Stdout;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time;
use std::time::Duration;

use crossterm::cursor;
use crossterm::style::Stylize;
use crossterm::terminal;
use crossterm::ExecutableCommand;
use crossterm::QueueableCommand;
use native_dialog::FileDialog;

use crate::console::Console;
use crate::Site;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::os::unix::fs::PermissionsExt;

pub struct HLS {}

impl HLS {
    fn root_path() -> PathBuf {
        let mut path = env::current_exe().expect("Not access in hls");
        path.pop();

        path
    }

    fn site_dir_path() -> PathBuf {
        let mut path = HLS::root_path();
        let mut current = Site::get_current();
        path = path.join(current.unwrap().title);

        path
    }

    pub fn init() {
        let mut path = HLS::root_path();
        let mut current = Site::get_current();

        if current.is_some() {
            path = path.join(current.unwrap().title);

            let _ = fs::create_dir(&path);

            let ـ = fs::create_dir(&path.join("videos"));
            let _ = fs::create_dir(&path.join("hls_videos"));
            let _ = fs::create_dir(&path.join("config"));
        }
    }

    fn get_video_list_text() -> (Option<String>, usize) {
        let video_path = HLS::site_dir_path().join("videos");

        let files = video_path.read_dir();

        if files.is_err() {
            Console::clear();
            Console::error("The program does not have permission to access and create files.");
            return (None, 0);
        }

        let mut text = String::new();
        let mut file_index = 0;

        for file in files.unwrap() {
            if file.is_ok() {
                let dfile = file.ok().unwrap();
                let file_name = &dfile.file_name();
                let buf = &dfile.path().clone();
                let ext = buf.extension();

                file_index += 1;

                let mut support = false;

                if ext.is_some() {
                    let ex = ext.unwrap();

                    if ex.eq("mp4") {
                        text += &format!("🎬 {}. {}\n", file_index, file_name.to_str().unwrap());
                        support = true;
                    }
                }

                if support == false {
                    text += &format!(
                        "⛔️ {}. {} {}\n",
                        file_index,
                        file_name.to_str().unwrap(),
                        "Not support".red()
                    );
                }
            }
        }
        (Some(text), file_index)
    }

    pub fn remove_all_org_videos() {
        let (text, count) = HLS::get_video_list_text();

        Console::clear();
        Console::warning("Delete all files above?");

        println!();
        println!("{}",text.unwrap());
        println!();



        Console::print_color("Enter 'yes' to delete: ".red().bold());

        let answer = Console::input();

        Console::clear();

        if answer.to_lowercase() == "yes" {
            let path = HLS::site_dir_path().join("videos");
            let result = fs::remove_dir_all(&path);
            fs::create_dir_all(path);

            if result.is_ok(){
                Console::success("All files were deleted\n");
            }
            else{
                println!("{:?}", result.err().unwrap().to_string());
                Console::error("The operation failed \n");
            }
        }
    }

    pub fn show_files_list() {
        let (text, file_index) = HLS::get_video_list_text();

        if text.is_none() {
            return;
        }

        let text = text.unwrap();

        Console::clear();

        if file_index == 0 {
            Console::warning("No video found\n");
        } else {
            Console::println_color("List of videos".blue().bold());
            println!();
            println!("{}", text);
            Console::print("(Enter to back)");
            Console::input();
            Console::clear();
        }
    }

    pub fn add_a_video() {
        let video_path = HLS::site_dir_path().join("videos");

        let files_path = FileDialog::new()
            .set_location("~/Downloads")
            .add_filter("Select mp4 video", &["mp4"])
            .show_open_multiple_file()
            .unwrap();

        let mut import_count = 0;

        for path in &files_path {
            println!("path:{}", path.display().to_string());
            let copy_result = fs::copy(path, &video_path.join(path.file_name().unwrap()));

            if copy_result.is_ok() {
                import_count += 1;
            }
        }

        let select_count = files_path.len();
        Console::clear();

        if select_count > 0 {
            if import_count == 0 {
                Console::warning("No files were added");
                let md = fs::metadata(video_path);
                match md {
                    Ok(meta) => {
                        let permissions = meta.permissions();
                        let readonly = permissions.readonly();

                        if readonly {
                            Console::warning(
                                "The program does not have permission to access and create files",
                            );
                        }
                    }
                    Err(_) => {
                        Console::warning(
                            "The program does not have permission to access and create files.",
                        );
                    }
                }
            } else {
                Console::success(&format!(" {} videos were added", import_count).to_string());
            }
        }
    }

    fn format_duration(seconds: i64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let seconds = seconds % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    pub fn start_all_video() {
        let site_path = HLS::site_dir_path();
        let video_path = site_path.join("videos").clone();
        let hls_video_path: &PathBuf = &site_path.join("hls_videos").clone();

        let files = fs::read_dir(&video_path).expect("Can not access to video directory");

        let mut index = 0;

        for result in files {
            index += 1;

            match result {
                Ok(file) => {
                    let file_path: String = file.path().display().to_string();
                    let file_name = file.file_name();

                    println!();

                    println!(
                        "# 🛠  Working on '{}' 🔻",
                        file.file_name().to_str().unwrap().blue().bold()
                    );

                    let dir = hls_video_path.join(&file_name);
                    let directory = fs::create_dir(&dir);

                    if dir.exists() == false && directory.is_err() {
                        println!();
                        Console::error(&format!(
                            "Can not create :\n   '{}' directory\n",
                            dir.display()
                        ));
                        return;
                    }

                    println!(
                        "   ✅  Created hls_videos/{} Directory",
                        file_name.as_os_str().to_str().unwrap().blue().bold()
                    );

                    for res in vec![360, 480, 720] {
                        let height = (res * 9 / 16) * 2;

                        let file_path: String = file.path().display().to_string();
                        let save_path = hls_video_path.join(&file_name);

                        let status_text = "🔄  Processing video ".yellow().bold();
                        let mut stdout: Stdout = stdout();
                        let mut index_sec = 0;

                        let result_status = Arc::new(Mutex::new(true));
                        let result_status_clone = Arc::clone(&result_status);

                        let status_run = Arc::new(Mutex::new(true));
                        let status_run_clone = Arc::clone(&status_run);

                        let timer = thread::spawn(move || {
                            // let res = 480;

                            let output = Command::new("ffmpeg")
                                .arg("-i")
                                .arg(file_path)
                                .arg("-c:a")
                                .arg("aac")
                                .arg("-strict")
                                .arg("experimental")
                                .arg("-c:v")
                                .arg("libx264")
                                .arg("-s")
                                // .arg(format!("{}x{}", res, res * 9 / 16))
                                .arg(format!("{}x{}", res, height)) // Use the calculated height
                                .arg("-aspect")
                                .arg("16:9")
                                .arg("-f")
                                .arg("hls")
                                .arg("-hls_list_size")
                                .arg("1000000")
                                .arg("-hls_time")
                                .arg("2")
                                .arg("-hls_key_info_file")
                                .arg(
                                    HLS::site_dir_path()
                                        .join("config")
                                        .join("enc.keyinfo")
                                        .display()
                                        .to_string(),
                                )
                                .arg(format!(
                                    "{}/video'.{}p.m3u8",
                                    save_path.display().to_string(),
                                    res
                                ))
                                // .stdout(Stdio::piped())
                                // .stderr(Stdio::inherit())
                                .output()
                                .unwrap();

                            if output.status.success() {
                                let mut run_mutex = status_run_clone.lock().unwrap();
                                *run_mutex = false;
                            } else {
                                let mut status = result_status_clone.lock().unwrap();
                                *status = false;

                                let mut run_mutex = status_run_clone.lock().unwrap();
                                *run_mutex = false;
                                eprintln!(
                                    "Command failed with exit code: {}",
                                    output.status.code().unwrap()
                                );
                            }
                        });

                        loop {
                            stdout.queue(cursor::SavePosition).unwrap();
                            stdout
                                .write_all(
                                    format!(
                                        "   {} ({})",
                                        status_text,
                                        HLS::format_duration(index_sec)
                                    )
                                    .as_bytes(),
                                )
                                .unwrap();
                            stdout.queue(cursor::RestorePosition).unwrap();
                            stdout.flush().unwrap();
                            thread::sleep(time::Duration::from_secs(1));

                            stdout.queue(cursor::RestorePosition).unwrap();
                            stdout
                                .queue(terminal::Clear(terminal::ClearType::FromCursorDown))
                                .unwrap();
                            index_sec += 1;

                            let run_mutex = *status_run.lock().unwrap();
                            if run_mutex == false {
                                break;
                            }
                        }
                        stdout.execute(cursor::Show).unwrap();

                        if *result_status.lock().unwrap() {
                            println!(
                                "   ✅  {} {} pixel in {}",
                                "Created".green().bold(),
                                res.to_string().green().bold(),
                                HLS::format_duration(index_sec)
                            );
                        } else {
                            println!(
                                "   ❌  {} {} pixel in {}",
                                "Error".red().bold(),
                                res.to_string().red().bold(),
                                HLS::format_duration(index_sec)
                            );
                        }

                        timer.join().unwrap();
                    }

                    // let get_str = HLS::get_command_string_hls(
                    //     video_path.join(&file_name).display().to_string(),
                    //     hls_video_path.join(&file_name).display().to_string(),
                    //     String::from("video"),
                    // );
                    // println!("{}", get_str);
                    // let output = Command::new("ffmpeg")
                    //     .arg("-i")
                    //     .arg(video_path.join(file_path))
                    //     .arg(hls_video_path)
                    //     .output();

                    // let command = Command::new("ffmpeg");

                    // command.arg(arg);
                    // thread::spawn(move ||{
                    // let output = Command::new("ffmpeg")
                    //     .arg("-i")
                    //     .arg("/app/fara_option/target/debug/faramadrak.com/videos/fff.mp4")
                    //     .arg("-c:a")
                    //     .arg("aac")
                    //     .arg("-strict")
                    //     .arg("experimental")
                    //     .arg("-c:v")
                    //     .arg("libx264")
                    //     .arg("-s")
                    //     .arg("640x360")
                    //     .arg("-aspect")
                    //     .arg("16:9")
                    //     .arg("-f")
                    //     .arg("hls")
                    //     .arg("-hls_list_size")
                    //     .arg("1000000")
                    //     .arg("-hls_time")
                    //     .arg("2")
                    //     .arg("-hls_key_info_file")
                    //     .arg("/app/fara_option/target/debug/faramadrak.com/config/enc.keyinfo")
                    //     .arg("/app/fara_option/target/debug/faramadrak.com/hls_videos/fff.mp4/video'.360p.m3u8")
                    //     // .arg("2>&1")
                    //     .output().unwrap();

                    // }).join();
                }
                Err(_) => todo!(),
                // Err(_) => {}
            }
        }

        if index == 0 {
            Console::warning("Videos directory is empty.");
            Console::warning(&format!("Put videos in '{}' path.", video_path.display()));
            return;
        }

        // match output {
        //     Ok(success) => {
        //         if success.status.success() {
        //             println!("FFmpeg command executed successfully!");
        //         } else {
        //             eprintln!(
        //                 "FFmpeg command failed with error code: {:?}",
        //                 success.status
        //             );
        //         }
        //     }
        //     Err(error) => {
        //         eprintln!("Failed to execute FFmpeg command: {}", error);
        //     }
        // }
    }

    pub fn get_command_string_hls(
        input_path: String,
        output_path: String,
        save_name: String,
    ) -> String {
        let keyinfo = HLS::site_dir_path().join("config").join("enc.keyinfo");
        format!("ffmpeg -i '{}'  -c:a aac -strict experimental -c:v libx264 -s 640x360 -aspect 16:9 -f hls -hls_list_size 1000000 -hls_time 2 -hls_key_info_file {} '{}/{}'.360p.m3u8 2>&1",
         input_path,keyinfo.display().to_string(), output_path, save_name).to_string()
    }

    pub fn select_key_file() {
        let to = HLS::site_dir_path();
        let path = FileDialog::new()
            .set_location("~/Desktop")
            .add_filter("keyinfo", &["keyinfo"])
            .show_open_single_file()
            .unwrap();

        match path {
            Some(path) => {
                let c = fs::copy(path, to.join("config/enc.keyinfo"));

                match c {
                    Ok(cc) => {}
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
            None => {}
        }
    }

    fn generate_secret_token() -> String {
        let hex_string: String = (0..16)
            .map(|_| format!("{:02x}", rand::thread_rng().gen::<u8>()))
            .collect();

        hex_string
    }

    fn generate_random_key() -> [u8; 16] {
        let mut rng = rand::thread_rng();
        let mut key: [u8; 16] = [0; 16];
        rng.fill(&mut key);

        key
    }

    pub fn create_new_key() {
        Console::clear();

        let token = HLS::generate_secret_token();
        let path_encinfo = HLS::site_dir_path().join("config").join("enc.keyinfo");
        let path_enc = HLS::site_dir_path().join("config").join("enc.key");

        if path_encinfo.exists() {
            Console::warning("The configuration file already exists");
            Console::print("Do you want to replace the new settings?(yes or no):");
            let answer = Console::input();

            if answer != "yes" {
                Console::warning("The operation was canceled");
                return;
            }
        }

        Console::warning("HLS settings include two enc.key files and enc.keyinfo files, the content of the enc.key file must be accessible through a secure address.");
        println!();
        Console::print("Enter the url to access the contents of the enc.key file \n Url:");
        let get_url = Console::input();

        let text: String =
            format!("{}\n{}\n{}", get_url, path_encinfo.display(), token).to_string();

        let mut enc = File::create(path_encinfo).expect("Can not create file");
        enc.write_all(text.as_bytes()).expect("Can not save file");

        let mut enckey = File::create(path_enc).expect("Can not create enc.key file");
        enckey
            .write_all(&HLS::generate_random_key())
            .expect("Can not save env.key file");

        Console::success(&format!(
            "The enc.key and enc.keyinfo files were saved in the {} path",
            HLS::site_dir_path().display()
        ));
    }
}
