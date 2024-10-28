use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};
use sysinfo::System;

fn main() {
    let interval = 1000; // milliseconds
    let mut sys = System::new_all();
    sys.refresh_all();
    execute!(stdout(), Clear(ClearType::All)).unwrap();

    loop {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
        sys.refresh_all();

        display_cpu_usage(&sys);
        display_memory_usage(&sys);

        thread::sleep(Duration::from_millis(interval));
        stdout().flush().unwrap();
    }
}

fn display_cpu_usage(sys: &System) {
    println!("CPU Usage:");
    for (i, processor) in sys.cpus().iter().enumerate() {
        let usage = processor.cpu_usage();
        let bars = (usage / 2.0) as usize; // Simple scaling for better visualization
        println!(
            "{:2}: [{}{}] {:.2}%",
            i,
            "█".repeat(bars),
            " ".repeat(50 - bars),
            usage
        );
    }
}

fn display_memory_usage(sys: &System) {
    println!("Memory Usage:");
    println!("Total RAM:  {:.2} MB", sys.total_memory() / (1024 * 1024));
    println!("Used RAM:   {:.2} MB", sys.used_memory() / (1024 * 1024));
    println!("Total swap: {:.2} MB", sys.total_swap() / (1024 * 1024));
    println!("Used swap:  {:.2} MB", sys.used_swap() / (1024 * 1024));
}
