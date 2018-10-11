fn main() {
    // 計算する式
    let source = "1+2+3+4+5";

    // 計算結果
    let mut result: isize = 0;
    // 数字を入れる変数
    let mut number: String = "".to_string();

    for i in 0..source.len() {
        let c = source.chars().nth(i).unwrap();
        if c.is_digit(10) {
            number += &c.to_string();
        } else {
            if c == '+' {
                result += number.parse::<isize>().unwrap();
            }
            number = "".to_string();
        }
    }

    // 計算結果表示
    println!("{}", result + number.parse::<isize>().unwrap());
}
