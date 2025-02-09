use argh::FromArgs;
use console::Emoji;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::ffi::OsStr;
use std::fs;
use std::io::{self};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[macro_use]
mod ui;
mod exercise;
mod nargo_utils;
mod run;

mod utils;
mod verify;

use exercise::{Exercise, ExerciseList};
use nargo_utils::check_nargo_installation;
use utils::clear_screen;

const VERSION: &str = "0.1.0";

#[derive(FromArgs, PartialEq, Debug)]
/// noirlings is a collection of small exercises to get you used to writing Noir code
struct Args {
    /// show the executable version
    #[argh(switch, short = 'v')]
    version: bool,

    #[argh(subcommand)]
    nested: Option<Subcommands>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    Verify(VerifyArgs),
    Watch(WatchArgs),
    Run(RunArgs),
    Reset(ResetArgs),
    Hint(HintArgs),
    List(ListArgs),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "verify")]
/// Verifies all exercises according to the recommended order
struct VerifyArgs {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "watch")]
/// Reruns `verify` when files were edited
struct WatchArgs {
    #[argh(positional)]
    start: Option<String>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "run")]
/// Runs/Tests a single exercise
struct RunArgs {
    #[argh(positional)]
    name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "reset")]
/// Resets a single exercise using "git stash -- <filename>"
struct ResetArgs {
    #[argh(positional)]
    name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "hint")]
/// Returns a hint for the given exercise
struct HintArgs {
    #[argh(positional)]
    name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "list")]
/// Lists the exercises available in noirlings
struct ListArgs {
    #[argh(switch, short = 'p')]
    /// show only the paths of the exercises
    paths: bool,
}

enum WatchStatus {
    Finished,
    Unfinished,
}

fn main() {
    let args: Args = argh::from_env();

    if args.version {
        println!("v{VERSION}");
        std::process::exit(0);
    }

    if args.nested.is_none() {
        println!("\n{WELCOME}\n");
    }

    if !Path::new("info.toml").exists() {
        println!(
            "{} must be run from the noirlings directory",
            std::env::current_exe().unwrap().to_str().unwrap()
        );
        println!("Try `cd noirlings/`!");
        std::process::exit(1);
    }

    if !check_nargo_installation() {
        println!("We cannot find `nargo`.");
        println!("Please install Noir using `noirup` first.");
        println!("For instructions, check the README.");
        std::process::exit(1);
    }
    let toml_str = &fs::read_to_string("info.toml").unwrap();
    let exercises = toml::from_str::<ExerciseList>(toml_str).unwrap().exercises;

    let command = args.nested.unwrap_or_else(|| {
        println!("{DEFAULT_OUT}\n");
        std::process::exit(0);
    });

    match command {
        Subcommands::List(subargs) => {
            if !subargs.paths {
                println!("{:<17}\t{:<46}\t{:<7}", "Name", "Path", "Status");
            }
            let mut exercises_done = 0;
            exercises.iter().for_each(|e| {
                if e.looks_done() {
                    exercises_done += 1;
                }
                let status = if e.looks_done() { "Done" } else { "Pending" };
                if subargs.paths {
                    println!("{}", e.path.display());
                } else {
                    println!("{:<17}\t{:<46}\t{:<7}", e.name, e.path.display(), status);
                }
            });
            let percentage_progress = exercises_done as f32 / exercises.len() as f32 * 100.0;
            println!(
                "\nProgress: You completed {} / {} exercises ({:.1} %).",
                exercises_done,
                exercises.len(),
                percentage_progress
            );
        }

        Subcommands::Run(subargs) => {
            let exercise = find_exercise(&subargs.name, &exercises);
            run::run(exercise).unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Reset(subargs) => {
            let exercise = find_exercise(&subargs.name, &exercises);
            run::reset(exercise).unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Hint(subargs) => {
            let exercise = find_exercise(&subargs.name, &exercises);
            println!("{}", exercise.hint);
        }

        Subcommands::Verify(_) => {
            verify::verify(&exercises, (0, exercises.len()))
                .unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Watch(subargs) => {
            let watching = match subargs.start {
                Some(exercise) => {
                    let idx = exercises
                        .iter()
                        .position(|e| e.name == exercise)
                        .unwrap_or_else(|| {
                            println!("Could not find exercise {}", exercise);
                            std::process::exit(1)
                        });
                    watch(&exercises[idx..])
                }
                None => watch(&exercises),
            };

            match watching {
                Err(e) => {
                    println!("Error: Could not watch your progress. Error message was {e:?}.");
                    println!("Most likely you've run out of disk space or your 'inotify limit' has been reached.");
                    std::process::exit(1);
                }
                Ok(WatchStatus::Finished) => {
                    println!(
                        "{emoji} All exercises completed! {emoji}",
                        emoji = Emoji("🎉", "★")
                    );
                    println!("\n{FINISH_LINE}\n");
                }
                Ok(WatchStatus::Unfinished) => {
                    println!("We hope you're enjoying learning about Noir!");
                    println!("If you want to continue working on the exercises later, you can simply run `noirlings watch` again");
                }
            }
        }
    }
}

fn spawn_watch_shell(
    failed_exercise_hint: &Arc<Mutex<Option<String>>>,
    should_quit: Arc<AtomicBool>,
) {
    let failed_exercise_hint = Arc::clone(failed_exercise_hint);
    println!("\n\nWelcome to watch mode! You can type 'help' to get an overview of the commands you can use here.");
    thread::spawn(move || loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                match input {
                    "hint" => {
                        if let Some(hint) = &*failed_exercise_hint.lock().unwrap() {
                            println!("{hint}");
                        }
                    }
                    "clear" => clear_screen(),
                    "quit" => {
                        should_quit.store(true, Ordering::SeqCst);
                        println!("Bye!");
                    }
                    "help" => {
                        println!("Commands available to you in watch mode:");
                        println!("  hint  - prints the current exercise's hint");
                        println!("  clear - clears the screen");
                        println!("  quit  - quits watch mode");
                        println!("  help  - displays this help message");
                        println!();
                        println!("Watch mode automatically re-evaluates the current exercise");
                        println!("when you edit a file's contents.");
                    }
                    _ => println!("unknown command: {input}"),
                }
            }
            Err(error) => println!("error reading command: {error}"),
        }
    });
}

