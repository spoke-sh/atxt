use std::process::ExitCode;

fn main() -> ExitCode {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    match atext::run_cli(&args, &atext::TerminalEnvironment::capture()) {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
