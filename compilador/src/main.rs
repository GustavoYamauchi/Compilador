use std::env;
use std::fs::File;
use std::io::prelude::*;

//# MT Variable
fn mt_assignment(list: &Vec<char>, mut index: usize) -> (usize, bool) {
	
	if list[index].is_alphanumeric() {
		index += 1;
		return mt_assignment(list, index);
	}
	else if list[index] == ' ' || list[index] == ';' || verify_operator(&list, index) {
		return (index, false);
	}
	else {
		return (index, true);
	}
	

}

//# MT Literal
fn mt_literal(state: i32, list: &Vec<char>, mut index: usize) -> (usize, bool) {

	match state {
		1 => { //se for um numero
			if list[index].is_numeric(){
				index += 1;
				return mt_literal(1, list, index);
			} else if list[index] == '.'{
				index += 1;
				return mt_literal(2, list, index);
			} else if list[index] == 'e' {
				index += 1;
				return mt_literal(3, list, index);
			} else if list[index] == ' ' || list[index] == ';' || verify_operator(&list, index) {
				return mt_literal(7, list, index);
			} else {
				index += 1;
				return mt_literal(5, list, index);
			}
		},
		2 =>{ //se for um ponto
			if list[index].is_numeric(){
				index += 1;
				return mt_literal(6, list, index);
			} else {
				index += 1;
				return mt_literal(5, list, index);
			}
		}, 
		3 =>{ //se for um e
			if list[index] == '+' || list[index] == '-'{
				index += 1;
				return mt_literal(4, list, index);
			} else {
				index += 1;
				return mt_literal(5, list, index);
			}
		}, 
		4 =>{ //se for um operador
			if list[index].is_numeric(){
				index += 1;
				return mt_literal(4, list, index);
			} else if list[index] == ' ' || list[index] == ';' || verify_operator(&list, index) {
				return mt_literal(7, list, index);
			} else {
				index += 1;
				return mt_literal(5, list, index);
			}
		}, 
		5 => { // se for UNK
			return (mt_unk(list, index), true);
		}, 
		6 =>{ // se for um numero dps do ponto 
			if list[index].is_numeric(){
				index += 1;
				return mt_literal(6, list, index);
			} else if list[index] == 'e' {
				index += 1;
				return mt_literal(3, list, index);
			} else if list[index] == ' ' || list[index] == ';' || verify_operator(&list, index) {
				return mt_literal(7, list, index);
			} else {
				index += 1;
				return mt_literal(5, list, index);
			}
		}, 
		7 =>{return (index, false)}, //Aceitacao
		_ =>return (index, true),
	}

}

//# MT Operator
fn mt_operator(state: i32, list: &Vec<char>, mut index: usize, last_char: char) -> (usize, bool) {

	match state {
		1 => { //se for o primeiro operador
			if list[index] == '*' || list[index] == '+' || list[index] == '-' || list[index] == '=' {
				index += 1;
				return mt_operator(2, list, index, list[index-1]);
			} else if list[index] == '(' || list[index] == ')' || list[index] == '/' {
				index += 1;
				return mt_operator(4, list, index, list[index-1]);
			} else {
				index += 1;
				return mt_operator(5, list, index, list[index-1]);
			}			
		},
		2 => { // se for o segundo operador
			if list[index] == last_char {
				index += 1;
				return mt_operator(3, list, index, list[index-1]);
			} if list[index] == ' ' || list[index] == ';' || list[index].is_alphanumeric() {
				return (index, false);
			} else {
				index += 1;
				return mt_operator(5, list, index, list[index-1]);
			}
			
		},
		3 => { // estado final 2 operadores
			if list[index] == ' ' || list[index] == ';' || list[index].is_alphanumeric() {
				return (index, false);
			} else {
				return mt_operator(5, list, index, list[index-1]);
			}
		},
		4 => { // estado final 1 operadores
			if list[index] == ' ' || list[index] == ';' || verify_operator(&list, index) || list[index].is_alphanumeric() {
				return (index, false);
			} else {
				return mt_operator(5, list, index, list[index-1]);
			}
		}
		5 => return (mt_unk(list, index), true),
		_ => return (index,true),
	}

}

