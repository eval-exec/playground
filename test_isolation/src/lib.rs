#[cfg(test)]
mod tests {
    use std::time::Duration;

    fn print_my_pid_and_tid() {
        let pid = std::process::id();
        let tid = nix::unistd::gettid();

        println!("my pid is {}, tid is {}", pid, tid);
    }

    fn hard_job() {
        std::thread::sleep(Duration::from_secs(2));
    }

    macro_rules! generate_test_functions {
        ($name:ident) => {
            #[test]
            fn $name() {
                hard_job();
                print_my_pid_and_tid();
            }
        };
    }

    generate_test_functions!(test_0);
    generate_test_functions!(test_1);
    generate_test_functions!(test_2);
    generate_test_functions!(test_3);
    generate_test_functions!(test_4);
    generate_test_functions!(test_5);
    generate_test_functions!(test_6);
    generate_test_functions!(test_7);
    generate_test_functions!(test_8);
    generate_test_functions!(test_9);
    generate_test_functions!(test_10);
    generate_test_functions!(test_11);
    generate_test_functions!(test_12);
    generate_test_functions!(test_13);
    generate_test_functions!(test_14);
    generate_test_functions!(test_15);
    generate_test_functions!(test_16);
    generate_test_functions!(test_17);
    generate_test_functions!(test_18);
    generate_test_functions!(test_19);
}
