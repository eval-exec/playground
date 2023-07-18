pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    fn send_ctrlc_later(duration: Duration) {
        std::thread::spawn(move || {
            std::thread::sleep(duration);

            // send CTRL_C event to myself on windows platform
            #[cfg(windows)]
            {
                let pid = std::process::id();
                println!("send CTRL_C event to myself on windows platform");
                unsafe {
                    winapi::um::wincon::GenerateConsoleCtrlEvent(
                        winapi::um::wincon::CTRL_C_EVENT,
                        pid,
                    );
                }
            }

            #[cfg(not(windows))]
            {
                eprintln!("not windows");
                std::process::exit(1);
            }
        });
    }

    #[test]
    fn basic() {
        let (sender, receiver) = std::sync::mpsc::channel();
        ctrlc::set_handler(move || sender.send(()).unwrap()).expect("Error setting Ctrl-C handler");

        send_ctrlc_later(Duration::from_secs(2));

        println!("Waiting for Ctrl-C...");
        receiver.recv().unwrap();
    }
}
