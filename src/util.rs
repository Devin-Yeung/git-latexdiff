use crossterm::style::Stylize;
use which::which;
use std::io::Write;

fn health_check() -> std::io::Result<()> {
    // TODO: executable can also be passed by config
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    let bins = vec!["latexdiff", "latexdiff-so", "latexpand", "pdflatex", "xelatex", "luatex", "bibtex"];

    let mut ready = true;
    for bin in bins {
        let msg;
        match which(bin) {
            Ok(path) => {
                msg = path.display().to_string().green();
            }
            Err(_) => {
                msg = String::from("Not found in $PATH").red();
                ready = false;
            }
        }
        writeln!(stdout, "Binary for {}: {}", bin, msg)?;
    }

    let msg;
    match ready {
        true => {
            msg = "✓".green();
        }
        false => {
            msg = "✘".red();
        }
    }
    writeln!(stdout, "Ready? {}", msg)?;

    Ok(())
}
