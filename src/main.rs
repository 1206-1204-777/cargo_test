use std::io;
fn main() {
    println!("数当てゲーム開始！");
    println!("予想した数を入力してね");

    let mut guess = String::new();  //変数guessを作成 mutがあるのは可変

    let apples = 5;                    // mutがないため不変

    //入力を受け取る処理
    io::stdin()
    .read_line(&mut guess)
        .expect("Failed to read line");     // 行の読み込みに失敗しました

    println!("予想：{}", guess);
    
//数当てゲームで当てる数を生成


}
