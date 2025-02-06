use crate::exercise::{Exercise, Mode};
use crate::nargo_utils;
use console::style;

// Compile the given Exercise
pub fn compile_exercise(exercise: &Exercise) -> Result<String, ()> {
    progress!("Compiling {} exercise...", exercise);

    match nargo_utils::nargo_compile(&exercise.path) {
        Ok(output) => Ok(output),
        Err(error) => {
            eprintln!("{}", error);
            warn!("Compilation of {} failed! Please try again.", exercise);
            Err(())
        }
    }
}

// Execute the given Exercise
pub fn execute_exercise(exercise: &Exercise) -> Result<String, ()> {
    progress!("Executing {} exercise...", exercise);

    match nargo_utils::nargo_execute(&exercise.path) {
        Ok(output) => Ok(output),
        Err(error) => {
            eprintln!("{}", error);
            warn!("Execution of {} failed! Please try again.", exercise);
            Err(())
        }
    }
}

// Test the given Exercise
pub fn test_exercise(exercise: &Exercise) -> Result<String, ()> {
    progress!("Testing {} exercise...", exercise);

    match nargo_utils::nargo_test(&exercise.path) {
        Ok(output) => Ok(output),
        Err(error) => {
            warn!(
                "Testing of {} failed! Please try again. Here's the output:",
                exercise
            );
            eprintln!("{}", error);
            Err(())
        }
    }
}

pub fn print_exercise_output(output: String) {
    if !output.is_empty() {
        println!("    {} {output}", style("Output").green().bold());
    }
}

pub fn print_exercise_success(exercise: &Exercise) {
    match exercise.mode {
        Mode::Compile => success!("Successfully compiled {}!", exercise),
        Mode::Execute => success!("Successfully executed {}!", exercise),
        Mode::Test => success!("Successfully tested {}!", exercise),
    }
}

// Clears the terminal with an ANSI escape code
pub fn clear_screen() {
    println!("\x1Bc");
}

#[macro_export]
macro_rules! warn {
    ($fmt:expr, $($arg:tt)*) => {{
        use console::{style, Emoji};
        use std::env;
        let formatstr = format!($fmt, $($arg)*);
        println!();
        if env::var("NO_EMOJI").is_ok() {
            println!("{} {}", style("!").red(), style(formatstr).red());
        } else {
            println!(
                "{} {}",
                style(Emoji("⚠️ ", "!")).red(),
                style(formatstr).red()
            );
        }
        println!();
    }};
}

#[macro_export]
macro_rules! success {
    ($fmt:expr, $($arg:tt)*) => {{
        use console::{style, Emoji};
        use std::env;
        let formatstr = format!($fmt, $($arg)*);
        println!();
        if env::var("NO_EMOJI").is_ok() {
            println!("{} {}", style("✓").green(), style(formatstr).green());
        } else {
            println!(
                "{} {}",
                style(Emoji("✅", "✓")).green(),
                style(formatstr).green()
            );
        }
        println!();
    }};
}

#[macro_export]
macro_rules! progress {
    ($fmt:expr, $($arg:tt)*) => {{
        use console::{style, Emoji};
        use std::env;
        let formatstr = format!($fmt, $($arg)*);
        println!();
        if env::var("NO_EMOJI").is_ok() {
            println!("{} {}", style("○").yellow(), style(formatstr).yellow());
        } else {
            println!(
                "{} {}",
                style(Emoji("🟡", "○")).yellow(),
                style(formatstr).yellow()
            );
        }
        println!();
    }};
}
