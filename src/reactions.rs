use std::process::Command;
use chrono::prelude::*;
use gethostname::gethostname;

// Tune these as desired. The defaults are mostly about capturing data, except for the disk reaction which includes pruning journals and restarting system logging,
// as well as killing processes and capturing system data.

pub fn diskcleaner1() {
    let host = gethostname();
    let output = Command::new("sh")
      .arg("-c")
      .arg("journalctl --vacuum-time=1d; aptitude clean; pkill -9 netcat; pkill -9 telnet; systemctl restart rsyslog; systemctl restart logrotate; dmesg; ps auxww; df -i; df -h; w;")
      .output()
      .expect("Failed to execute command");
    let readi: DateTime<Utc> = Utc::now();
    println!("[{} INFO] - {:?} {:?}", readi, host, output);
}

pub fn reportload1() {
    let host = gethostname();
    let output = Command::new("sh")
      .arg("-c")
      .arg("dmesg; ps auxww; df -i; df -h; w;")
      .output()
      .expect("Failed to execute command");
    let readi: DateTime<Utc> = Utc::now();
    println!("[{} INFO] - {:?} {:?}", readi, host, output);
}

pub fn reportnet1() {
    let host = gethostname();
    let output = Command::new("sh")
      .arg("-c")
      .arg("dmesg; ss -tulpan; ps auxww; ip a; df -i; df -h; w;")
      .output()
      .expect("Failed to execute command");
    let readi: DateTime<Utc> = Utc::now();
    println!("[{} INFO] - {:?} {:?}", readi, host, output);
}
