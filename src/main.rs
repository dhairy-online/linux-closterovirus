use rand::Rng; 
use std::process::Command;
use rand::seq::SliceRandom; 
use std::{thread, time};

struct clov;

impl clov {
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
    pub fn unwanted_files(name: &str) {
    loop {
        let millis = time::Duration::from_millis(10000);
        let cmd = Command::new("xdg-open")
                                .arg(&name)
                                .output()
                                .expect("Failed to execute command");
            println!("{}", std::str::from_utf8(&cmd.stdout).unwrap());
            thread::sleep(millis);
        }
    }
    pub fn create_unwanted_files(name: &str) {
        let cmd = Command::new("touch")
                                .arg(&name)
                                .output()
                                .expect("Failed to execute command");
            println!("{}", std::str::from_utf8(&cmd.stdout).unwrap());
    }
}

// FUNCTION "MAIN";
fn main() {
     clov::hack("Dhairy");    
     clov::command("touch", "000_cirsusviruS.sh");
     clov::delete("000_cirsusviruS.sh");
     clov::fake_bytes();
     clov::create_unwanted_files("closterovirus.exe");
     clov::unwanted_files(".")

}