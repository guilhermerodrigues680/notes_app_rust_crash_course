use std::{env, fs::OpenOptions, io::Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        Err(format!("Usage: {} <note text>", args[0]))?;
    }

    let new_note = &args[1];
    let now = chrono::Local::now().to_rfc3339();

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("notes.txt")?;

    println!("Nova nota:\n{}", new_note);

    // write the current time to the file
    file.write_all(b"<!-- ")?;
    file.write_all(now.as_bytes())?;
    file.write_all(b" -->\n")?;
    file.write_all(new_note.as_bytes())?;
    file.write_all(b"\n\n")?;

    Ok(())
}
