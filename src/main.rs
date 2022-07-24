// mod print;
// mod vars;
// mod types;
// mod string;
// mod pointer_ref;


// fn main() {
//     print::run();
//     vars::varun(); 
//     types::typrun();
//     string::str();
//     pointer_ref::point();
// }

enum Direction {

    up,
    down,
    left,
    right
}

fn main() {
 let player_direction:Direction = Direction::down;

 match player_direction {

    Direction::up => println!("The player id facing Up"),
    Direction::down => println!("The player id facing Down"),
    Direction::left => println!("The player id facing Left"),
    Direction::right => println!("The player id facing Right"),
 }


}