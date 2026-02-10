mod config;
mod errors;
mod env_loader;
mod rules;
mod formatter;



use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "env-parser",
    about = "Validate .env files",
    version
)]
struct Args {
    
    path: String,

    #[arg(long)]
    strict: bool,

    #[arg(long, help = "Format the .env file (remove unnecessary quotes)")]
    format: bool,

    #[arg(long, help = "Write the formatted content back to file (use with --format)")]
    write: bool,
}

fn main() {
    let args = Args::parse();


    if args.format {
        match formatter::format_env_file(&args.path) {
            Ok(formatted) => {
                if args.write {
                    match formatter::write_formatted_file(&args.path, &formatted) {
                        Ok(_) => println!("File formatted and written successfully"),
                        Err(e) => {
                            eprintln!("Error writing formatted file: {}", e);
                            std::process::exit(1);
                        }
                    }
                } else {
                    println!("Formatted content:\n{}", formatted);
                }
            }
            Err(e) => {
                eprintln!("Error formatting file: {}", e);
                std::process::exit(1);
            }   
        }
    } else {
        println!("File not formatted. Use --format to format the file.");
        std::process::exit(1);
    }

    match env_loader::load_from_path(&args.path).and_then(|config|{
        rules::validate(&config)?;
        Ok(config)
    }) {
        Ok(config) => {
            println!("Config loaded successfully: {:?}", config);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
