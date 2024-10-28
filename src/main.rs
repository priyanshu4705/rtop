use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    terminal::{Clear, ClearType},
};
use std::{
    io::{stdout, Write},
    thread,
    time::{Duration, Instant},
};
use sysinfo::System;

fn main() {
    let frame_duration = Duration::from_millis(500);
    let mut sys = System::new_all();
    sys.refresh_all();

    execute!(stdout(), Clear(ClearType::All)).unwrap();

    loop {
        let frame_start = Instant::now();
        sys.refresh_all();

        let mut cpu_output = String::new();
        cpu_output.push_str("CPU Usage:\n");
        for (i, processor) in sys.cpus().iter().enumerate() {
            let usage = processor.cpu_usage();
            let bars = (usage / 2.0) as usize;
            cpu_output.push_str(&format!(
                "{:2}: [{}{}] {:.2}%\n",
                i,
                "█".repeat(bars),
                " ".repeat(50 - bars),
                usage
            ));
        }

        let mut memory_output = String::new();
        memory_output.push_str("Memory Usage:\n");
        memory_output.push_str(&format!(
            "Total RAM:               {} MB\n",
            sys.total_memory() / (1024 * 1024)
        ));
        memory_output.push_str(&format!(
            "Used RAM:                {} MB\n",
            sys.used_memory() / (1024 * 1024)
        ));
        memory_output.push_str(&format!(
            "Total swap:              {} MB\n",
            sys.total_swap() / (1024 * 1024)
        ));
        memory_output.push_str(&format!(
            "Used swap:               {} MB\n",
            sys.used_swap() / (1024 * 1024)
        ));

        execute!(stdout(), Hide).unwrap();

        execute!(stdout(), MoveTo(0, 0)).unwrap();
        print!("{}", cpu_output);

        execute!(stdout(), MoveTo(0, (sys.cpus().len() + 2) as u16)).unwrap();
        print!("{}", memory_output);

        stdout().flush().unwrap();
        execute!(stdout(), Show).unwrap();

        let frame_time = frame_start.elapsed();
        if frame_duration > frame_time {
            thread::sleep(frame_duration - frame_time);
        }
    }
}
