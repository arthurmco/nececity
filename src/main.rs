mod logic;

fn main() {
    println!(
        "He works on the area {:?} and has {:?} instruction level",
        logic::WorkingArea::Education,
        logic::InstructionLevel::Intermediate
    );

    println!("Hello, world!");
}
