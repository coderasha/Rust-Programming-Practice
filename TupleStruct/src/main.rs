// ** Pass By Reference**
 struct Color{
   red:u8,
   green:u8,
   blue:u8,

 }

fn main() {
let blue= Color{red:0, green:0 ,blue:255 };
bg_color(&blue);
}
fn bg_color(c:&Color){
 println!("background is R:{} B:{} G:{}", c.red, c.blue , c.green); 
}