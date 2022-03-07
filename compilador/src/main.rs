use std::io::stdin;

struct Token {
    name: String,
    config: String,
    value: String,
    typ: String,
}


fn defineVar(token: String, index: u32){
    
    match index {
        0 => configVar(line),
        1 => nameAndTypeVar(line),
        2 => assignment(line),
        3 => valueVar(line),
        4 => "Final "
    }


}


fn main() {
    
    let listTokens:[Token] = [Token{name: "teste".to_string(), config: "teste".to_string(), value: "teste".to_string(), typ: "teste".to_string()}];
    let mut line = String::new();
    
    stdin().read_line(&mut line); 

    let mut tokens = line.split_whitespace();

    for (index, token) in tokens.enumerate(){
        defineVar(token, index);
    }
    // println!("{:?}", tokens(0));
    // println!("no of bytes read, {}", b1);
}
