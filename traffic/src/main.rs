use std::io::{self, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Instant};

fn main() -> io::Result<()> {
    loop {
        println!("Welcome to the Network Speed Test!");

        // Prompt the user to enter the server address
        let server_address = prompt_server_address()?;

        // Prompt the user to enter the download size
        let download_size_bytes = prompt_download_size()?;

        println!("Testing network speed...");

        // Resolve the server address
        let server_socket = format!("{}:80", server_address);
        let mut server_addrs = match server_socket.to_socket_addrs() {
            Ok(addrs) => addrs,
            Err(e) => {
                println!("Failed to resolve server address: {}", e);
                continue;
            }
        };

        // Connect to the server
        let start = Instant::now();
        let server_addr = match server_addrs.next() {
            Some(addr) => addr,
            None => {
                println!("Invalid server address");
                continue;
            }
        };
        let mut stream = match TcpStream::connect(server_addr) {
            Ok(stream) => stream,
            Err(e) => {
                println!("Failed to connect to the server: {}", e);
                continue;
            }
        };

        // Send dummy data to the server
        let data = vec![0; download_size_bytes];
        if let Err(e) = stream.write_all(&data) {
            println!("Failed to send data to the server: {}", e);
            continue;
        }

        // Get the total elapsed time
        let elapsed = start.elapsed();

        // Calculate the download speed in megabits per second (Mbps)
        let elapsed_secs = elapsed.as_secs_f64();
        let download_speed = (download_size_bytes as f64 * 8.0) / (elapsed_secs * 1_000_000.0);

        // Print the download speed
        println!("Download Speed: {:.2} Mbps", download_speed);

        // Prompt the user to restart the speed test
        if !prompt_restart()? {
            break;
        }
    }

    Ok(())
}

fn prompt_server_address() -> io::Result<String> {
    loop {
        print!("Enter the server address: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let server_address = input.trim();
        if !server_address.is_empty() {
            return Ok(server_address.to_string());
        }
        println!("Invalid input. Please try again.");
    }
}

fn prompt_download_size() -> io::Result<usize> {
    loop {
        print!("Enter the download size in bytes: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().parse::<usize>() {
            Ok(download_size) => {
                if download_size > 0 {
                    return Ok(download_size);
                }
                println!("Download size must be a positive number.");
            }
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn prompt_restart() -> io::Result<bool> {
    loop {
        print!("Do you want to restart the speed test? (y/n): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "y" => return Ok(true),
            "n" => return Ok(false),
            _ => println!("Invalid input. Please enter 'y' or 'n'."),
        }
    }
}
