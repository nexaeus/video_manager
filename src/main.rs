use std::{ env, io,
    path::PathBuf,
    process::Command,
};
use video_manager::{
    resolve_abs_location,
    files_discovery,
    filter_files,
    search,
};

struct Params<'a> {
    store: Vec<PathBuf>,
    key: &'a str,
}

impl<'a> Params<'a> {
    fn new(store: Vec<PathBuf>, key: &'a str) -> Params<'a> {
        Params { store, key }
    }
}

fn main() {
    let env_args: Vec<String> = env::args().collect();

    if env_args.len() < 2 {
        println!("The search key was not specified!");
        return;
    }

    match env_args[1].as_str() {
        "--help" | "-h" => return print_help(),
        "-P" => {
            if env_args.len() <= 3 {
                println!("Requirements were not satisfied because some parameters were not given!");
                return;
            }

            run(Params::new(Vec::new(), &env_args[3]), Some(&env_args[2])).unwrap()
        },
        _ => run(Params::new(Vec::new(), &env_args[1]), None).unwrap(),
    }
}

fn run(args: Params, player: Option<&String>) -> io::Result<()> {
    let Params { mut store, key } = args;

    let _ = files_discovery(
        resolve_abs_location("Videos").unwrap(),
        &mut store
    );

    let search_results = search(&filter_files(store, "jpg"), &key);

    if search_results.len() == 0 {
        println!("\x1b[1;32mNo results found for '{key}'\x1b[0m]\n");
        return Ok(())
    } else {
        println!("\x1b[1;32mSearch results for '{key}':\x1b[0m\n");
    }

    for (index, item) in search_results.iter().enumerate() {
        println!("    {} - {:?}\n", index+1,item);
    }

    println!("\x1b[1;32mPlease select a file to open:\x1b[0m");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to get user input!");

        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid value!");
                continue
            },
        };

        let player = if let Some(player) = player {
            player
        } else {
            "dragon"
        };

        let status = Command::new(player)
            .arg(&search_results[input - 1])
            .spawn().expect("Error occured when attempting to open file!")
            .wait()?;

        println!("Player exited with {status}");

        break;
    }

    Ok(())
}

fn print_help() {
    println!("Videos manager\n");
    // \x1b[1;32m = Bold + Standard Green (Cargo style)
    // \x1b[0m  = Resets all styles
    println!("\x1b[1;32mUSAGE:\x1b[0m\n    videos-manager [OPTIONS] \n    videos-manager [OPTIONS] <KEY>\n");

    println!("\x1b[1;32mOPTIONS:\x1b[0m");
    println!("    -P <PLAYER>   Specify your desired media player");
    println!("    -h, --help              Print help information");
}

