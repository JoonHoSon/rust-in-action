#[derive(Debug)]
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];

    grains.push(Cereal::Rye);

    //drop(grains); // 주석처리 하지않을 경우 println! macro에서 borrow error 발생

    println!("{:?}", grains); // borrow of moved value: `grains` value borrowed here after move
}
