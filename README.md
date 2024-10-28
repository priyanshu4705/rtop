# rtop

This is a simple system monitoring tool built in Rust, similar to `htop`. It displays real-time CPU and memory usage, refreshing every 500 milliseconds. The tool uses the `sysinfo` crate to gather system statistics and `crossterm` for terminal manipulation.

## Features

- **CPU Monitoring**: Displays a bar graph of CPU usage for each core.
- **Memory Monitoring**: Shows total and used RAM, as well as total and used swap memory.
- **Real-Time Updates**: Refreshes the information every 500 milliseconds.
- **Cross-Platform**: Uses `crossterm` for terminal manipulation, which works on Linux, macOS, and Windows.

## Prerequisites

- **Rust**: Make sure you have Rust installed. You can install Rust using the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- Dependencies: The project requires the following Rust crates:
  - sysinfo for fetching system information.
  - crossterm for terminal manipulation.

## Installation
Clone the repository or download the source code.
```bash
git clone https://github.com/priyanshu4705/rtop.git
cd system-monitoring-tool
```
Install the necessary dependencies using cargo:
```bash
cargo build --release
```
Usage
To run the tool, simply execute:
```bash
cargo run --release
```
Example Output
```yaml
CPU Usage:
 0: [██████████████████████████                          ] 52.00%
 1: [████████████████                                    ] 32.00%
 2: [██████████████████████████                          ] 54.00%
 3: [██████████████                                      ] 24.00%

Memory Usage:
Total RAM:               16384 MB
Used RAM:                7800 MB
Total swap:              2048 MB
Used swap:               512 MB
```
## Customization
Frame Duration: The frame refresh rate is controlled by the frame_duration variable in the code. To modify how frequently the terminal updates, adjust the value of frame_duration in milliseconds:
```rust
let frame_duration = Duration::from_millis(500);
```
Bar Length: The CPU usage bars are currently set to a length of 50 characters. You can modify the length by changing the repeat counts in the following code block:
```rust
"█".repeat(bars),
" ".repeat(50 - bars),
```
## Contributions
Feel free to fork this repository, make improvements, and submit pull requests. Any suggestions or issues can be raised in the Issues section.
