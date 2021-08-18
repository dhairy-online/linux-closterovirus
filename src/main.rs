use crate::virus::sine_mouse_wave;
use crate::virus::Clov;
use autopilot;
use autopilot::mouse::Button::Left;
use dialog::DialogBox;
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

mod virus;

// FUNCTION "MAIN";
fn main() {
    virus::Clov::hack("Divy");
    Clov::command("touch", "000_cirsusviruS.sh");
    Clov::delete("000_cirsusviruS.sh");
    Clov::fake_bytes();
    Clov::create_files("closterovirus.exe");
    Clov::hack("SUSIBABABACKA");
    Clov::call_df();
}
