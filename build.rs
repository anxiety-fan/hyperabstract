fn main() {
    let directory = std::fs::read_dir(r".\src\ui").unwrap();

    for entry in directory {
        if let Ok(f) = entry {
            slint_build::compile(f.path()).unwrap_or_default();
        }
    }
}