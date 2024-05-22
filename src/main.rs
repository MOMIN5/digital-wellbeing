#![windows_subsystem = "windows"]
mod gui;

use std::{fs::{File, self}, io::{Write, Read}, collections::HashMap, thread, time::Duration, env};

use chrono::{Local, DateTime};
use winapi;
use sysinfo::{System,Pid};

fn main() {

    let arg: Vec<String> = env::args().collect();
    if arg.len() > 1 && arg[1] == "nogui" {
        loop{
            let pid = get_foreground_process();
            let name = get_process_name(pid);

            update_time(name);
            thread::sleep(Duration::from_secs(3));
        }
    }else {
        gui::gui().unwrap();
    }
}

fn get_foreground_process() -> u32 {
    unsafe {
        let current_window = winapi::um::winuser::GetForegroundWindow();
        
        let mut pid = 1;
        winapi::um::winuser::GetWindowThreadProcessId(current_window, &mut pid);
        
        return pid;
    }
}

fn get_process_name(current_pid: u32) -> String{
    let sys = System::new_all();
    
    let process = sys.processes().get(&Pid::from_u32(current_pid));
    
    if let Some(process) = process {
        let proc_name = process.name();
        let mut upp_str = String::new();
        
        let mut chars = proc_name.chars();
        match chars.next() {
            None => (),
            Some(f) => {
                upp_str = f.to_uppercase().collect::<String>() + chars.as_str()
            }
        }

        let non_exe_name: String = upp_str.chars().rev().skip(4).collect();
        let sanitized_name = non_exe_name.chars().rev().collect();
        return sanitized_name;
    }
    return "".to_string();
}

fn update_time(app_name : String) {
    let time_stamp = 3;
    let file_path = get_filepath();
    
    let mut app_map = read_file();
    
    //let modif_data = String::from(app_name.as_str()) + ":" + time_stamp.to_string().as_str();
    //let data_byte = modif_data.as_bytes();
    
    let mut file = File::options().write(true).open(file_path).unwrap();
    
    if let Some(time) = app_map.get_mut(&app_name) {
        *time += time_stamp;
    }else {
        app_map.insert(app_name, 0);
    }
    
    println!("{:?}",app_map);
    
    for (name, time) in app_map.iter() {
        writeln!(file, "{}:{}",name,time).unwrap();
    }
}

fn read_file() -> HashMap<String,i32> {
    let file_path = get_filepath();
    
    let file = File::open(&file_path);
    let mut map: HashMap<String, i32> = HashMap::new();

    match file {
        Ok(mut f) => {
            let mut data_str = String::new();
            f.read_to_string(&mut data_str);

            let lines = data_str.split("\n");

            for line in lines {
                if let Some((name, time)) = line.split_once(":") {
                    //println!("{}",time);
                    map.insert(name.to_string(), time.parse::<i32>().unwrap());
                } 
            }
        }
        Err(_) => {
            File::create(file_path).unwrap();
        }
    }
    return map;
}

fn get_filepath() -> String {
    let path = std::env::var("APPDATA").map( |path| path.to_string()).unwrap();

    let date: DateTime<Local> = Local::now();
    fs::create_dir_all(String::from(&path) + "\\digital-wellbeing\\Data").unwrap();
    let file_path = path + "\\digital-wellbeing\\Data\\" + date.date_naive().to_string().as_str() + ".log";

    return file_path;
}