pub fn load_env() {
    let file = if let Ok(file) = std::fs::read_to_string(".env") {
        file
    } else {
        return;
    };

    for line in file.lines() {
        let (ident, val) = if let Some(r) = line.split_once('=') {
            r
        } else {
            return;
        };
        std::env::set_var(ident, val);
    }
}
