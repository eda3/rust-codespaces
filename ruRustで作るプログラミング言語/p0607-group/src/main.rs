// 値の種類を表す列挙型
// Num: 数値
// Op: 演算子
// Block: 式のブロック
#[derive(Debug, PartialEq, Eq)]
enum Value<'src> {
    Num(i32),
    Op(&'src str),
    Block(Vec<Value<'src>>),
}

// Value型の数値を取り出すためのメソッド
impl<'src> Value<'src> {
    fn as_num(&self) -> i32 {
        match self {
            Self::Num(val) => *val,             // 数値の場合はその値を返す
            _ => panic!("Value is not number"), // 数値以外の場合はパニック
        }
    }
}

fn main() {
    // 標準入力から一行ずつ読み込み、解析する
    for line in std::io::stdin().lines().flatten() {
        parse(&line);
    }
}

fn parse<'a>(line: &'a str) -> Vec<Value> {
    // スタックを初期化
    let mut stack = vec![];

    // 入力行を空白で分割し、配列に格納
    let input: Vec<_> = line.split(" ").collect();

    // 入力の参照を作成
    let mut words = &input[..];

    // 各単語を処理
    while let Some((&word, mut rest)) = words.split_first() {
        if word.is_empty() {
            break;
        }

        // "{"ならブロックの解析を行う
        if word == "{" {
            let value;
            (value, rest) = parse_block(rest);
            stack.push(value);
        } else if let Ok(parsed) = word.parse::<i32>() {
            // 数値ならスタックにプッシュ
            stack.push(Value::Num(parsed));
        } else {
            // 演算子ならスタックから2つの値を取り出し、計算
            match word {
                "+" => add(&mut stack),
                "-" => sub(&mut stack),
                "*" => mul(&mut stack),
                "/" => div(&mut stack),
                _ => panic!("{word:?} could not be parsed"),
            }
        }

        // 未処理の単語に更新
        words = rest;
    }

    // 結果をスタックとして表示
    println!("stack: {stack:?}");

    // スタックを返す
    stack
}

// ブロックの解析
fn parse_block<'src, 'a>(input: &'a [&'src str]) -> (Value<'src>, &'a [&'src str]) {
    let mut tokens = vec![];
    let mut words = input;

    // ブロック内の単語を処理
    while let Some((&word, mut rest)) = words.split_first() {
        if word.is_empty() {
            break;
        }
        if word == "{" {
            let value;
            (value, rest) = parse_block(rest);
            tokens.push(value);
        } else if word == "}" {
            return (Value::Block(tokens), rest);
        } else if let Ok(value) = word.parse::<i32>() {
            tokens.push(Value::Num(value));
        } else {
            tokens.push(Value::Op(word));
        }
        words = rest;
    }

    (Value::Block(tokens), words)
}

fn add(stack: &mut Vec<Value>) {
    let rhs = stack.pop().unwrap().as_num();
    let lhs = stack.pop().unwrap().as_num();
    stack.push(Value::Num(lhs + rhs));
}

fn sub(stack: &mut Vec<Value>) {
    let rhs = stack.pop().unwrap().as_num();
    let lhs = stack.pop().unwrap().as_num();
    stack.push(Value::Num(lhs - rhs));
}

fn mul(stack: &mut Vec<Value>) {
    let rhs = stack.pop().unwrap().as_num();
    let lhs = stack.pop().unwrap().as_num();
    stack.push(Value::Num(lhs * rhs));
}

fn div(stack: &mut Vec<Value>) {
    let rhs = stack.pop().unwrap().as_num();
    let lhs = stack.pop().unwrap().as_num();
    stack.push(Value::Num(lhs / rhs));
}
