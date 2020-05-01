use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process::Child;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::process::exit;

fn main() {
    let root_config: Arc<Config> = Arc::new(get_config());
    let config = Arc::clone(&root_config);

    let root_clutter_session: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
    let active_clutter_session = Arc::clone(&root_clutter_session);
    let touch_thread = thread::spawn(move || {
        let f = File::open(&config.activator_event).unwrap();
        let mut reader = BufReader::new(f);

        let mut buffer = [0u8; 24];
        loop {
            reader.read(&mut buffer).unwrap();
            let mut session_option = active_clutter_session.lock().unwrap();
            if let Some(_) = *session_option {
            } else {
                match Command::new("/usr/bin/unclutter")
                    .arg("-idle")
                    .arg("0.0000001")
                    .arg("-root")
                    .spawn()
                {
                    Err(e) => println!("{}", e),
                    Ok(session) => *session_option = Some(session),
                }
            }
        }
    });

    let config = Arc::clone(&root_config);
    let active_clutter_session = Arc::clone(&root_clutter_session);
    let mouse_thread = thread::spawn(move || {
        let f = File::open(&config.deactivator_event).unwrap();
        let mut reader = BufReader::new(f);

        let mut buffer = [0u8; 24];
        loop {
            reader.read(&mut buffer).unwrap();
            let session_option: &mut Option<Child> = &mut *active_clutter_session.lock().unwrap();
            if let Some(session) = session_option {
                session.kill().unwrap();
                *session_option = None;
            } else {
            }
        }
    });
    touch_thread.join().unwrap();
    mouse_thread.join().unwrap();
}

struct Config {
    activator_event: String,
    deactivator_event: String,
}

impl Config {
    pub fn new(activator_event: String, deactivator_event: String) -> Config {
        Config {
            activator_event,
            deactivator_event,
        }
    }
}

fn get_config() -> Config {
    let args: Vec<String> = env::args().collect();
    for arg in env::args(){
        if arg.eq("--help"){
            println!("vanish-cursor by Alexander Strickner
            
Usage: # vanish-cursor <path-to-activator-event> <path-to-deactivator-event>
Example: # vanish-cursor /dev/input/event8 /dev/input/event5

The script has to be run as root.

To find the appropriate event-number, look at the /proc/bus/input/devices file.
There you have to find the entry for the touchscreen and use the event listed under 'Handlers' as activator.
For the deactivator event you have to look for the entry for the Touchpad.

Have fun using this tool!
");
            exit(0);
        }
    }

    if args.len() != 3 {
        panic!("incorrect number of arguments!");
    }
    let activator = args[1].to_string();
    let deactivator = args[2].to_string();

    if !Path::new(&activator).exists() || !Path::new(&deactivator).exists() {
        panic!("at least one event path is wrong!")
    }

    Config::new(activator, deactivator)
}
