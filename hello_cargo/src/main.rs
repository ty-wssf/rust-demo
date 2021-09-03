use rand::Rng;
use std::cmp::Ordering;
use std::io; // prelude // trait

fn main( ) {
    println!("猜数游戏!!");
    let secret_number = rand::thread_rng().gen_range(1, 101); // 132 u32 164
    println!( "神秘数字是: {}", secret_number );

    loop {
        println!("猜测一个数" );

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        // match 模式匹配
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // 不关心错误，继续下次循环
            Err(_ ) => continue,
        };
        println!("你猜测的数是: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // ar'm 
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!( "You win!" );
                break ;
            }
        }
    }
}