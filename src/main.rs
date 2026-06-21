fn print_nautilus(file: &str, line_no: usize, content: &str) {
    println!("Cwte tail at {} line {}:", file, line_no);
    println!(">>");
    println!(">>  {}", content);
    println!(">>");
    // Cooked by rust at the beginning, now I cry.
    // `}` should be `}}` in rust fmt.
    // I miss my cprintf now.
    println!("::}} Here's a nautilus, have an ice cream and write a fix, or it'll become a fossil QwQ");
}

fn main() {
    print_nautilus("test.ce", 4, "foo() ::};");
}
