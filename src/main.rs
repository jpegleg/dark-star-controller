use std::{thread, time, str, cmp, env};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

use sysinfo::{System};
use gethostname::gethostname;
use chrono::prelude::*;

extern crate nix;
extern crate systemstat;

use systemstat::{System as StatSys, Platform};
use nix::sys::statvfs::statvfs;

mod reactions;
mod stats;
mod util;
mod netstat2;

use stats::Stats;

const FS_SPEC: usize = 0;
const FS_FILE: usize = 1;

pub fn memz() -> String {
    let mut sys = System::new_all();
    sys.refresh_memory();
    let total_memory: f64 = sys.total_memory() as f64;
    let used_memory: f64 = sys.used_memory() as f64;
    let percmem: f64 = used_memory/total_memory;
    percmem.to_string()
}

pub fn dskroot() -> String {
    let file = match File::open("/proc/mounts") {
       Ok(f) => f,
       Err(e) => {
           eprintln!("ERROR: Could not open /proc/mounts - {}", e);
           process::exit(1);
       }
    };
    let reader = BufReader::new(&file);
    let mut stats: Vec<Stats> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let fields: Vec<&str> = line.split_whitespace().collect();
                let statvfs = match statvfs(fields[FS_FILE]) {
                    Ok(s) => s,
                    Err(_err) => {
                        continue;
                     }
                };
                let fsid = statvfs.filesystem_id();
                if stats.iter().any(|s| s.fsid == fsid) {
                    continue;
                }
                let size = statvfs.blocks() * statvfs.block_size();
                let avail = statvfs.blocks_available() * statvfs.block_size();
                let s = Stats::new(fields[FS_SPEC],  size, avail, fields[FS_FILE], fsid);
                stats.push(s);
            }
            Err(_err) => continue,
        }
    }

    stats.sort();

    for stat in stats {
        let percent = if stat.percent.is_nan() {
            "0".to_string()
        } else {
            format!("{:>5}", stat.percent)
        };
        if stat.mount == "/" {
            let star = percent.to_string();
            return star;
        };
    };
    "0".to_string()
}

pub fn dskvar() -> String {
    let file = match File::open("/proc/mounts") {
       Ok(f) => f,
       Err(e) => {
           eprintln!("ERROR: Could not open /proc/mounts - {}", e);
           process::exit(1);
       }
    };
    let reader = BufReader::new(&file);
    let mut stats: Vec<Stats> = Vec::new();
    let mut max_width = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let fields: Vec<&str> = line.split_whitespace().collect();
                let statvfs = match statvfs(fields[FS_FILE]) {
                    Ok(s) => s,
                    Err(_err) => {
                        continue;
                     }
                };
                let fsid = statvfs.filesystem_id();
                if stats.iter().any(|s| s.fsid == fsid) {
                    continue;
                }
                let size = statvfs.blocks() * statvfs.block_size();
                let avail = statvfs.blocks_available() * statvfs.block_size();
                let s = Stats::new(fields[FS_SPEC],  size, avail, fields[FS_FILE], fsid);
                max_width = cmp::max(max_width, s.filesystem.len());
                stats.push(s);
            }
            Err(_err) => continue,
        }
    }

    stats.sort();

    for stat in stats {
        let _percent = if stat.percent.is_nan() {
            "0".to_string()
        } else {
            format!("{:>5}", stat.percent)
        };
        if stat.mount == "/var" {
            let star = "{}".to_string();
            return star;
        };
    };
    "0".to_string()
}

pub fn loadavg() -> String {
    let syz = StatSys::new();
    match syz.load_average() {
        Ok(loadavg) => format!("{}", loadavg.one),
        Err(_e) => "0".to_string()
    }
}

pub fn procnum() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    let star = sys.processes().iter().count();
    star.to_string()
}

pub fn cpucount() -> f64 {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.cpus().len() as f64
}

fn compare_strings(str1: &str, str2: &f64) -> f64 {
    let mbo = { *str2 };
    if let (Ok(num1), num2) = (str1.parse::<f64>(), mbo) {
        if num1 == num2 {
            0.0
        } else if num1 > num2 {
            1.0
        } else {
            2.0
        }
    } else {
        255.0
    }
}

pub fn lowend(num: i32) -> i32 {
    if num < -2000000000 {
        -2000000000
    } else {
        num
    }
}

