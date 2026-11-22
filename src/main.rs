#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod audio_player;
mod gui;
mod sound;
mod workout_plan;

use crate::workout_plan::WorkoutPlan;
use eframe::Error;

fn main() -> eframe::Result {
    let is_cli_mode = std::env::args().any(|argument| argument == "--cli");
    if is_cli_mode {
        run_cli()
    } else {
        gui::run()
    }
}

fn run_cli() -> Result<(), Error> {
    let workout_plan = WorkoutPlan::from_stdin();

    print_separator();
    println!("Workout plan:");
    println!("{workout_plan}");
    print_separator();

    workout_plan.start_session();
    Ok(())
}

fn print_separator() {
    println!("-------------------------------------")
}
