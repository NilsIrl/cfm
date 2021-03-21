use regex::Regex;
use std::os::unix::process::CommandExt;
use std::{path::Path, process::Command};
use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt)]
#[structopt(setting = AppSettings::TrailingVarArg)]
struct App {
    #[structopt(short, long, value_delimiter = ":", env = "CDPATH")]
    path: Vec<String>,

    #[structopt(allow_hyphen_values = true, use_delimiter = false, required = true)]
    cmd: Vec<String>,
}

fn main() {
    let rule_regex = Regex::new(r"\A(\d*)([-+]?)([t])\z").unwrap();
    let app = App::from_args();
    let mut commands = Command::new(&app.cmd[0]);
    let paths = app.path.iter().map(Path::new).collect::<Vec<_>>();
    'outer: for option in app.cmd.iter().skip(1) {
        if Path::new(option).exists() {
            commands.arg(option);
            continue 'outer;
        }

        for path in &paths {
            let fullpath = path.join(option);
            if fullpath.exists() {
                commands.arg(fullpath);
                continue 'outer;
            }
        }

        // TODO use regex for full option or just for rule
        // Could fix the need to use nightly?
        if let Some((basepath, rule)) = option.rsplit_once(':') {
            // TODO if rule is invalid, what to do? panic or just add option to end of command
            let caps = rule_regex.captures(rule).unwrap();
            let count_capture = caps.get(1).unwrap();
            let count = if count_capture.start() == count_capture.end() {
                1
            } else {
                count_capture.as_str().parse().unwrap()
            };
            let reverse_capture = caps.get(2).unwrap();
            // TODO which one should be the default?
            let reverse = if reverse_capture.start() == reverse_capture.end() {
                false
            } else {
                if reverse_capture.as_str() == "+" {
                    false
                } else {
                    // must be "-"
                    true
                }
            };
            for path in &paths {
                let fullpath = path.join(basepath);
                if fullpath.exists() {
                    match caps.get(3).unwrap().as_str() {
                        "t" => {
                            if let Ok(dirlist) = fullpath.read_dir() {
                                let mut dirlist: Vec<_> =
                                    dirlist.map(|dir_entry| dir_entry.unwrap()).collect();
                                dirlist.sort_by_cached_key(|k| {
                                    k.metadata().unwrap().modified().unwrap()
                                });
                                if reverse {
                                    for dir in dirlist.iter().take(count) {
                                        commands.arg(dir.path());
                                    }
                                } else {
                                    for dir in dirlist.iter().rev().take(count) {
                                        commands.arg(dir.path());
                                    }
                                }
                                continue 'outer;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        commands.arg(option);
    }
    // TODO: print the command ran
    //dbg!(&commands);
    println!("{}", commands.exec());
}
