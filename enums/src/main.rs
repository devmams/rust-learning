enum Color {
    White,
    Black,
    Yellow,
    Red
}

fn main() {
    let my_color:Color = Color::Yellow;

    match my_color {
        Color::White => println!("my color is White"),
        Color::Black => println!("my color is Black"),
        Color::Yellow => println!("my color is Yellow"),
        Color::Red => println!("my color is Red"),
    }
}