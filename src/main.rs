use std::{thread::sleep, time::Duration};

fn main() {
    env_logger::init();

    let mut reedline = reedline::Reedline::create();
    let prompt = reedline::DefaultPrompt::default();

    loop {
        let signal = reedline.read_line(&prompt);
        match signal {
            Ok(reedline::Signal::Success(buffer)) => {
                println!("We processed: {:?}", buffer);
                sleep(Duration::from_secs(2));
            }
            Ok(reedline::Signal::CtrlD) | Ok(reedline::Signal::CtrlC) => {
                println!("\nAborted!");
                break;
            }
            x => {
                println!("Event: {:?}", x);
            }
        }
    }
}
