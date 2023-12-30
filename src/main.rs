#[derive(Debug)]
#[allow(dead_code)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

fn main() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    print_suit(&heart);
    print_suit(&diamond);

    println!("{:?}", heart);
    println!("{:?}", diamond);
    println!("index {:?}",heart as i32);
    println!("index {:?}",diamond as i32);

}

fn print_suit(card: &PokerSuit) {
    // 需要在定义 enum PokerSuit 的上面添加上 #[derive(Debug)]，否则会报 card 没有实现 Debug
    println!("{:?}",*card);
    // println!("index {:?}",card as i32);
}