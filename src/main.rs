fn search_term(term: &str, letter: &str) {
    for (i, line) in letter.lines().enumerate() {
        if line.contains(term) {
            let line_num = i + 1;
            println!("{}: {}", line_num, line)
        }
    }
}

fn main() {
    let term = "picture";
    let quote = "\
Every face, every shop, bedroom window, public-hose, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek throught millions of pages?";
    search_term(term, quote);
}
