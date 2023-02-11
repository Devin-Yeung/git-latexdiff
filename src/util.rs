use crossterm::style::Stylize;
use std::io::Write;
use which::which;

pub fn health_check() {
    // TODO: executable can also be passed by config
    let mut stdout = std::io::stdout();

    let bins = vec![
        "perl",
        "latexdiff",
        "latexdiff-so",
        "latexpand",
        "pdflatex",
        "xelatex",
        "luatex",
        "bibtex",
    ];

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
        writeln!(stdout, "Binary for {}: {}", bin, msg).unwrap();
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
    writeln!(stdout, "Ready? {}", msg).unwrap();
}
