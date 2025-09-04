fn main() {
    println!("制御フロー学習");
    println!("if文");
    if_statement();
    loop_processing();
}
fn if_statement() {
    let num = 3;

    println!("もしもnumが5より大きかった場合の結果");
    if num < 5 {
        println!("true");
    } else {
        println!("false");
    }

    println!("else if文");
    println!("割り切れる数を求める");
    let num = 4583685;

    if num % 4 == 0 {
        println!("4で割り切れる");
    } else if num % 3 == 0 {
        println!("3で割り切れる");
    } else if num % 2 == 0 {
        println!("2で割り切れる");
    } else {
        println!("条件の数値では割り切れません");
    }

    println!("let内でのifの使用");
    let check = true;
    let num = if check { 9 } else { 6 };
    println!("{}", num);
}

fn loop_processing() {
    loop {
        println!("loop処理");
        break;
    }

    let mut count = 0;
    'counting_up: loop {
        println!("count number{}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 0 {
                break;
            }
            if count == 2 {
                println!("カウント停止");
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count {}", count);
}
