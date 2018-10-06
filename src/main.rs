fn main() {
    let syntax: [&str; 4] = ["list", "elements", "element", "NAME"];

    // sample code
    let lexer = Lexer::new("[1, 2, 3]", syntax);
    let token = Token::new(1, "hoge", &lexer);

    println!("token? : {}", token.to_string());
    // let parser = Parser::new(lexer);
    // parser.parse();
}

/// 仮定する文法
/// list: '[' elements ']';
/// elements: (',', element)* ;
/// element: NAME | list;
/// NAME: ('a'..'z'|'A'..'Z') + ;

// パターン1: 再帰的下向き認識器
// 文法で定義された規則一つひとつに対して同じ名前のメソッドを構築する
/*struct G;

impl G {
    fn list() {}
    fn elements() {}
    fn element() {}
    fn NAME() {}

    // Token を消費する match 関数
    fn match_token(token: &str) {
        /*match token {
            '
        }*/
    }
}*/

// パターン2: LL(1) 再帰的下向き字句解析器 (scanner, tokenizer, lexer)
struct Lexer<'a> {
    input: String,
    token_type_names: [&'a str; 4],
}

impl<'a> Lexer<'a> {
    fn new(text: &'a str, type_names: [&'a str; 4]) -> Lexer<'a> {
        Lexer {
            input: text.to_string(),
            token_type_names: type_names,
        }
    }

    fn get_token_name(&self, token_type: usize) -> &str {
        assert!(token_type < 4);

        self.token_type_names[token_type]
    }

    /*fn next_token() -> Token {
        Token {}
    }*/
}

struct Token<'a> {
    token_type: u32,
    text: String,
    lexer_ptr: &'a Lexer<'a>,
}

impl<'a> Token<'a> {
    fn new(token_type: u32, text: &str, lexer: &'a Lexer) -> Token<'a> {
        Token {
            token_type: token_type,
            text: text.to_string(),
            lexer_ptr: lexer
        }
    }

    fn to_string(&self) -> String {
        let type_name = self.lexer_ptr.get_token_name(self.token_type as usize);

        return "<'".to_string() + &self.text + "'," + type_name + ">";
    }
}