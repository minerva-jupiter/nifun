use crossterm::event::{self, Event, KeyCode};
use jiff::{Span, Timestamp};
use notify_rust::Notification;
use std::{io::Write, io::stdout, process::Command, thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let interval: Span = Span::new().minutes(2);
    let start_time = Timestamp::now();
    let mut next_notif_time = start_time.checked_add(interval).unwrap();
    loop {
        let now = Timestamp::now();
        if now >= next_notif_time {
            notif();
            next_notif_time = next_notif_time.checked_add(interval).unwrap();
        } else {
            if event::poll(Duration::from_millis(50))?
                && let Event::Key(key_event) = event::read()?
                && key_event.code == KeyCode::Char('q')
            {
                println!("\n'q' detected. Exiting.");
                break;
            }
            print!("\r\x1b[2K{}", now - start_time);
            stdout().flush().unwrap();
            thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    Ok(())
}

fn notif() {
    Command::new("powershell")
        .args(["-c", "[console]::beep()"])
        .status()
        .expect("failed to execute process");

    Notification::new()
        .summary("COMing!!!")
        .body("Picture will be taken soon!")
        .show()
        .unwrap();
}
