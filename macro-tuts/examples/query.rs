use macro_tuts::query;

fn main() {
    query!(SELECT * FROM table WHERE id > 1024);
    hello()
}