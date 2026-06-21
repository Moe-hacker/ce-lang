use std::env;
use std::fs;
use std::io::Write;
// Print a nautilus with file name, line number and content.
// This is a TODO note.
fn print_nautilus(file: &str, line_no: usize, content: &str, enforce: bool) {
    println!("Cwte tail at {} line {}:", file, line_no);
    println!(">>");
    println!(">>  {}", content);
    println!(">>");
    // Cooked by rust at the beginning, now I cry.
    // `}` should be `}}` in rust fmt.
    // I miss my cprintf now.
    println!(
        "::}} Here's a nautilus, have an ice cream and write a fix, or it'll become a fossil QwQ"
    );
    if enforce {
        // If enforce is true, panic to prevent compiling.
        panic!("Cwte ::}} tail is enforced, you must fix this before compiling.");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }
    let content = fs::read_to_string(&args[1]).expect("Failed to read file");
    // Open a new file for writing by line, with line number.
    let lines: Vec<&str> = content.lines().collect();
    let output_file = format!("{}.c", args[1]);
    let mut output = fs::File::create(&output_file).expect("Failed to create file");
    for (i, line) in lines.iter().enumerate() {
        // If the line contains `::}`, print the nautilus and skip this line.
        if line.contains("::}") {
            print_nautilus(&args[1], i + 1, line, false);
            // Replace ::} with empty string, and write the line to the output file.
            let fixed = line.replace("::}", "");
            writeln!(output, "{}", fixed).expect("Failed to write to file");
            continue;
        }
        // Or, write the line to the output file.
        writeln!(output, "{}", line).expect("Failed to write to file");
    }
}
