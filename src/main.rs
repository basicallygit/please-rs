use std::fs::OpenOptions;
use std::path::Path;
use std::env;
use std::time;
use std::fs::{read_to_string, create_dir_all, File};
use std::io::{Write, stdout};

#[link(name = "terminal_size", kind = "static")]
extern "C" {
    fn getcols() -> i32;
    fn getlines() -> i32;
}

fn get_terminal_size() -> (usize, usize) {
    //! Returns the terminal size in columns and lines
    //! 
    //! # Safety
    //! 
    //! The return values of ioctl and GetConsoleScreenBufferInfo are checked for validity
    
    let (cols, lines) = unsafe {
        (getcols(), getlines())
    };
    if cols == -1 || lines == -1 {
        panic!("Could not get terminal size");
    }

    (cols as usize, lines as usize)
}

fn centre_text(text: String) {
    let (cols, _) = get_terminal_size();
    let len = text.len();

    let spaces = (cols - len) / 2;
    println!("{}{}", " ".repeat(spaces), text);
}

fn get_tasks(config_path: String) -> Vec<(String, bool)> {
    let mut tasks = Vec::new();

    read_to_string(format!("{}/tasks.txt", config_path)).unwrap().lines().for_each(|line| {
        let mut task = line.split(":");
        let name = task.next().unwrap().to_string();
        let done = task.next().unwrap().parse::<bool>().unwrap();

        tasks.push((name, done));
    });

    tasks
}

fn setup(home: &String) {
    if cfg!(target_os = "windows") {
        if !Path::new(&format!("{}/.config/please-rs", home)).exists() {
            create_dir_all(&format!("{}/.config/please-rs", home)).unwrap();
            File::create(&format!("{}/.config/please-rs/tasks.txt", home)).unwrap();
        }
    } else {
        let home = env::var("HOME").unwrap();

        if !Path::new(&format!("{}/.config/please-rs", home)).exists() {
            create_dir_all(&format!("{}/.config/please-rs", home)).unwrap();
            File::create(&format!("{}/.config/please-rs/tasks.txt", home)).unwrap();
        }
    }
}

fn getuser() -> String {
    if cfg!(target_os = "windows") {
        env::var("USERNAME").unwrap()
    } else {
        env::var("USER").unwrap()
    }
}

fn print_tasks(tasks: Vec<(String, bool)>) {
    if tasks.len() == 0 {
        print!("\x1b[38;5;119m");
        stdout().flush().unwrap();
        centre_text("You have no tasks!".to_string());
        print!("\x1b[0m");
        stdout().flush().unwrap();
    } else {
        print!("\x1b[38;5;241m");
        stdout().flush().unwrap();
        centre_text("Tasks".to_string());
        print!("\x1b[0m");
        stdout().flush().unwrap();

        let mut longest = 0;
        for task in tasks.iter() {
            if task.0.len() > longest {
                longest = task.0.len();
            }
        }  

        // top line
        let mut buffer = String::new();

        buffer.push_str(format!("+{}+{}+{}+", "-".repeat(5), "-".repeat(longest + 4), "-".repeat(3)).as_str());

        centre_text(buffer);
        // tasks
        let (cols, _) = get_terminal_size();
        for (i, task) in tasks.iter().enumerate() {
            //centre
            if cols / 2 != 0 {
                print!(" ");
            }
            print!("{}", " ".repeat(cols / 2 - (longest + 18) / 2));


            print!("|");
            print!("\x1b[38;5;219m");
            print!(" {:<3} ", i + 1);
            print!("\x1b[0m");
            print!("|");
            print!("{}", if task.1 { "\x1b[38;5;40m" } else { "\x1b[38;5;9m" });
            print!(" {:<width$} ", task.0, width = longest);
            print!("\x1b[0m");
            print!("  |");
            print!("{}", if task.1 { "\x1b[38;5;40m" } else { "\x1b[38;5;196m" });
            print!(" {}", if task.1 { "✅" } else { "❌" });
            print!("\x1b[0m");
            println!("|");

            if cols / 2 != 0 {
                print!(" ");
            }

            // middle line
            print!("{}", " ".repeat(cols / 2 - (longest + 18) / 2));
            print!("+");
            for _ in 0..5 {
                print!("-");
            }
            print!("+");
            for _ in 0..longest + 4 {
                print!("-");
            }
            print!("+");
            for _ in 0..3 {
                print!("-");
            }
            println!("+");
        }
    }
}

fn print_info(message: &str) {
    let (cols, _) = get_terminal_size();
    let start_cols = (cols - message.len()) / 2 - 1;
    let end_cols = if (cols - message.len()) % 2 == 0 {start_cols} else {start_cols + 1};

    println!("\x1b[48;5;119m\x1b[38;5;0m{} {} \x1b[38;5;0m{}\x1b[0m", " ".repeat(start_cols), message, " ".repeat(end_cols));
}

fn print_error(message: &str) {
    let (cols, _) = get_terminal_size();
    let start_cols = (cols - message.len()) / 2 - 1;
    let end_cols = if (cols - message.len()) % 2 == 0 {start_cols} else {start_cols + 1};

    eprintln!("\x1b[48;5;9m\x1b[38;5;0m{} {} \x1b[38;5;0m{}\x1b[0m", " ".repeat(start_cols), message, " ".repeat(end_cols));
}

