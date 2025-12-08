use clap::Parser;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;

/// Crier - Simple push notification tool
///
/// Start listener first, then send messages from anywhere.
#[derive(Parser, Debug)]
#[command(name = "crier", version, about)]
#[command(group(
    clap::ArgGroup::new("mode")
        .required(true)
        .args(["listen", "send"]),
))]
#[command(after_help = "EXAMPLES:
  Listen: crier --listen 0.0.0.0:5555 -m 'notify-send \"Alert\" \"{}\"'
  Send:   crier --send 192.168.1.10:5555 -m 'Build done!'")]
struct Args {
    /// Listen mode: bind address (e.g., 0.0.0.0:5555)
    #[arg(long, value_name = "ADDR")]
    listen: Option<String>,

    /// Send mode: target address (e.g., 192.168.1.10:5555)
    #[arg(long, value_name = "ADDR")]
    send: Option<String>,

    /// Listen: command template (use {} as placeholder)
    /// Send: message to send
    #[arg(long, short)]
    message: String,

    /// Optional authentication token
    #[arg(long, short)]
    auth: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(addr) = args.listen {
        listen(&addr, &args.message, args.auth.as_deref());
    } else if let Some(addr) = args.send {
        send(&addr, &args.message, args.auth.as_deref());
    } else {
        unreachable!("Clap ensures listen or send is present");
    }
}

/// Listen for incoming messages
fn listen(addr: &str, cmd_template: &str, auth: Option<&str>) {
    let listener = TcpListener::bind(addr).unwrap_or_else(|e| {
        eprintln!("Failed to bind {}: {}", addr, e);
        std::process::exit(1);
    });

    println!("Listening on {}", addr);
    println!("Command: {}", cmd_template);
    if auth.is_some() {
        println!("Auth: enabled");
    }
    println!();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let peer = stream.peer_addr().map(|a| a.to_string()).unwrap_or_default();
                
                let reader = BufReader::new(&stream);
                let mut lines = reader.lines();

                // Check auth if required
                if let Some(expected_auth) = auth {
                    match lines.next() {
                        Some(Ok(line)) if line == format!("AUTH:{}", expected_auth) => {}
                        _ => {
                            eprintln!("[{}] Auth failed", peer);
                            let _ = stream.write_all(b"ERR:AUTH\n");
                            continue;
                        }
                    }
                }

                // Read message
                if let Some(Ok(message)) = lines.next() {
                    println!("[{}] {}", peer, message);
                    
                    // Execute command with message
                    let cmd = cmd_template.replace("{}", &message);
                    run_command(&cmd);
                    
                    let _ = stream.write_all(b"OK\n");
                }
            }
            Err(e) => eprintln!("Connection error: {}", e),
        }
    }
}

/// Send a message
fn send(addr: &str, message: &str, auth: Option<&str>) {
    let mut stream = TcpStream::connect(addr).unwrap_or_else(|e| {
        eprintln!("Failed to connect to {}: {}", addr, e);
        std::process::exit(1);
    });

    // Send auth if provided
    if let Some(auth_token) = auth {
        writeln!(stream, "AUTH:{}", auth_token).unwrap();
    }

    // Send message
    writeln!(stream, "{}", message).unwrap();

    // Wait for response
    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    if reader.read_line(&mut response).is_ok() {
        if response.trim() == "OK" {
            println!("Sent: {}", message);
        } else {
            eprintln!("Error: {}", response.trim());
            std::process::exit(1);
        }
    }
}

fn run_command(cmd: &str) {
    println!("Running: {}", cmd);
    match Command::new("sh").arg("-c").arg(cmd).status() {
        Ok(s) if !s.success() => eprintln!("Command failed: {}", s),
        Err(e) => eprintln!("Failed to run: {}", e),
        _ => {}
    }
}
