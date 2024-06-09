fn main() {
    let fmt = "cd docker && ENTRY_PORT=8020 NAME=items docker compose up -d".to_string();

    let mut cmd = std::process::Command::new("sh");
    let cmd = cmd.arg("-c").arg(fmt);

    let child = cmd.spawn().unwrap();

    match child.wait_with_output() {
        Ok(output) => println!("exited with: {output:?}"),
        Err(e) => println!("error attempting to wait: {e}"),
    }
}
