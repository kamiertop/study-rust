fn main(){
    let x:i32 = 10;
    let (a,b) = (1,2);
    println!("{}",x);
    println!("{}-{}",a,b);

    // 变量遮蔽
    let x = "hello";
    println!("x is {}", x);

    let x = [100,200,300];
    println!("x is {:#?}", x);

    type_derivation()
}


fn type_derivation() {
    let elem = 5u8;
    println!("{:?}", elem);

    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);

    let player_scores = [
        ("jack",20),("jane",10)
    ];
    let players: Vec<_> = player_scores.
        iter().
        map(|&(player, _score)|{
            player
        }).
        collect();
    println!("{:?}", players);
}

#[allow(unused,non_camel_case_types)]
type age = u32;

#[allow(unused)]
static GLOBAL: i32 = 0;

/*
#![feature(const_fn)]
fn const_value(){
    use std::sync::atomic::AtomicBool;
    static FLAG: AtomicBool = AtomicBool::new(true);
}
*/