use std::process::Command;

fn main() {
    Command::new("npx")
        .args([
            "tailwindcss",
            "-i",
            "./input.css",
            "-o",
            "./assets/tailwind.css",
        ])
        .status()
        .unwrap();
}
