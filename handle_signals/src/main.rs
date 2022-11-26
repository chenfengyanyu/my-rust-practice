use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{error::Error, thread, time::Duration};
// 为了对更多的 Unix 信号做出反应，可以使用 signal-hook
fn main() -> Result<(), Box<dyn Error>> {
    let mut signals =  Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
        }
    });

    // Following code does the actual work, and can be interrupted by pressing
    // Ctrl-C. As an example: Let's wait a few seconds.
    thread::sleep(Duration::from_secs(4));

    Ok(())
}

// 在实际的程序中，最好是在这个执行信号处理的程序中设置一个变量，然后在程序的各个位置进行检查。
// 例如，可以在信号处理中设置一个 Arc<AtomicBool> ，然后在热循环（hot loops）中，或者当等待一个线程时，你定时地检查它，并在当它变为 true 时中断（break）。
