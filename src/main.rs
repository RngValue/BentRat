use std::process::Command;
use sysinfo::{CpuExt, System, SystemExt, };
//NetworkExt, NetworksExt, ProcessExt, CpuExt, 
fn main() {
    let mut sys = System::new_all();
    loop {
        if Command::new("clear").status().unwrap().success() {
            sys.refresh_all();
            println!("System name: {:?}", sys.name());
            for cpu in sys.cpus() {
                println!("{} usage: {} %;", cpu.name(), cpu.cpu_usage());
            }
            println!("RAM: {} MB", sys.total_memory()/1048576);
            println!("RAM usage: {} MB", sys.used_memory()/1048576);
            println!("press ctr+c");
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}
