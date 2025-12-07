mod syntax_tree;
mod tokenizer;

fn main() {
    let code = include_str!("../playground/main.mt");

    let tokens = tokenizer::tokenize(code);

    for token in &tokens {
        println!("{}", token);
    }

    print!("\n\n");

    let tree = syntax_tree::build_syntax_tree(&tokens);

    for node in tree {
        println!("{}", node);
    }
}