fn main () {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Set the log level with: \n\n -d for debug \n\n -w warn \n\n or -q or any other character for none");
        std::process::exit(1);
    }

    let mode = args[1].clone();

    let mut sim: [i32; 5] = [100, 100, 100, 100, 100];
    let mut marker1 = 0;
    let mut marker2 = 0;
    let mut marker3 = 0;
    let mut marker4 = 0;
    let mut marker5 = 0;
    let host = gethostname();
    let datestart: DateTime<Utc> = Utc::now();

    println!("[{} INFO] - {:?} - Started dark-star-controller", datestart, &host);

    loop {

        let mem = memz();
        let ce = compare_strings(&mem, &0.5);
        if ce == 1.0 {
            sim[0] -= 1;
            let newval = lowend(sim[0]);
            sim[0] = newval
        }

        if ce == 0.0 {
            sim[0] -= 1;
            let newval = lowend(sim[0]);
            sim[0] = newval
        }

        if ce == 2.0 {
            if sim[0] < 100 {
                sim[0] += 2000000100
            }
            if sim[0] > 100 {
                sim[0] = 100
            }
        }

        let lda = loadavg();
        let count = cpucount();
        let pe = compare_strings(&lda, &count);

        if pe == 1.0 {
            sim[1] -= 1;
            let newval = lowend(sim[1]);
            sim[1] = newval

        }

        if pe == 0.0 {
            sim[1] -= 1;
            let newval = lowend(sim[1]);
            sim[1] = newval

        }

        if pe == 2.0 {
            if sim[1] < 100 {
                sim[1] += 2000000100
            }
            if sim[1] > 100 {
                sim[1] = 100
            }

        }

        let disk = dskroot();
        let de = compare_strings(&disk, &90.0);

        if de == 1.0 {
            sim[2] -= 1;
            let newval = lowend(sim[2]);
            sim[2] = newval

        }

        if de == 0.0 {
            sim[2] -= 1;
            let newval = lowend(sim[2]);
            sim[2] = newval

        }

        if de == 2.0 {
            if sim[2] < 100 {
                sim[2] += 2000000100
            }
            if sim[2] > 100 {
                sim[2] = 100
            }

        }

        let diskt = dskvar();
        let dt = compare_strings(&diskt, &90.0);

        if dt == 1.0 {
            sim[2] -= 1;
            let newval = lowend(sim[2]);
            sim[2] = newval

        }

        if dt == 0.0 {
            sim[2] -= 1;
            let newval = lowend(sim[2]);
            sim[2] = newval

        }

        if dt == 2.0 {
            if sim[2] < 100 {
                sim[2] += 2000000100
            }
            if sim[2] > 100 {
                sim[2] = 100
            }

        }

        let neto = netstat2::netz().expect("Failed to gather socket info.");
        let net = neto.to_string();
        let ne = compare_strings(&net, &1000.0);

        if ne == 1.0 {
            sim[3] -= 1;
            let newval = lowend(sim[3]);
            sim[3] = newval

        }

        if ne == 0.0 {
            sim[3] -= 1;
            let newval = lowend(sim[3]);
            sim[3] = newval

        }

        if ne == 2.0 {
            if sim[3] < 100 {
                sim[3] += 100
            }
            if sim[3] > 100 {
                sim[3] = 100
            }
        }

        let proc = procnum();
        let ne = compare_strings(&proc, &2000.0);

        if ne == 1.0 {
            sim[4] -= 1;
            let newval = lowend(sim[4]);
            sim[4] = newval

        }

        if ne == 0.0 {
            sim[4] -= 1;
            let newval = lowend(sim[4]);
            sim[4] = newval

        }

        if ne == 2.0 {
            if sim[4] < 100 {
                sim[4] += 100
            }
            if sim[4] > 100 {
                sim[4] = 100
            }
        }

        if sim[2] == 100 {
            marker1 = 0;
        } else if marker1 == 2 {
             _ = 0;
        } else if mode == "-w" {
            let nim: DateTime<Utc> = Utc::now();
            println!("[{} WARN] - {:?} {:?} -> disk over threshold: {:?}", nim, &host, &sim, &disk);
        } else if mode == "-d" {
            let nim: DateTime<Utc> = Utc::now();
            println!("[{} WARN] - {:?} {:?} -> disk over threshold: / {:?} /var {:?}", nim, &host, &sim, &disk, &diskt);
        }

        if sim[1] == 100 {
            marker2 = 0;
        } else if marker2 == 2 {
            _ = 0;
        } else if mode == "-w" {
            let xim: DateTime<Utc> = Utc::now();
            println!("[{} WARN] - {:?} {:?} -> load average over threshold: {:?}", xim, &host, &sim, &lda);
        } else if mode == "-d" {
            let xir: DateTime<Utc> = Utc::now();
            println!("[{} WARN] - {:?} {:?} -> load average over threshold: {:?}", xir, &host, &sim, &lda);
        }

        if sim[0] == 100 {
            marker3 = 0;
        } else if marker3 == 2 {
             _ = 0;
        } else if mode == "-w" {
            let wim: DateTime<Utc> = Utc::now();
            println!("[{} WARN] - {:?} {:?} ->  RAM over threshold: {:?}", wim, &host, &sim, &mem);
        } else if mode == "-d" {
            let riu: DateTime<Utc> = Utc::now();
            println!("[{} WARN] - {:?} {:?} ->  RAM over threshold: {:?}", riu, &host, &sim, &mem);
        }


        if sim[3] == 100 {
            marker4 = 0;
        } else if marker4 == 2 {
             _ = 0;
        } else if mode == "-w" {
            let qim: DateTime<Utc> = Utc::now();
             println!("[{} WARN] - {:?} {:?} -> net connections over threshold: {:?}", qim, &host, &sim, &net);
        } else if mode == "-d" {
            let rim: DateTime<Utc> = Utc::now();
             println!("[{} WARN] - {:?} {:?} -> net connections over threshold: {:?}", rim, &host, &sim, &net);
        }

        if sim[4] == 100 {
            marker5 = 0;
        } else if marker5 == 2 {
            _ = 0;
        } else if mode == "-w" {
            let pim: DateTime<Utc> = Utc::now();
            println!("[{} WARN] - {:?} {:?} -> process count over threshold: {:?}", pim, &host, &sim, &proc);
        } else if mode == "-d" {
            let rim: DateTime<Utc> = Utc::now();
            println!("[{} WARN] - {:?} {:?} -> process count over threshold: {:?}", rim, &host, &sim, &proc);
        }

        if sim[2] < 0 {
           if mode == "-w" {
               let fim: DateTime<Utc> = Utc::now();
               println!("[{} WARN] - {:?} {:?} -> disk prolonged risk detected: {:?}", fim, &host, &sim, &disk);
           } else if mode == "-d" {
                let rym: DateTime<Utc> = Utc::now();
               println!("[{} WARN] - {:?} {:?} -> disk prolonged risk detected: / {:?} /var {:?} ", rym, &host, &sim, &disk, &diskt);
           }
           marker1 = 2;
        }

        if sim[1] < 0 {
           if mode == "-w" {
               let xir: DateTime<Utc> = Utc::now();
               println!("[{} WARN] - {:?} {:?} -> load prolonged risk detected: {:?}", xir, &host, &sim, &lda);
           } else if mode == "-d" {
               let rir: DateTime<Utc> = Utc::now();
               println!("[{} WARN] - {:?} {:?} -> load prolonged risk detected: {:?}", rir, &host, &sim, &lda);
           }
           marker2 = 2;
        }

        if sim[0] < 0 {
           if mode == "-w" {
               let wwm: DateTime<Utc> = Utc::now();
               println!("[{} WARN] - {:?} {:?} -> RAM prolonged risk detected: {:?}", wwm, &host, &sim, &mem);
           } else if mode == "-d" {
               let rwm: DateTime<Utc> = Utc::now();
               println!("[{} WARN] - {:?} {:?} -> RAM prolonged risk detected: {:?}", rwm, &host, &sim, &mem);
           }
           marker3 = 2;
        }

        if sim[3] < 0 {
           if mode == "-w" {
               let uwm: DateTime<Utc> = Utc::now();
               println!("[{} WARN] - {:?} {:?} -> net prolonged risk detected: {:?}", uwm, &host, &sim, &net);
           } else if mode == "-d" {
               let bwr: DateTime<Utc> = Utc::now();
               println!("[{} WARN] - {:?} {:?} -> net prolonged risk detected: {:?}", bwr, &host, &sim, &net);
           }
           marker4 = 2;
        }

        if sim[4] < 0 {
           if mode == "-w" {
               let pim: DateTime<Utc> = Utc::now();
               println!("[{} WARN] - {:?} {:?} -> process count over threshold: {:?}", pim, &host, &sim, &proc);
           } else if mode == "-d" {
               let rjm: DateTime<Utc> = Utc::now();
               println!("[{} WARN] - {:?} {:?} -> process count over threshold: {:?}", rjm, &host, &sim, &proc);
           }
           marker5 = 2;
        }

        if sim[2] < 0 && marker1 < 1 {

            let dskcont = thread::spawn(|| {
                reactions::diskcleaner1();
            });

            dskcont.join().unwrap();
            marker1 = 1;
        }

        if sim[1] < 75 && marker2 < 1 {

            let repcont = thread::spawn(|| {
                reactions::reportload1();
            });

            repcont.join().unwrap();
            marker2 = 1;

        }

        if sim[0] < 99 && marker3 < 1 {

            let memcont = thread::spawn(|| {
                reactions::reportload1();
            });

            memcont.join().unwrap();
            marker3 = 1;

        }

        if sim[3] < 99 && marker4 < 1 {

            let netcont = thread::spawn(|| {
                reactions::reportnet1();
            });

            netcont.join().unwrap();
            marker4 = 1;
        }

        if sim[4] < 99 && marker5 < 1 {

            let proccont = thread::spawn(|| {
                reactions::reportload1();
            });
            let netcont = thread::spawn(|| {
                reactions::reportnet1();
            });

            proccont.join().unwrap();
            netcont.join().unwrap();
            marker5 = 1;
        }

        if mode == "-d" {
            let pim: DateTime<Utc> = Utc::now();
            println!("[{} DEBUG] - {:?} {:?} -> disk: {:?} net: {:?} proc: {:?} load: {:?} mem: {:?}", pim, &host, &sim, &disk, &net, &proc, &lda, &mem);
        }


        thread::sleep(time::Duration::from_millis(1999));

   }
}
