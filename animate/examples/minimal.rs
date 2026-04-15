use animate::animate;
use std::{
    io::{Write, stdout},
    thread,
    time::Duration,
};

#[animate]
struct Counter {
    #[once(duration = 400)]
    value: u32,
}

fn main() -> std::io::Result<()> {
    // new() is auto-generated
    let mut c = Counter::new(0);

    loop {
        // must be called at the start of each frame
        animate::tick(8);

        let v = *c.value;
        if v == 0 {
            c.value.set(100);
        }

        print!("\rcounter: {v}");
        stdout().flush()?;

        if v == 100 {
            break;
        }

        thread::sleep(Duration::from_millis(8));
    }

    Ok(())
}
