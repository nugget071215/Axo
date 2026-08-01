use std::env;
use std::path::Path;
use std::process::Command;
use std::fs::{self, File, OpenOptions};
use std::io::Write;

fn main() {
    let config_dir = dirs::config_dir().expect("Could not find config directory").join("axo");
    let config_file = config_dir.join("pkgs.txt");

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: axo search <package>");
        return;
    }

    match args[1].as_str() {
        "search" => {
            check_import();
            let package = &args[2];
            let output = Command::new("nix")
                .args(["search", "nixpkgs", package])
                .output()
                .expect("Failed to run nix");

            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        "add" => {
            check_import();
            let package = &args[2];

            ensure_config(&config_dir, &config_file).expect("Could not prepare config directory or file");
            let contents = fs::read_to_string(&config_file).unwrap_or_default();

            if contents.lines().any(|line| line.trim() == package) {
                println!("Package '{}' is already installed.", package);
            } else {
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&config_file)
                    .expect("Could not open config file");

                writeln!(file, "{}", package).expect("Could not write to config file");

                let packages = read_packages(&config_file);
                generate_nix(packages);

                println!("Package '{}' has been installed. Please rebuild your system for the changes to take effect.", package);
            }
        }
        "remove" => {
            check_import();
            let package = &args[2];

            ensure_config(&config_dir, &config_file).expect("Could not prepare config directory or file");
            let contents = fs::read_to_string(&config_file).unwrap_or_default();

            if !contents.lines().any(|line| line.trim() == package) {
                println!("Package '{}' is not installed.", package);
            } else {
                let new_contents: String = contents
                    .lines()
                    .filter(|line| line.trim() != package)
                    .map(|line| format!("{}\n", line))
                    .collect();

                let mut file = File::create(&config_file).expect("Could not open config file");
                file.write_all(new_contents.as_bytes()).expect("Could not write to config file");

                let packages = read_packages(&config_file);
                generate_nix(packages);

                println!("Package '{}' has been removed. Please rebuild your system for the changes to take effect.", package);
            }
        }
        _ => {
            println!("Usage: axo search <package>");
        }
    }
}

fn ensure_config(config_dir: &Path, config_file: &Path) -> std::io::Result<()> {
    fs::create_dir_all(config_dir)?;
    if !config_file.exists() {
        File::create(config_file)?;
    }
    Ok(())
}

fn read_packages(config_file: &std::path::Path) -> Vec<String> {
    let contents = fs::read_to_string(config_file).unwrap_or_else(|_| String::new());

    contents
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

fn generate_nix(packages: Vec<String>) {
    let mut nix = String::from(
        "{ pkgs, ... }:\n\n{\n  environment.systemPackages = with pkgs; [\n",
    );

    for package in packages {
        let pkg = package.trim();

        if !pkg.is_empty() {
            nix.push_str("    ");
            nix.push_str(pkg);
            nix.push('\n');
        }
    }

    nix.push_str("  ];\n}\n");

    let output = Path::new("/etc/nixos/generated.nix");

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).expect("Could not create output directory");
    }

    fs::write(output, nix).expect("Failed to write generated.nix");
}

fn check_import () {
    let configuration_nix = std::fs::read_to_string("/etc/nixos/configuration.nix").expect("Could not read configuration.nix");

    if configuration_nix.contains("./generated.nix") {
        return;
    } else {
        println!(
            "configuration.nix does not import generated.nix.\n\nAdd this inside your imports list:\n\n  ./generated.nix"
        );    }
}