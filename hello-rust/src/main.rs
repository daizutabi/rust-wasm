use ferris_says::say;

fn main() {
    let stdout = std::io::stdout();
    _ = say("Hello, world!", 20, stdout);
}
