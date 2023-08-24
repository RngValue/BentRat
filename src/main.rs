use std::process::Command;
use sysinfo::{CpuExt, System, SystemExt};
use colored::Colorize;
use directories::ProjectDirs;
use std::fs::read_to_string;

pub mod config_creator;

fn main() {
    let mut sys = System::new_all();
    let mut ram_usage_in_perc;

    let mut sys_name = format!("{:?}", sys.name());
    let mut sys_ver = format!("{:?}", sys.os_version());
    sys_name = str::replace(&sys_name, "Some(\"","");
    sys_name = str::replace(&sys_name, "\")","");
    sys_ver = str::replace(&sys_ver, "Some(\"","");
    sys_ver = str::replace(&sys_ver, "\")","");
    let sys_name_n_ver = format!("{} {}", sys_name, sys_ver);

    //COLORS
    let syscol;
    let acccol = [0, 110, 255];
    let goocol = [50, 200, 0];
    let okcol = [255, 255, 0];
    let badcol = [190, 0, 0];

    let proj_dirs = ProjectDirs::from("com", "CodMan",  "BentRat");
    // Linux:   /home/alice/.config/barapp
    // Windows: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App
    // macOS:   /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App

    let binding = proj_dirs.expect("ha");
    let path_to_conf = binding.config_dir().to_str().unwrap();
    config_creator::check_and_fix_config(path_to_conf);
    let mut ascii_line;
    let which_ascii;
    

    if sys_name == "Ubuntu" {
        which_ascii = "Ubuntu";
        syscol = [255, 100, 0];
    } else if sys_name == "Windows" {
        which_ascii = "Windows";
        syscol = [40, 198, 201];
    } else if sys_name == "Arch Linux" {
        which_ascii = "Arch Linux";
        syscol = [23, 217, 178];
    } else if sys_name == "Linux Mint" {
        which_ascii = "Linux Mint";
        syscol = [138, 226, 52];
    } else if sys_name == "EndeavourOS" {
        which_ascii = "EndeavourOS";
        syscol = [196, 58, 224];
    } else {
        which_ascii = "Default";
        syscol = [180, 180, 180];
    }

    let lines = read_lines(format!("{}/{}.txt", path_to_conf, which_ascii).as_str());

    //println!("{}", format!("{}{}.txt", path_to_conf, sys_name).as_str());
    loop {
        ascii_line = 0;
        if Command::new("clear").status().unwrap().success() {
            sys.refresh_all();
            //System
            print!("{}\t", lines[ascii_line].truecolor(syscol[0], syscol[1], syscol[2]));
            ascii_line += 1;
            println!("{}\t{}", "System name:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), sys_name_n_ver.truecolor(syscol[0], syscol[1], syscol[2]));
            
            //CPU
            for cpu in sys.cpus() {
                print!("{}\t", lines[ascii_line].truecolor(syscol[0], syscol[1], syscol[2]));
                ascii_line += 1;
                if cpu.cpu_usage().floor() < 50.0 {
                    println!("{}{}\t{}", cpu.name().truecolor(acccol[0], acccol[1], acccol[2]).bold(), " usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} %", cpu.cpu_usage().floor()).truecolor(goocol[0], goocol[1], goocol[2]));
                } else if cpu.cpu_usage().floor() > 50.0 && cpu.cpu_usage().floor() < 100.0 {
                    println!("{}{}\t{}", cpu.name().truecolor(acccol[0], acccol[1], acccol[2]).bold(), " usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} %", cpu.cpu_usage().floor()).truecolor(okcol[0], okcol [1], okcol[2]));
                } else {
                    println!("{}{}\t{}", cpu.name().truecolor(acccol[0], acccol[1], acccol[2]).bold(), " usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} %", cpu.cpu_usage().floor()).truecolor(badcol[0], badcol[1], badcol[2]));
                }
            }

            //RAM
            print!("{}\t", lines[ascii_line].truecolor(syscol[0], syscol[1], syscol[2]));
            ascii_line += 1;
            println!("{}\t{}", "RAM capacity:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} MB", sys.total_memory()/1048576).truecolor(goocol[0], goocol[1], goocol[2]));
            
            print!("{}\t", lines[ascii_line].truecolor(syscol[0], syscol[1], syscol[2]));
            ascii_line += 1;
            ram_usage_in_perc = (sys.used_memory()*10)/sys.total_memory();
            if ram_usage_in_perc < 5 {
                println!("{}\t{}", "RAM usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} MB", sys.used_memory()/1048576).truecolor(goocol[0], goocol[1], goocol[2]));
            } else if ram_usage_in_perc > 5 && ram_usage_in_perc < 10 {
                println!("{}\t{}", "RAM usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} MB", sys.used_memory()/1048576).truecolor(okcol[0], okcol [1], okcol[2]));
            } else {
                println!("{}\t{}", "RAM usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} MB", sys.used_memory()/1048576).truecolor(badcol[0], badcol[1], badcol[2]));
            }
            
            print!("{}\t", lines[ascii_line].truecolor(syscol[0], syscol[1], syscol[2]));
            println!("press [CTRL + C] to exit...");
            for i in &lines[ascii_line..lines.len()] {
                println!("{}\t", i.truecolor(syscol[0], syscol[1], syscol[2]));
            }
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    return result;
}