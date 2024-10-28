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
    let frame_duration = Duration::from_millis(2000/3);
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut prev_cpu_output = String::new();
    let mut prev_memory_output = String::new();

    execute!(stdout(), Clear(ClearType::All)).unwrap();

    loop {
        let frame_start = Instant::now();
        sys.refresh_all();

        // Prepare CPU output in a string buffer to avoid excessive screen clearing
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

        // Prepare memory output in a string buffer to avoid excessive screen clearing
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

        execute!(stdout(), Hide).unwrap(); // Hide the cursor for better visuals

        // Only update CPU section if there's a change
        if cpu_output != prev_cpu_output {
            execute!(stdout(), MoveTo(0, 0)).unwrap();
            print!("{}", cpu_output);
            prev_cpu_output = cpu_output.clone();
        }

        // Only update memory section if there's a change
        if memory_output != prev_memory_output {
            execute!(stdout(), MoveTo(0, (sys.cpus().len() + 2) as u16)).unwrap();
            print!("{}", memory_output);
            prev_memory_output = memory_output.clone();
        }

        stdout().flush().unwrap();
        execute!(stdout(), Show).unwrap(); // Show cursor back

        let frame_time = frame_start.elapsed();
        if frame_duration > frame_time {
            thread::sleep(frame_duration - frame_time);
        }
    }
}
