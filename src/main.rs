mod tokenizer;

fn main() {
    let code = include_str!("../playground/main.mt");

    let tokens = tokenizer::tokenize(code);

    for token in tokens {
        println!("{}", token);
    }
}
