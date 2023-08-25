use std::process::Command;
use sysinfo::{CpuExt, System, SystemExt};
use colored::Colorize;
use directories::ProjectDirs;
use std::fs::read_to_string;
use ini::Ini;

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

    let proj_dirs = ProjectDirs::from("com", "CodMan",  "BentRat");
    // Linux:   /home/alice/.config/BentRat
    // Windows: C:\Users\Alice\AppData\Roaming\CodMan\BentRat

    let binding = proj_dirs.expect("ha");
    let path_to_conf = binding.config_dir().to_str().unwrap();
    config_creator::check_and_fix_config(path_to_conf);
    let mut ascii_line;
    let mut which_ascii;
    
    let i = Ini::load_from_file(format!("{}/Config.ini", path_to_conf)).unwrap();

    //COLORS
    let mut section = i.section(Some("accent_color")).unwrap();
    let mut sect_r = section.get("r").unwrap().to_string();
    let mut sect_g = section.get("g").unwrap().to_string();
    let mut sect_b = section.get("b").unwrap().to_string();

    let mut syscol;
    let acccol: [u8; 3] = [sect_r.parse::<u8>().unwrap(), sect_g.parse::<u8>().unwrap(), sect_b.parse::<u8>().unwrap()];
    
    section = i.section(Some("good_color")).unwrap();
    sect_r = section.get("r").unwrap().to_string();
    sect_g = section.get("g").unwrap().to_string();
    sect_b = section.get("b").unwrap().to_string();

    let goocol: [u8; 3] = [sect_r.parse::<u8>().unwrap(), sect_g.parse::<u8>().unwrap(), sect_b.parse::<u8>().unwrap()];
    
    section = i.section(Some("ok_color")).unwrap();
    sect_r = section.get("r").unwrap().to_string();
    sect_g = section.get("g").unwrap().to_string();
    sect_b = section.get("b").unwrap().to_string();
    
    let okcol: [u8; 3] = [sect_r.parse::<u8>().unwrap(), sect_g.parse::<u8>().unwrap(), sect_b.parse::<u8>().unwrap()];
    
    section = i.section(Some("bad_color")).unwrap();
    sect_r = section.get("r").unwrap().to_string();
    sect_g = section.get("g").unwrap().to_string();
    sect_b = section.get("b").unwrap().to_string();

    let badcol: [u8; 3] = [sect_r.parse::<u8>().unwrap(), sect_g.parse::<u8>().unwrap(), sect_b.parse::<u8>().unwrap()];

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
    section = i.section(Some("system_color")).unwrap();
    let use_custom_colors = section.get("use_custom_color").unwrap().to_string();
    if use_custom_colors == "true" {
        sect_r = section.get("r").unwrap().to_string();
        sect_g = section.get("g").unwrap().to_string();
        sect_b = section.get("b").unwrap().to_string();
        syscol = [sect_r.parse::<u8>().unwrap(), sect_g.parse::<u8>().unwrap(), sect_b.parse::<u8>().unwrap()];
    }

    let misc_section = i.section(Some("misc")).unwrap();
    if misc_section.get("use_custom_ascii_art").unwrap().to_string() == "true" {
        which_ascii = misc_section.get("custom_ascii_art").unwrap();
    }
    let lines = read_lines(format!("{}/{}.txt", path_to_conf, which_ascii).as_str());

    section = i.section(Some("stats")).unwrap();
    let start_line = section.get("start_line").unwrap().to_string().parse::<i64>().unwrap();
    let show_system_name = section.get("show_system_name").unwrap().to_string();
    let show_cpu_usage = section.get("show_cpu_usage").unwrap().to_string();
    let show_ram_capacity = section.get("show_ram_capacity").unwrap().to_string();
    let show_ram_usage = section.get("show_ram_usage").unwrap().to_string();

    loop {
        ascii_line = 0;
        if Command::new("clear").status().unwrap().success() {
            sys.refresh_all();
            if start_line > 0 {
                for _i in 0..start_line {
                    println!("{}\t", lines[ascii_line].truecolor(syscol[0], syscol[1], syscol[2]));
                    ascii_line += 1;
                }

            }
            //System
            if show_system_name == "true" {
                if ascii_line < lines.len() { print!("{}\t", lines[ascii_line].truecolor(syscol[0], syscol[1], syscol[2]));
                    ascii_line += 1;
                }
                println!("{}\t{}", "System name:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), sys_name_n_ver.truecolor(syscol[0], syscol[1], syscol[2]));
            }
            
            //CPU
            if show_cpu_usage == "true" {
                for cpu in sys.cpus() {
                    if ascii_line < lines.len() { print!("{}\t", lines[ascii_line].truecolor(syscol[0], syscol[1], syscol[2]));
                        ascii_line += 1;
                    }
                    if cpu.cpu_usage().floor() < 50.0 {
                        println!("{}{}\t{}", cpu.name().truecolor(acccol[0], acccol[1], acccol[2]).bold(), " usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} %", cpu.cpu_usage().floor()).truecolor(goocol[0], goocol[1], goocol[2]));
                    } else if cpu.cpu_usage().floor() > 50.0 && cpu.cpu_usage().floor() < 100.0 {
                        println!("{}{}\t{}", cpu.name().truecolor(acccol[0], acccol[1], acccol[2]).bold(), " usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} %", cpu.cpu_usage().floor()).truecolor(okcol[0], okcol [1], okcol[2]));
                    } else {
                        println!("{}{}\t{}", cpu.name().truecolor(acccol[0], acccol[1], acccol[2]).bold(), " usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} %", cpu.cpu_usage().floor()).truecolor(badcol[0], badcol[1], badcol[2]));
                    }
                }
            }

            //RAM
            if show_ram_capacity == "true" {
                if ascii_line < lines.len() { print!("{}\t", lines[ascii_line].truecolor(syscol[0], syscol[1], syscol[2]));
                    ascii_line += 1;
                }
                println!("{}\t{}", "RAM capacity:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} MB", sys.total_memory()/1048576).truecolor(goocol[0], goocol[1], goocol[2]));
            }
            
            if show_ram_usage == "true" {
                if ascii_line < lines.len() { print!("{}\t", lines[ascii_line].truecolor(syscol[0], syscol[1], syscol[2]));
                    ascii_line += 1;
                }
                ram_usage_in_perc = (sys.used_memory()*10)/sys.total_memory();
                if ram_usage_in_perc < 5 {
                    println!("{}\t{}", "RAM usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} MB", sys.used_memory()/1048576).truecolor(goocol[0], goocol[1], goocol[2]));
                } else if ram_usage_in_perc > 5 && ram_usage_in_perc < 10 {
                    println!("{}\t{}", "RAM usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} MB", sys.used_memory()/1048576).truecolor(okcol[0], okcol [1], okcol[2]));
                } else {
                    println!("{}\t{}", "RAM usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} MB", sys.used_memory()/1048576).truecolor(badcol[0], badcol[1], badcol[2]));
                }
            }
            
            if ascii_line < lines.len() {print!("{}\t",  lines[ascii_line].truecolor(syscol[0], syscol[1], syscol[2]));
                println!("{}", misc_section.get("exit_text").unwrap());
            }
            if ascii_line < lines.len() {
                for i in &lines[ascii_line..lines.len()] {
                    println!("{}\t", i.truecolor(syscol[0], syscol[1], syscol[2]));
                }
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