fn main() {
    let home: String;
    if cfg!(target_os = "windows") {
        home = env::var("USERPROFILE").unwrap();
    } else {
        home = env::var("HOME").unwrap();
    }

    let (cols, _) = get_terminal_size();

    setup(&home); //runs no matter what to make sure the config directory exists

    let argv = env::args().collect::<Vec<String>>();
    let argc = argv.len();

    match argc {
        1 => {
            let tasks = get_tasks(format!("{}/.config/please-rs", home));
            let now = time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .unwrap();

            let hour = now.as_secs() / 3600 % 24;
            let minute = now.as_secs() / 60 % 60;

            let message = format!("Hello, {}! | It's {}:{}", getuser(), hour, if minute < 10 { format!("0{}", minute) } else { format!("{}", minute) });

            let start_cols = (cols - message.len()) / 2 - 1;
            let end_cols = if (cols - message.len()) % 2 == 0 {start_cols} else {start_cols + 1};
            println!("\x1b[38;5;11m{} \x1b[38;5;214m{} \x1b[38;5;11m{}\x1b[0m", "—".repeat(start_cols), message, "—".repeat(end_cols));

            print_tasks(tasks);
        }

        2 => {
            match argv[1].as_str() {
                "cleanall" => {
                    let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(format!("{}/.config/please-rs/tasks.txt", home))
                        .unwrap();

                    file.write_all("".as_bytes()).unwrap();
                    print_info("CLEANED ALL TASKS");
                },

                "clean" => {
                    let tasks = get_tasks(format!("{}/.config/please-rs", home));
                    //remove all finished tasks
                    let mut new_file_contents = String::new();
                    for task in tasks.iter() {
                        if !task.1 {
                            new_file_contents.push_str(format!("{}:false\n", task.0).as_str());
                        }
                    }

                    let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(format!("{}/.config/please-rs/tasks.txt", home))
                        .unwrap();

                    file.write_all(new_file_contents.as_bytes()).unwrap();

                    print_info("CLEANED FINISHED TASKS");
                },

                "list" => {
                    print_tasks(get_tasks(format!("{}/.config/please-rs", home)));
                },

                _ => {
                    print_error("INVALID COMMAND / ARGUMENTS");
                }
            }
        },

        _ => {
            let command = argv[1].as_str();
            match command {
                "add" => {
                    let mut file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(format!("{}/.config/please-rs/tasks.txt", home))
                        .unwrap();

                    let task = argv[2..].join(" ");
                    file.write_all(format!("{}:false\n", task).as_bytes()).unwrap();
                    let message = format!("ADDED TASK '{}'", task);
                    
                    print_info(message.as_str());
                },

                "remove" => {
                    let tasks = get_tasks(format!("{}/.config/please-rs", home));
                    let mut new_file_contents = String::new();
                    for (i, task) in tasks.iter().enumerate() {
                        if i != argv[2].parse::<usize>().unwrap() - 1 {
                            new_file_contents.push_str(format!("{}:{}\n", task.0, task.1).as_str());
                        }
                    }

                    let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(format!("{}/.config/please-rs/tasks.txt", home))
                        .unwrap();

                    file.write_all(new_file_contents.as_bytes()).unwrap();
                    let index = argv[2].parse::<usize>().unwrap() - 1;
                    if index < tasks.len() {
                        let message = format!("REMOVED TASK '{}'", tasks[index].0);
                        print_info(message.as_str());
                    } else {
                        print_error("INVALID TASK NUMBER");
                    }
                },
                
                "cleanall" => {
                    let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(format!("{}/.config/please-rs/tasks.txt", home))
                        .unwrap();

                    file.write_all("".as_bytes()).unwrap();
                    print_info("CLEANED ALL TASKS");
                },

                "do" => {
                    let tasks = get_tasks(format!("{}/.config/please-rs", home));
                    let mut new_file_contents = String::new();
                    for (i, task) in tasks.iter().enumerate() {
                        if i == argv[2].parse::<usize>().unwrap() - 1 {
                            new_file_contents.push_str(format!("{}:true", task.0).as_str());
                        } else {
                            new_file_contents.push_str(format!("{}:{}", task.0, task.1).as_str());
                        }
                        new_file_contents.push_str("\n");
                    }

                    let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(format!("{}/.config/please-rs/tasks.txt", home))
                        .unwrap();
                        
                    file.write_all(new_file_contents.as_bytes()).unwrap();

                    let index = argv[2].parse::<usize>().unwrap() - 1;

                    if index < tasks.len() {
                        let message = format!("COMPLETED TASK '{}'", tasks[index].0);
                        print_info(message.as_str());
                    } else {
                        print_error("INVALID TASK NUMBER");
                    }
                },

                "undo" => {
                    let tasks = get_tasks(format!("{}/.config/please-rs", home));
                    let mut new_file_contents = String::new();
                    for (i, task) in tasks.iter().enumerate() {
                        if i == argv[2].parse::<usize>().unwrap() - 1 {
                            new_file_contents.push_str(format!("{}:false", task.0).as_str());
                        } else {
                            new_file_contents.push_str(format!("{}:{}", task.0, task.1).as_str());
                        }
                        new_file_contents.push_str("\n");
                    }

                    let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(format!("{}/.config/please-rs/tasks.txt", home))
                        .unwrap();

                    file.write_all(new_file_contents.as_bytes()).unwrap();

                    let index = argv[2].parse::<usize>().unwrap() - 1;

                    if index < tasks.len() {
                        let message = format!("UNDONE TASK '{}'", tasks[index].0);
                        print_info(message.as_str());
                    } else {
                        print_error("INVALID TASK NUMBER");
                    }
                },

                _ => todo!(),
            }
        }
    }

    println!();
}