/*
 * Now I scream. WTH is this QwQ?
 * Don't blame me QwQ, all rust code is written by LLMs,
 * and I have never learned rust in fact.
 */
use colored::*;
use rustix::fs::{MemfdFlags, memfd_create};
use rustix::fs::{SealFlags, fcntl_add_seals};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::os::fd::AsFd;

// Add a hook for testing build,
// when any panic, print /proc/pid/fd,
// and sleep to freeze forever to just wait user to kill it.
#[cfg(debug_assertions)]
fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("Panic occurred: {}", info);
        let pid = std::process::id();
        eprintln!("Listing /proc/{}/fd:", pid);
        if let Ok(entries) = fs::read_dir(format!("/proc/{}/fd", pid)) {
            for entry in entries.flatten() {
                if let Ok(target) = fs::read_link(entry.path()) {
                    eprintln!(
                        "{} -> {}",
                        entry.file_name().to_string_lossy(),
                        target.display()
                    );
                }
            }
        }
        eprintln!("Freezing forever. Waiting to be killed...");
        loop {
            std::thread::sleep(std::time::Duration::from_secs(3600));
        }
    }));
}

// Print a nautilus with file name, line number and content.
// The `::}` is a cwte TODO note.
fn print_nautilus(file: &str, line_no: usize, content: &str, enforce: bool) {
    println!(
        "\n{}{}{}{}:",
        "Cwte tail at ".yellow(),
        file.to_string().blue(),
        " line ".yellow(),
        line_no.to_string().blue()
    );
    println!("{}", ">>".yellow());
    println!("{}{}", ">>  ".yellow(), content.blue());
    println!("{}", ">>".yellow());
    println!(
        "{}",
        "::} Here's a nautilus, have an ice cream and write a fix,".yellow()
    );
    println!("{}", "    and don't left it to be a fossil QwQ\n".yellow());
    if enforce {
        // If enforce is true, panic to prevent compiling.
        // Cooked by rust at the beginning, now I cry.
        // `}` should be `}}` in rust fmt.
        // I miss my cprintf now.
        panic!("Cwte ::}} tail is enforced, you must fix this before compiling.");
    }
}
fn prepare_layer(mut input: File) -> File {
    /*
     * prepare, add ce_line_xx mark to each line.
     */
    // Seek to the beginning of the file.
    input
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek input file");
    // Read input to string.
    let mut content = String::new();
    input
        .read_to_string(&mut content)
        .expect("Failed to read input file");
    // memfd magic!
    let fd = memfd_create(
        "cwte_output",
        MemfdFlags::CLOEXEC | MemfdFlags::ALLOW_SEALING,
    )
    .expect("Failed to create memfd");
    let mut mfd_file = fs::File::from(fd);
    // Now, erase the `::}` in content, and print the nautilus for it.
    for (i, line) in content.lines().enumerate() {
        // Or, write the line to the output file.
        writeln!(mfd_file, "@ce_line_{}@{}", i + 1, line).expect("Failed to write to file");
    }
    // Make the memfd immutable to prevent further modification.
    mfd_file.sync_all().expect("Failed to sync memfd");
    fcntl_add_seals(mfd_file.as_fd(), SealFlags::WRITE).expect("Failed to add seals to memfd");
    // Return the memfd file for further processing.
    mfd_file
}
fn nautilus_layer(mut input: File, file: &str) -> File {
    /*
     * Nautilus mark ::} is cwte TODO mark.
     */
    // Seek to the beginning of the file.
    input
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek input file");
    // Read input to string.
    let mut content = String::new();
    input
        .read_to_string(&mut content)
        .expect("Failed to read input file");
    // memfd magic!
    let fd = memfd_create(
        "cwte_output",
        MemfdFlags::CLOEXEC | MemfdFlags::ALLOW_SEALING,
    )
    .expect("Failed to create memfd");
    let mut mfd_file = fs::File::from(fd);
    // Now, erase the `::}` in content, and print the nautilus for it.
    for (i, line) in content.lines().enumerate() {
        // If the line contains `::}`, print the nautilus and skip this line.
        if line.contains("::}") {
            print_nautilus(file, i + 1, line, false);
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
    // Return the memfd file for further processing.
    mfd_file
}
fn linter_layer(mut input: File, file: &str) -> File {
    /*
     * :D is cwte ignore forever mark.
     * This will bypass #[[ce_enforce(func)]] in the future.
     *
     */
    // Seek to the beginning of the file before reading.
    input
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek input file");
    // Read input to string.
    let mut content = String::new();
    input
        .read_to_string(&mut content)
        .expect("Failed to read input file");
    // memfd magic!
    let fd = memfd_create(
        "cwte_output",
        MemfdFlags::CLOEXEC | MemfdFlags::ALLOW_SEALING,
    )
    .expect("Failed to create memfd");
    let mut mfd_file = fs::File::from(fd);
    // Now, erase the `:D` in content, and print the nautilus for it.
    for (i, line) in content.lines().enumerate() {
        // If the line contains `:D`, print the nautilus and skip this line.
        if line.contains(":D") {
            // !FIXME
            // print we got :D at line i, and the content of this line.
            println!(
                "\n{}{}{}{}:",
                "Cwte linter at ".yellow(),
                file.to_string().blue(),
                " line ".yellow(),
                i.to_string().blue()
            );
            println!("{}", ">>".yellow());
            println!("{}{}", ">>  ".yellow(), line.blue());
            println!("{}", ">>".yellow());
            println!("{}", ":D you choose to ignore this.".yellow());
            //
            //
            // Replace :D with empty string, and write the line to the output file.
            let fixed = line.replace(":D", "");
            writeln!(mfd_file, "{}", fixed).expect("Failed to write to file");
            continue;
        }
        // Or, write the line to the output file.
        writeln!(mfd_file, "{}", line).expect("Failed to write to file");
    }
    // Make the memfd immutable to prevent further modification.
    mfd_file.sync_all().expect("Failed to sync memfd");
    fcntl_add_seals(mfd_file.as_fd(), SealFlags::WRITE).expect("Failed to add seals to memfd");
    // Return the memfd file for further processing.
    mfd_file
}
fn final_layer(mut input: File) -> File {
    /*
     * Final, remove @ce_line_xx@ mark.
     */
    // Seek to the beginning of the file.
    input
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek input file");
    // Read input to string.
    let mut content = String::new();
    input
        .read_to_string(&mut content)
        .expect("Failed to read input file");
    // memfd magic!
    let fd = memfd_create(
        "cwte_output",
        MemfdFlags::CLOEXEC | MemfdFlags::ALLOW_SEALING,
    )
    .expect("Failed to create memfd");
    let mut mfd_file = fs::File::from(fd);
    // Now, erase the `::}` in content, and print the nautilus for it.
    for line in content.lines() {
        // The line_no is now untrustable.
        // So we just match first @ and second @, and erase it.
        if let Some(start) = line.find('@') {
            // Check if the first character is @, if not, this line is unmarked, just write it to the output file.
            if line[start..].starts_with('@') {
                if let Some(end) = line[start + 1..].find('@') {
                    let fixed = format!("{}{}", &line[..start], &line[start + end + 2..]);
                    writeln!(mfd_file, "{}", fixed).expect("Failed to write to file");
                    continue;
                }
            }
        }
        // Or, write the line to the output file.
        writeln!(mfd_file, "{}", line).expect("Failed to write to file");
    }
    // Make the memfd immutable to prevent further modification.
    mfd_file.sync_all().expect("Failed to sync memfd");
    fcntl_add_seals(mfd_file.as_fd(), SealFlags::WRITE).expect("Failed to add seals to memfd");
    // Return the memfd file for further processing.
    mfd_file
}
fn main() {
    /*
     * We will never release any memfd file, kernel will help us do that.
     * Say thanks to the kernel, say thanks to memfd,
     * and have an ice cream.
     */
    #[cfg(debug_assertions)]
    setup_panic_hook();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }
    // Open the input file.
    let input_file = fs::File::open(&args[1]).expect("Failed to open input file");
    // Process the input file with prepare layer, and get the memfd file.
    let after_prepare = prepare_layer(input_file);
    // Process the input file with nautilus layer, and get the memfd file.
    let mut mfd_file = nautilus_layer(after_prepare, &args[1]);
    // Process the memfd file with linter layer, and get the new memfd file.
    mfd_file = linter_layer(mfd_file, &args[1]);
    // Process the memfd file with final layer, and get the new memfd file.
    mfd_file = final_layer(mfd_file);
    // Write the content of memfd to the output file.
    let output_file = format!("{}.c", args[1]);
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
        "{}{}{}",
        "\nCwte processing completed. Output written to ".green(),
        output_file.blue(),
        " >w<!!!".yellow()
    );
    println!("{}", "I hope I'm just a cute tail...".green());
}
