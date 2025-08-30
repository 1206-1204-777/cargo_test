fn main() {
    //データ型
    //整数型
    let a:i8 = 8; //8bit整数
    println!("8bit整数 {}",a);
    let b:i16 = 16;
    println!("16bit整数 {}",b);
    let c:i32 = 32;
    println!("32bit整数 {}",c);
    let d:i64 = 64;
    println!("64bit整数 {}",d);
    let e:i128 = 128;
    println!("128bit整数 {}",e);

    println!();

    //符号なし整数型 (符号なしはu)
    println!("符号なし整数型");
    let a1:u8 = 8;
    println!("8bit {}",a1);
    let b2:u16 = 16;
    println!("16bit {}",b2);

    println!();

    //浮動小数点型
    let x = 2.546;
    println!("64bit {}",x);
    let y:f32 = 3.4;
    println!("32bit {}",y);

    println!();

    println!("計算式");
    //計算式
    let sum =y + x;
    println!("{}",sum);
    
    println!();

    println!("複合型");

    let tup = (1,4.56,5);
    let (x,y,z) = tup;
    println!("{} {} {}",x,y,z);

    println!();

    println!("配列");

    let a=[5;4];
    println!("{}",a[3]);

    println!();

    function();
}

fn function(){
    println!("function!");
}