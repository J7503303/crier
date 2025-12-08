# 📣 Crier

Simple push notification tool. Send messages from one machine, execute commands on another.

## Install

```bash
cargo build --release
# Binary: ./target/release/crier
```

## Usage

**1. Start listener** (on machine receiving notifications):
```bash
crier --listen 0.0.0.0:5555 -m 'notify-send "Alert" "{}"'
```

**2. Send messages** (from anywhere):
```bash
crier --send 192.168.1.10:5555 -m "Build complete!"
```

The `{}` placeholder is replaced with the received message.

## Examples

### Build notifications
```bash
# Dev machine (run first)
crier --listen 0.0.0.0:5555 -m 'notify-send "Build" "{}"'

# Build server
make && crier --send devmachine:5555 -m "✓ Build passed"
```

### With authentication
```bash
# Listener
crier --listen 0.0.0.0:5555 -m 'echo "{}"' --auth secret

# Sender
crier --send host:5555 -m "Hello" --auth secret
```

### Custom commands
```bash
# Play sound
crier --listen 0.0.0.0:5555 -m 'paplay /usr/share/sounds/freedesktop/stereo/complete.oga'

# Log to file
crier --listen 0.0.0.0:5555 -m 'echo "[$(date)] {}" >> /tmp/crier.log'

# Run any script
crier --listen 0.0.0.0:5555 -m './on-message.sh "{}"'
```

## Options

```
--listen <HOST:PORT>   Listen mode (e.g., 0.0.0.0:5555)
--send <HOST:PORT>     Send mode (e.g., 192.168.1.10:5555)
-m, --message          Listen: command with {} placeholder
                       Send: message to send
-a, --auth             Optional authentication token
```

## License

MIT
