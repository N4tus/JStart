fn main() -> std::io::Result<()> {
    let launch_data = std::fs::read("launcher")?;
    let launch_str = String::from_utf8_lossy(&launch_data);
    let commands: Vec<_> = launch_str.split(" ").collect();
    let program = {
        if cfg!(windows) {
            "jdk/bin/java.exe"
        } else if cfg!(macos) {
            "jdk/Contents/Home/bin/java"
        } else if cfg!(unix) {
            "jdk/bin/java"
        } else {
            panic!("only windows, mac and unix is supported")
        }
    };
    let res = std::process::Command::new(program).args(&commands).spawn()?.wait()?;
    std::process::exit(res.code().unwrap_or(0));
}
