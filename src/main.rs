use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("数当てゲーム開始！");
    println!("予想した数を入力してね");

    let mut guess = String::new(); //変数guessを作成 mutがあるのは可変

    let apples = 5; // mutがないため不変

    //入力を受け取る処理
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // 行の読み込みに失敗しました

    //変数guessを数値に変換
    let guess: u32 = guess.trim().parse().expect("Please type a bumber!");
    println!("予想：{}", guess);

    //数当てゲームで当てる数を生成
    //乱数を生成
    let select_number = rand::thread_rng().gen_range(1..101);
    println!("The Seclet number:{}", select_number);

    //数を比較
    match guess.cmp(&select_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("Too match!"),
        Ordering::Greater => println!("Too big!"),
    }
}
