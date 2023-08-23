use std::env;
use std::error::Error;
use std::f32::consts::E;
use std::fs;
use std::fs::metadata;
use std::fs::File;
use std::io;
use std::io::stdout;
use std::io::Read;
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

use chrono::Local;
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
// use hex_literal::hex;

pub struct HLS_Log {
    date_time: String,
    video_name: String,
    Video_hash: String,
}

pub struct HLS_list_log {
    list: Vec<HLS_Log>,
}

pub struct HLS {}

impl HLS {
    fn add_text_to_log(file_path: &PathBuf)->String {
        let file_name = &file_path.file_name().unwrap();
        let site_path = HLS::site_dir_path();
        let log_path = site_path.join("config").join("hls.logs.json");

        let current_datetime = Local::now();
        let formatted_datetime = current_datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        if log_path.exists() == false {
            let log_file = fs::write(&log_path, "");
        }

        let mut get_log_file = fs::read_to_string(&log_path).unwrap();

        let hash = HLS::file_md5(&file_path.display().to_string());
        let new_log_text = format!(
            "Date Time : {}\nFile : '{}'\nhash : {}\n\n",
            formatted_datetime,
            file_name.to_str().unwrap().to_string(),
            &hash,
        );

        let lines:Vec<&str> = get_log_file.lines().collect();
        let count = lines.len();


        
        if count > 40{
            let mut new_lines = String::new();
            for (i,line) in lines.iter().enumerate(){
                if i > 40{
                    break;
                }
                new_lines.push_str(&line);
                new_lines.push_str("\n");

            }

            get_log_file = new_lines;
        }

        get_log_file = format!("{}{}",new_log_text, get_log_file); 

        let _ = fs::write(log_path, get_log_file);
        hash
    }

    fn file_md5(file_path: &str) -> String {
        let mut file = File::open(file_path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let digest = md5::compute(buffer);
        let hash = format!("{:?}", digest);

        hash
    }

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

    fn get_video_list_text(directory: &str) -> (Option<String>, usize) {
        let video_path = HLS::site_dir_path().join(directory);

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
        let (text, count) = HLS::get_video_list_text("videos");

        Console::clear();
        Console::warning("Delete all files above?");

        println!();
        println!("{}", text.unwrap());
        println!();

        Console::print_color("Enter 'yes' to delete: ".red().bold());

        let answer = Console::input();

        Console::clear();

        if answer.to_lowercase() == "yes" {
            let path = HLS::site_dir_path().join("videos");
            let result = fs::remove_dir_all(&path);
            fs::create_dir_all(path);

            if result.is_ok() {
                Console::success("All files were deleted\n");
            } else {
                println!("{:?}", result.err().unwrap().to_string());
                Console::error("The operation failed \n");
            }
        }
    }

    pub fn remove_all_hls_videos() {
        let (text, count) = HLS::get_video_list_text("hls_videos");

        Console::clear();
        Console::warning("Delete all files above? (HLS Files)");

        println!();
        println!("{}", text.unwrap());
        println!();

        Console::print_color("Enter 'yes' to delete: ".red().bold());

        let answer = Console::input();

        Console::clear();

        if answer.to_lowercase() == "yes" {
            let path = HLS::site_dir_path().join("hls_videos");
            let result = fs::remove_dir_all(&path);
            fs::create_dir_all(path);

            if result.is_ok() {
                Console::success("All files were deleted\n");
            } else {
                println!("{:?}", result.err().unwrap().to_string());
                Console::error("The operation failed \n");
            }
        }
    }

    pub fn show_files_list() {
        let (text, file_index) = HLS::get_video_list_text("videos");

        if text.is_none() {
            return;
        }

        let text = text.unwrap();

        Console::clear();

        if file_index == 0 {
            Console::warning("Video not found\n");
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

            let mut new_file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            new_file_name = new_file_name.trim().to_lowercase().replace(" ", "_");
            new_file_name = new_file_name.replace("&", "_and_");
            new_file_name = new_file_name.replace("$", "_");

            let copy_result = fs::copy(path, &video_path.join(&new_file_name));

            if copy_result.is_ok() {
                Console::success(&new_file_name);
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

        let config_enc_key = HLS::site_dir_path().join("config").join("enc.keyinfo");

        if (&config_enc_key).exists() == false {
            Console::clear();
            Console::error("Config file not found");
            return;
        }

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

                    if file.path().extension().unwrap().eq("mp4") == false {
                        println!("{}", "   ❌ File not support".red());
                        continue;
                    }

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

                    let mut done_status = true;

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

                        let config_enc_key = config_enc_key.clone();

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
                                .arg(config_enc_key.display().to_string())
                                .arg(format!(
                                    "{}/video.{}p.m3u8",
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
                                "   ✅  {} {} pixel {}",
                                "Created".green().bold(),
                                res.to_string().green().bold(),
                                HLS::format_duration(index_sec)
                            );
                        } else {

                            
                            done_status = false;

                            println!(
                                "   ❌  {} {} pixel in {}",
                                "Error".red().bold(),
                                res.to_string().red().bold(),
                                HLS::format_duration(index_sec)
                            );
                        }

                        timer.join().unwrap();
                    } // end convert all files


                    // If the conversion is done correctly
                    if done_status {

                        // get hash string
                        let hash = HLS::add_text_to_log(&file.path());

                        // show video hash
                        println!(
                            "   ✅  Video hash : {}",
                            hash.green().bold().italic(),
                        );
                    }
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
        let site_path = HLS::site_dir_path();
        let files_path = FileDialog::new()
            .set_location("~/Desktop")
            .add_filter("keyinfo", &["keyinfo", "key"])
            .show_open_multiple_file()
            .unwrap();

        let mut import_index = 0;

        for path in files_path {
            let file_name = path.file_name().clone().unwrap().to_str().unwrap();

            let to = site_path.join("config").join(&file_name);

            let c = fs::copy(&path, &to);

            if c.is_ok() {
                import_index += 1;
            }

            if (&path).extension().unwrap().eq("keyinfo") {
                let text = fs::read_to_string(&path);

                match text {
                    Ok(text) => {
                        let path_enc = HLS::site_dir_path().join("config").join("enc.key");

                        let lines: Vec<&str> = text.lines().collect();

                        let mut new_str = String::new();

                        for (i, line) in lines.iter().enumerate() {
                            if i == 1 {
                                new_str.push_str(path_enc.display().to_string().as_str());
                            } else {
                                new_str.push_str(&line);
                            }
                            new_str.push('\n');
                        }

                        let _ = fs::write(&to, new_str);
                    }
                    Err(_) => {}
                }
            }

            Console::clear();

            if import_index == 2 {
                Console::success("Config file added\n");
            } else {
                Console::warning("Both 'enc.keyinfo' and 'enc.key' files must be selected for the program to function properly\n");
            }
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
