mod audio_player;
mod gui;
mod sound;
mod workout_plan;

use crate::workout_plan::WorkoutPlan;

fn print_separator() {
    println!("-------------------------------------")
}

fn main() -> eframe::Result {
    if std::env::args().any(|argument| argument == "--cli") {
        let workout_plan = WorkoutPlan::from_stdin();

        print_separator();
        println!("Workout plan:");
        println!("{workout_plan}");
        print_separator();

        workout_plan.start_session();
        Ok(())
    } else {
        gui::run()
    }
}
