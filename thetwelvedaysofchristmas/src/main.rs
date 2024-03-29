fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let presents = [
        "a partridge in a pear tree.",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ];


    for i in 0..12 {
        let day = days[i];
        let mut present = String::new();
        for j in (0..=i).rev() {
            if j == 0 && i != 0 {
                present.push_str("And ");
            }
            present.push_str(presents[j]);
            if j > 0 {
                present.push_str(",\n");
            } else if i != 0 {
                present.push_str("\n");
            } else if j == 0 && i == 0{
                present.push_str("\n");
            }
        }
        println!("On the {} day of Christmas,\nmy true love gave to me\n{}", day, present);
    }
}