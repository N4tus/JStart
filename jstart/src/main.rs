#![windows_subsystem = "windows"]

fn main() -> std::io::Result<()> {
    let launch_data = std::fs::read("launcher")?;
    let app_home_path = std::env::current_dir()?;
    let launch_str = String::from_utf8_lossy(&launch_data).replace("%APP_HOME%", &format!("{}", app_home_path.parent().unwrap().display()));
    let commands: Vec<_> = launch_str.split(" ").collect();
    let program = {
        if cfg!(windows) {
            "jdk/bin/javaw.exe"
        } else if cfg!(macos) {
            "jdk/Contents/Home/bin/javaw"
        } else if cfg!(unix) {
            "jdk/bin/javaw"
        } else {
            panic!("only windows, mac and unix is supported")
        }
    };
    let res = std::process::Command::new(program).args(&commands).spawn()?.wait()?;
    std::process::exit(res.code().unwrap_or(0));
}
