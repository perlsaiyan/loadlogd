extern crate log;
extern crate syslog;

use std::env;
use std::fs::File;
use std::io::Read;

use std::thread;
use std::time::Duration;
use syslog::{Facility, Formatter3164};

const LOADFILE: &str = "/proc/loadavg";

const BUFSIZ: usize = 1024;
const INTERVAL: u64 = 60;

fn main() {
    let args: Vec<String> = env::args().collect();
    let interval = if args.len() == 2 {
        args[1].parse::<u64>().unwrap_or(INTERVAL)
    } else {
        INTERVAL
    };

    // Child process
    //unsafe { libc::setsid() };
    env::set_current_dir("/").unwrap();

    loop {
        let formatter = Formatter3164 {
            facility: Facility::LOG_DAEMON,
            hostname: None,
            process: "loadlogd".into(),
            pid: 0,
        };

        match syslog::unix(formatter) {
            Err(e) => println!("impossible to connect to syslog: {:?}", e),
            Ok(mut writer) => {
                let mut buffer = String::with_capacity(BUFSIZ);
                if let Ok(mut file) = File::open(LOADFILE) {
                    if file.read_to_string(&mut buffer).is_ok() {
                        let load: Vec<&str> = buffer.split_whitespace().collect();

                        writer
                            .info(&format!(
                                "Load average: {} {} {}",
                                load[0], load[1], load[2]
                            ))
                            .unwrap();
                    }
                }
            }
        }
        thread::sleep(Duration::from_secs(interval));
    }
}
