
fn main() -> std::io::Result<()>{
    let launch_data = std::fs::read("launcher")?;
    let launch_str = String::from_utf8_lossy(&launch_data);
    let commands: Vec<_> = launch_str.split(" ").collect();

    let res = std::process::Command::new("printer.bat").args(&commands).spawn()?.wait()?;
    std::process::exit(res.code().unwrap_or(0));
}
