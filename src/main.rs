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

    let _ = run(Params::new(Vec::new(), &env_args[1]));
}

fn run(args: Params) -> io::Result<()> {
    let Params { mut store, key } = args;

    let _ = files_discovery(
        resolve_abs_location("Videos").unwrap(),
        &mut store
    );

    let search_results = search(&filter_files(store, "jpg"), &key);

    println!("Search results for '{key}':\n");

    for (index, item) in search_results.iter().enumerate() {
        println!("{} - {:?}\n", index+1,item);
    }

    println!("Please select a file to open: ");

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

        let status = Command::new("dragon")
            .arg(&search_results[input - 1])
            .spawn()?
            .wait()?;

        println!("Player exited with {status}");

        break;
    }

    Ok(())
}
