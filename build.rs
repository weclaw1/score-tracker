use std::process::Command;

fn main() {
    let python_installed = Command::new("sh")
        .args(["-c", "command -v python3"])
        .status()
        .unwrap()
        .success();
    let pip_installed = Command::new("sh")
        .args(["-c", "command -v pip3"])
        .status()
        .unwrap()
        .success();
    let wget_installed = Command::new("sh")
        .args(["-c", "command -v wget"])
        .status()
        .unwrap()
        .success();

    if python_installed && pip_installed && wget_installed {
        let aiohttp_installed = Command::new("sh")
            .args(["-c", "pip3 show aiohttp"])
            .status()
            .unwrap()
            .success();

        let toml_installed = Command::new("sh")
            .args(["-c", "pip3 show toml"])
            .status()
            .unwrap()
            .success();

        if aiohttp_installed && toml_installed {
            Command::new("wget").arg("https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py").status().unwrap();
            Command::new("python3")
                .args([
                    "flatpak-cargo-generator.py",
                    "Cargo.lock",
                    "-o",
                    "src/resources/cargo-sources.json",
                ])
                .status()
                .unwrap();
            Command::new("rm")
                .arg("flatpak-cargo-generator.py")
                .status()
                .unwrap();
        }
    }

    println!("cargo:rerun-if-changed=Cargo.lock");
}
