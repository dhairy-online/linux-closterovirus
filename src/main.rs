#[warn(unused_imports)]
use notify_rust::Notification;
use rand::Rng; 
use std::process::Command;
use rand::seq::SliceRandom; 
use std::{thread, time};
use dialog::DialogBox;

struct Clov;

impl Clov {
    pub fn hack(target: &str) {
        println!("Hacking...");
        for _x in 0..24 {
            let millis = time::Duration::from_millis(10);
            let num: i64 = rand::thread_rng().gen_range(0..10000000000000);
            println!("{}", num);
            println!("{}", num);
            let vs = vec!["CODE: RED ", "CODE: DARK_RED", "CODE: BLUE", "CODE: ZERO NUM x86",
            "CODE: x86 null"];
            println!("{}", vs.choose(&mut rand::thread_rng()).unwrap()); 
            thread::sleep(millis);
    
        }
        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);
        
        println!("Deleted {}.", target);
        let some_millis = time::Duration::from_millis(10);
        thread::sleep(some_millis);
    }
    pub fn command( cmdl: &str,arg: &str) {
        let cmd = Command::new(&cmdl)
                            .arg(&arg)
                            .output()
                            .expect("Failed to execute command");
        println!("{}", std::str::from_utf8(&cmd.stdout).unwrap());
    }
    pub fn delete(file : &str) {
        
            let cmd = Command::new("rm")
                                .args(vec!["-rf" ,file])
                                .output()
                                .expect("Failed to execute command");
            println!("{}", std::str::from_utf8(&cmd.stderr).unwrap());
    }
    pub fn fake_bytes() {
        for _y in 1..40 {
            let add_virus = vec!["remove: ", "Add: "];
            let millis = time::Duration::from_millis(10);
            let x86bom  = rand::thread_rng().gen_range(0..2);
            println!("{}{}",add_virus.choose(&mut rand::thread_rng()).unwrap(), x86bom);
            thread::sleep(millis);
        }
    }
    // unwanted_files
    pub fn unwanted_files(name: &str, name2: &str) {
    loop {
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
}

// FUNCTION "MAIN";
fn main() {
      Clov::hack("Divy");    
      Clov::command("touch", "000_cirsusviruS.sh");
      Clov::delete("000_cirsusviruS.sh");
      Clov::fake_bytes();
     for _o in 0..4 {
     #[warn(unused_must_use)]
     let mut a = dialog::Question::new("window process exited eith code: 9_972");
     let mut b = dialog::Message::new("window process exited eith code: 9_970");
     let mut c = dialog::Message::new("window process exited eith code: 9_973");
         a.title("[X] ERROR");
         b.title("[X] s");
         c.title("[x] e");
       let asycl = thread::spawn( move || {
            a.show();
        });
        let asycl2 = thread::spawn( move || {
            b.show();
        });
        let asycl3 = thread::spawn( move || {
            c.show();
        });
        let asycl4 = thread::spawn( move || {
            Clov::unwanted_files(".", "..");
         });
        asycl.join().unwrap();
        asycl2.join().unwrap();
        asycl3.join().unwrap();
        asycl4.join().unwrap();
        Notification::new()
            .summary("System Error X86_gbtap")
            .body("You have executed the evil :3")
            .icon("virus")
            .show()
            .expect("ERRR");
       }
      Clov::create_files("closterovirus.exe");
      Clov::hack("SUSIBABABACKA")
}