/*
 * Now I scream. WTH is this QwQ?
 * Don't blame me QwQ, all rust code is written by LLMs,
 * and I have never learned rust in fact.
 */
use rustix::fs::{MemfdFlags, memfd_create};
use rustix::fs::{SealFlags, fcntl_add_seals};
use std::env;
use std::fs;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::os::fd::AsFd;
// Print a nautilus with file name, line number and content.
// The `::}` is a cwte TODO note.
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
    // Open a new fd for writing by line, with line number.
    let lines: Vec<&str> = content.lines().collect();
    let output_file = format!("{}.c", args[1]);
    let fd = memfd_create(
        "cwte_output",
        MemfdFlags::CLOEXEC | MemfdFlags::ALLOW_SEALING,
    )
    .expect("Failed to create memfd");
    let mut mfd_file = fs::File::from(fd);
    for (i, line) in lines.iter().enumerate() {
        // If the line contains `::}`, print the nautilus and skip this line.
        if line.contains("::}") {
            print_nautilus(&args[1], i + 1, line, false);
            // Replace ::} with empty string, and write the line to the output file.
            let fixed = line.replace("::}", "");
            writeln!(mfd_file, "{}", fixed).expect("Failed to write to file");
            continue;
        }
        // Or, write the line to the output file.
        writeln!(mfd_file, "{}", line).expect("Failed to write to file");
    }
    // Make the memfd immutable to prevent further modification.
    mfd_file.sync_all().expect("Failed to sync memfd");
    fcntl_add_seals(mfd_file.as_fd(), SealFlags::WRITE).expect("Failed to add seals to memfd");
    // Write the content of memfd to the output file.
    let mut output = fs::File::create(&output_file).expect("Failed to create output file");
    let mut memfd_content = Vec::new();
    mfd_file
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek memfd");
    mfd_file
        .read_to_end(&mut memfd_content)
        .expect("Failed to read memfd");
    output
        .write_all(&memfd_content)
        .expect("Failed to write to output file");
    println!(
        "Cwte processing completed. Output written to {}",
        output_file
    );
}
