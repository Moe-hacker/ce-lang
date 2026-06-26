#[cfg(debug_assertions)]
use std::fs;
#[cfg(debug_assertions)]
use std::fs::File;
#[cfg(debug_assertions)]
use std::io::Read;
#[cfg(debug_assertions)]
use std::io::Seek;
#[cfg(debug_assertions)]
use std::io::Write;
#[cfg(debug_assertions)]
pub fn setup_panic_hook() {
    /*
     * Add a hook for testing build,
     * when any panic, print /proc/pid/fd,
     * and sleep to freeze forever to just wait user to kill it.
     */
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
#[cfg(debug_assertions)]
pub fn cwte_dump(mut input: File, file: &str) {
    /*
     * Dump a file, only for debugging.
     */
    let mut output_file = fs::File::create(file).expect("Failed to create output file");
    let mut content = Vec::new();
    input
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek input file");
    input
        .read_to_end(&mut content)
        .expect("Failed to read input file");
    output_file
        .write_all(&content)
        .expect("Failed to write to output file");
}