// MT UNK
fn mt_unk(list: &Vec<char>, mut index:usize) -> usize{
	if list[index] == ' ' || list[index] == ';' {
		return index;
	} else {
		index += 1;
		return mt_unk(list, index);
	}
}

// fn verify line
fn linha(line_input: String) -> Vec<Token> {
	let mut aux = String::new();
	let mut list: Vec<char> = line_input.chars().collect();
	let mut list_tokens: Vec<Token> = Vec::new();
	
	let mut index = 0;
	if list[index] == '\n' {list.remove(0);}
	
	while list[index] != ';' {
		let state = 1;
		aux.push(list[index]);
		
		if list[index].is_alphabetic() {
			let (index_end, is_known) = mt_assignment(&list, index);
			if index_end == 0 {println!("Deu ruim1");}
			let value = list[index..index_end].into_iter().collect();

			let mut typ = 0;
			if is_known {
				typ = 4;
			} else {
				typ = 1;
			}
			
			list_tokens.push(create_token(typ, value));
			index = index_end;
		}
		else if list[index].is_numeric() {
			let (index_end, is_known) = mt_literal(1, &list, index);
			if index_end == 0 {println!("Deu ruim2");}
			let value = list[index..index_end].into_iter().collect();
			
			let mut typ = 0;
			if is_known {
				typ = 4;
			} else {
				typ = 2;
			}
			list_tokens.push(create_token(typ, value));
			index = index_end;
			
		}
		else if verify_operator(&list, index) {
			let (index_end, is_known) = mt_operator(state, &list, index, list[index]);
			if index_end == 0 {println!("Deu ruim3");}
			let value = list[index..index_end].into_iter().collect();
			
			let mut typ = 0;
			if is_known {
				typ = 4;
			} else {
				typ = 3;
			}
			
			list_tokens.push(create_token(typ, value));
			index = index_end;
		}
		else if list[index] == ' ' {
			aux.clear();
			index += 1;
		}
		else {
			let index_end = mt_unk(&list, index);
			if index_end == 0 {println!("Deu ruimUNK");}
			let value = list[index..index_end].into_iter().collect();
			
			list_tokens.push(create_token(4, value));
			index = index_end;
		}
	}

	list_tokens.push(create_token(5, list[index..index+1].into_iter().collect()));

	return list_tokens;
}

// Create/Config Token
fn create_token(typ: i32, value:String) -> Token {
	let mut token = Token{typ: "typ".to_string(), value: value};
	
	token.typ = match typ {
		1 => "Variable",
		2 => "Number",
		3 => "Operator",
		4 => "UNK",
		5 => "EndLine",
		_ => "404 type",
	}.to_string();

	return token;
}
// fn verify is operator
fn verify_operator(list: &Vec<char>, index: usize) -> bool {
	let list_operators = ['+', '-', '/', '*', '(', ')', '='];

	return list_operators.iter().find(|&&x| x == list[index]).is_some();
}



// struct main
struct Token {
	typ: String,
	value: String,
}



// fn main
fn main() {

    let mut f = File::open("./src/testes.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");


	let mut aux = String::new();
	let mut token = 0;
	let mut list_all_tokens: Vec<Token> = Vec::new();
	
	for elem in contents.chars(){
		if elem == ';' {token += 1};

		match token {
			0 => aux.push(elem),
			1 => {
				aux.push(elem); 
			  	list_all_tokens.append(&mut linha(aux.to_string())); 
			  	token = 0; aux = "".to_string();
			},
			_ => println!("Deuruim"),
		};
		// println!("{}", elem);
	}

	for tk in list_all_tokens {
		println!("Type: {} \nValue: {}\n", tk.typ, tk.value);
	}
    println!("Terminou");
}
