pub fn parse_node(tokens: &Vec<Token>) -> Node {
    let mut stack: Vec<Vec<Token>> = Vec::new();

    for token in tokens {
        match token.lexeme.value.as_str() {
            "(" => stack.push(vec![token]),
            ")" => {
                if let Some(last) = stack.last_mut() {
                    last.push(token);
                }
                stack.pop();
            }
            _ => {
                if let Some(last) = stack.last_mut() {
                    last.push(token);
                }
            }
        }
    }
}
