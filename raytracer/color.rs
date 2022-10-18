use vec::Color;

pub fn write_color(color: &Color) -> String{
    let c = 255.99;
    format!("{} {} {}\n", (c*color.x).round(), (c*color.y).round(), (c*color.z).round())
}