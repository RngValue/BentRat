use std::process::Command;
use sysinfo::{CpuExt, System, SystemExt};
use colored::Colorize;

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

    if sys_name == "Ubuntu" {
        syscol = [180, 180, 180];
        //syscol = [255, 100, 0];
    } else if sys_name == "Windows" {
        syscol = [40, 198, 201];
    } else {
        syscol = [180, 180, 180];
    }

    loop {
        if Command::new("clear").status().unwrap().success() {
            sys.refresh_all();
            //System
            println!("{}\t{}", "System name:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), sys_name_n_ver.truecolor(syscol[0], syscol[1], syscol[2]));
            
            //CPU
            for cpu in sys.cpus() {
                if cpu.cpu_usage().floor() < 50.0 {
                    println!("{}{}\t{}", cpu.name().truecolor(acccol[0], acccol[1], acccol[2]).bold(), " usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} %", cpu.cpu_usage().floor()).truecolor(goocol[0], goocol[1], goocol[2]));
                } else if cpu.cpu_usage().floor() > 50.0 && cpu.cpu_usage().floor() < 100.0 {
                    println!("{}{}\t{}", cpu.name().truecolor(acccol[0], acccol[1], acccol[2]).bold(), " usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} %", cpu.cpu_usage().floor()).truecolor(okcol[0], okcol [1], okcol[2]));
                } else {
                    println!("{}{}\t{}", cpu.name().truecolor(acccol[0], acccol[1], acccol[2]).bold(), " usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} %", cpu.cpu_usage().floor()).truecolor(badcol[0], badcol[1], badcol[2]));
                }
            }

            //RAM
            println!("{}\t{}", "RAM capacity:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} MB", sys.total_memory()/1048576).truecolor(goocol[0], goocol[1], goocol[2]));
            
            ram_usage_in_perc = (sys.used_memory()*10)/sys.total_memory();
            if ram_usage_in_perc < 5 {
                println!("{}\t{}", "RAM usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} MB", sys.used_memory()/1048576).truecolor(goocol[0], goocol[1], goocol[2]));
            } else if ram_usage_in_perc > 5 && ram_usage_in_perc < 10 {
                println!("{}\t{}", "RAM usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} MB", sys.used_memory()/1048576).truecolor(okcol[0], okcol [1], okcol[2]));
            } else {
                println!("{}\t{}", "RAM usage:".truecolor(acccol[0], acccol[1], acccol[2]).bold(), format!("{} MB", sys.used_memory()/1048576).truecolor(badcol[0], badcol[1], badcol[2]));
            }
            
            println!("press [CTRL + C] to exit...");
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}
