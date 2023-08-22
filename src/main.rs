use std::process::Command;
use sysinfo::{CpuExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    let mut sys_name = format!("{:?}", sys.name());
    sys_name = str::replace(&sys_name, "Some(\"","");
    sys_name = str::replace(&sys_name, "\")","");
    let mut sys_ver = format!("{:?}", sys.os_version());
    sys_ver = str::replace(&sys_ver, "Some(\"","");
    sys_ver = str::replace(&sys_ver, "\")","");
    loop {
        if Command::new("clear").status().unwrap().success() {
            sys.refresh_all();
            println!("System name:\t{} {}", sys_name, sys_ver);
            for cpu in sys.cpus() { println!("{} usage:\t{} %", cpu.name(), cpu.cpu_usage()); }
            println!("RAM capacity:\t{} MB", sys.total_memory()/1048576);
            println!("RAM usage:\t{} MB", sys.used_memory()/1048576);
            println!("press [CTRL + C] to exit...");
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}
