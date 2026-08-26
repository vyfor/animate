#[cfg(feature = "macros")]
mod enabled {
    use animate::{Clock, animate};
    use std::{
        io::{Write, stdout},
        thread,
        time::Duration,
    };

    #[animate]
    struct Counter {
        #[tween(duration = 400)]
        value: u32,
    }

    pub fn run() -> std::io::Result<()> {
        // new() is auto-generated
        let mut c = Counter::new(0);
        let mut clock = Clock::new();

        loop {
            let time = clock.advance(Duration::from_millis(8));
            // must be called at the start of each frame
            let activity = c.advance(time);

            let v = *c.value;
            if v == 0 {
                c.value.to(100);
            }

            print!("\rcounter: {v}");
            stdout().flush()?;

            if activity.finished() {
                break;
            }

            thread::sleep(Duration::from_millis(8));
        }

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    #[cfg(feature = "macros")]
    enabled::run()?;

    #[cfg(not(feature = "macros"))]
    println!("this example requires the macros feature");

    Ok(())
}
