use std::fs::File;
use std::io::prelude::*;

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct CpuInfo {
    pub entries: Vec<CPU>,
}

#[allow(dead_code)]
impl CpuInfo {
    pub fn new() -> Result<CpuInfo, std::io::Error> {
        let mut file = File::open("/proc/cpuinfo")?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let mut cpu_info = CpuInfo::default();
        let mut cpu = CPU::default();
        for line in contents.lines() {
            if !line.is_empty() {
                let (key, value) = line.split_once(':').unwrap();
                let key = key.trim();
                
                match key {
                    "physical id" => cpu.id = {
                        match value.trim().parse() {
                            Ok(i) => i,
                            Err(_) => error("Failed to parse \"physical id\""),
                        }
                    },
                    "vendor_id" => cpu.vendor_id = value.trim().to_string(),
                    "model name" => cpu.model_name = value.trim().to_string(),
                    "cpu cores" => cpu.cores = {
                        match value.trim().parse() {
                            Ok(i) => i,
                            Err(_) => error("Failed to parse \"cpu cores\""),
                        }
                    },
                    "siblings" => cpu.threads = {
                        match value.trim().parse() {
                            Ok(i) => i, Err(_) => error("Failed to parse \"siblings\""),
                            
                        }
                    },
                    "flags" => cpu.flags = {
                        let mut flags = Vec::new();
                        value.split_whitespace().for_each(|s| {
                            flags.push(s.trim().to_string());
                        });
                        flags
                    },
                    "bugs" => cpu.bugs = {
                        let mut bugs = Vec::new();
                        value.split_whitespace().for_each(|s| {
                            bugs.push(s.trim().to_string());
                        });
                        bugs 
                    },
                    // End of entry
                    // Start filling a new CPU
                    "power management" => {
                        cpu_info.push(cpu);
                        cpu = CPU::default();
                    }
                    _ => (),
                }
            }
            
        }
        let mut found_ids: Vec<u32> = Vec::new();

        // Filter out all duplicate entries
        cpu_info.entries.retain(|cpu| {
            if found_ids.contains(&cpu.id) {
                false
            } else {
                found_ids.push(cpu.id);
                true
            }
        });

        Ok(cpu_info)
    }

    fn push(&mut self, cpu: CPU) { self.entries.push(cpu); }
}

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct CPU {
    pub id: u32,
    pub vendor_id: String,
    pub model_name: String,
    pub cores: u32,
    pub threads: u32,
    pub flags: Vec<String>,
    pub bugs: Vec<String>,
}

fn error(msg: &str) -> ! {
    eprintln!("{msg}");
    std::process::exit(1);
}
