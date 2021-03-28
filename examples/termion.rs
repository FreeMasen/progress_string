use std::thread::sleep;
use std::time::Duration;

const TOTAL: usize = 1000;

#[cfg(unix)]
fn main() {
    let mut bar = progress_string::BarBuilder::new()
        .total(TOTAL)
        .include_percent()
        .build();

    println!("starting the progress");
    for i in 0..TOTAL {
        bar.replace(i);
        print!(
            "{}{}",
            termion::cursor::Left(bar.get_last_width() as u16),
            bar.to_string()
        );
        sleep(Duration::from_millis(10));
    }
    println!("\ndone with progress");
}

#[cfg(not(unix))]
fn main() {}
