use std::process::Command;

fn main() {
    //check for gcc, clang, msvc or cc

    if Command::new("gcc").arg("-v").status().is_ok() {
        Command::new("gcc")
            .arg("-c")
            .arg("src/terminal_size.c")
            .arg("-o")
            .arg("terminal_size.o")
            .status()
            .unwrap();
        Command::new("ar")
            .arg("crs")
            .arg("libterminal_size.a")
            .arg("terminal_size.o")
            .status()
            .unwrap();
    } else if Command::new("clang").arg("-v").status().is_ok() {
        Command::new("clang")
            .arg("-c")
            .arg("src/terminal_size.c")
            .arg("-o")
            .arg("terminal_size.o")
            .status()
            .unwrap();
        Command::new("ar")
            .arg("crs")
            .arg("libterminal_size.a")
            .arg("terminal_size.o")
            .status()
            .unwrap();
    } else if Command::new("cc").arg("-v").status().is_ok() {
        Command::new("cc")
            .arg("-c")
            .arg("src/terminal_size.c")
            .arg("-o")
            .arg("terminal_size.o")
            .status()
            .unwrap();
        Command::new("ar")
            .arg("crs")
            .arg("libterminal_size.a")
            .arg("terminal_size.o")
            .status()
            .unwrap();
    } else if cfg!(target_os = "windows") {
        Command::new("cl")
            .arg("/c")
            .arg("src/terminal_size.c")
            .status()
            .unwrap();
        Command::new("lib")
            .arg("/OUT:libterminal_size.a")
            .arg("terminal_size.obj")
            .status()
            .unwrap();
    } else {
        panic!("No C compiler found");
    }


    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=static=terminal_size");
}
