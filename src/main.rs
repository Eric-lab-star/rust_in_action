use regex::Regex;

fn main() {
    let regex = Regex::new("picture").unwrap();
    let quote = "Every face, every shop, bedroom window, public-house, and 
dark square is a picture feverishly turned--in serach of what?
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        let contains_substring = regex.find(line);
        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