fn find_exercise<'a>(name: &str, exercises: &'a [Exercise]) -> &'a Exercise {
    if name.eq("next") {
        exercises
            .iter()
            .find(|e| !e.looks_done())
            .unwrap_or_else(|| {
                println!("🎉 Congratulations! You have done all the exercises!");
                println!("🔚 There are no more exercises to do next!");
                std::process::exit(1)
            })
    } else {
        exercises
            .iter()
            .find(|e| e.name == name)
            .unwrap_or_else(|| {
                println!("No exercise found for '{name}'!");
                std::process::exit(1)
            })
    }
}

fn watch(exercises: &[Exercise]) -> notify::Result<WatchStatus> {
    println!("Inside exercise method");
    let (tx, rx) = channel();
    let should_quit = Arc::new(AtomicBool::new(false));

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;
    watcher.watch(Path::new("./exercises"), RecursiveMode::Recursive)?;

    clear_screen();

    let failed_exercise_hint = Arc::new(Mutex::new(None::<String>));

    match verify::verify(exercises.iter(), (0, exercises.len())) {
        Ok(_) => return Ok(WatchStatus::Finished),
        Err(exercise) => {
            let mut failed_exercise_hint = failed_exercise_hint.lock().unwrap();
            *failed_exercise_hint = Some(exercise.hint.clone());
        }
    }

    spawn_watch_shell(&failed_exercise_hint, Arc::clone(&should_quit));

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => match event {
                DebouncedEvent::Create(b) | DebouncedEvent::Chmod(b) | DebouncedEvent::Write(b) => {
                    if b.extension() == Some(OsStr::new("nr")) && b.exists() {
                        let filepath = b.as_path().canonicalize().unwrap();
                        let pending_exercises = exercises
                            .iter()
                            .find(|e| filepath.ends_with(&e.path))
                            .into_iter()
                            .chain(
                                exercises
                                    .iter()
                                    .filter(|e| !e.looks_done() && !filepath.ends_with(&e.path)),
                            );
                        let num_done = exercises.iter().filter(|e| e.looks_done()).count();
                        match verify::verify(pending_exercises, (num_done, exercises.len())) {
                            Ok(_) => return Ok(WatchStatus::Finished),
                            Err(exercise) => {
                                let mut failed_exercise_hint = failed_exercise_hint.lock().unwrap();
                                *failed_exercise_hint = Some(exercise.hint.clone());
                            }
                        }
                    }
                }
                _ => {}
            },
            Err(RecvTimeoutError::Timeout) => {
                // Just check if we should quit
            }
            Err(e) => println!("watch error: {e:?}"),
        }

        if should_quit.load(Ordering::SeqCst) {
            return Ok(WatchStatus::Unfinished);
        }
    }
}

const DEFAULT_OUT: &str = r#"Thanks for installing noirlings!

Is this your first time? Don't worry, noirlings is made for beginners! We are
going to teach you a bunch of stuff about Noir and zero-knowledge proofs.

Here's how noirlings works:

1. To start noirlings run `cargo run -r --bin noirlings watch`
2. It'll automatically start with the first exercise. Don't get confused by
   error messages popping up as soon as you run noirlings! This is part of the
   exercise that you're supposed to solve, so open the exercise file in an editor
   and start your detective work!
3. If you're stuck on an exercise, there is a helpful hint you can view by
   typing `hint` (in watch mode), or running `cargo run -r --bin noirlings hint
   exercise_name`
4. When you have solved the exercise successfully, remove the `// I AM NOT DONE`
   comment to move on to the next exercise.
5. If an exercise doesn't make sense to you, please open an issue on GitHub!

Got all that? Great! To get started, run `noirlings watch` in order to get the
first exercise. Make sure to have your editor open!"#;

const FINISH_LINE: &str = r#"+----------------------------------------------------+
|          You made it to the finish line!          |
+--------------------------  ------------------------+

                          🎉 🎉 🎉

We hope you enjoyed learning about Noir and zero-knowledge proofs!
If you noticed any issues, please don't hesitate to report them."#;

const WELCOME: &str = r#"noirlings - An interactive tutorial to get started with Noir

  _ __   ___  (_)_ __ | (_)_ __   __ _ ___ 
 | '_ \ / _ \ | | '__|| | | '_ \ / _` / __|
 | | | | (_) || | |   | | | | | | (_| \__ \
 |_| |_|\___/ |_|_|   |_|_| |_| \__, |___/
                                 |___/      "#;
