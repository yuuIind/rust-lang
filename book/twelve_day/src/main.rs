const GIFTS : [&str; 12] = [
    "A Partridge in a Pear Tree",
    "Two Turtle Doves",
    "Three French Hens",
    "Four Calling Birds",
    "Five Golden Rings",
    "Six Geese a-Laying",
    "Seven Swans a-Swimming",
    "Eight Maids a-Milking",
    "Nine Ladies Dancing",
    "Ten Lords a-Leaping",
    "Eleven Pipers Piping",
    "Twelve Drummers Drumming",
];

const ORDINALS : [&str; 12] = [
    "First",
    "Second",
    "Third",
    "Fourth",
    "Fifth",
    "Sixth",
    "Seventh",
    "Eighth",
    "Ninth",
    "Tenth",
    "Eleventh",
    "Twelfth",
];

fn main() {
    for day in 0..12 {
        println!("On the {} day of Christmas my true love sent to me:", ORDINALS[day]);
        for gift in (0..day+1).rev() {
            println!("{}", GIFTS[gift]);
        }
        println!();
    }
}
