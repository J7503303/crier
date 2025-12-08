# 📣 Crier

Simple push notification tool. Works on LAN or across the internet (no public IP needed!).

## Install

```bash
cargo build --release
# Binary: ./target/release/crier
```

## Usage

### Direct Mode (TCP)
Listener binds a port, sender connects directly. Fast, but requires network access.

```bash
# Listener
crier listen 0.0.0.0:5555 -m 'notify-send "Alert" "{}"'

# Sender
crier send 192.168.1.10:5555 -m "Build complete!"
```

### Relay Mode (MQTT)
Both connect outbound to a public MQTT broker. Works across NAT/firewalls.

```bash
# Listener
crier listen --relay test.mosquitto.org -t mybuilds -m 'notify-send "Build" "{}"'

# Sender  
crier send --relay test.mosquitto.org -t mybuilds -m "Build complete!"
```

**Free public brokers:** `test.mosquitto.org`, `broker.hivemq.com`

## Authentication

### Direct Mode
```bash
crier listen 0.0.0.0:5555 -m 'echo "{}"' --auth secret123
crier send host:5555 -m "Hello" --auth secret123
```

### Relay Mode
```bash
# Listener - only accepts messages with matching auth
crier listen --relay test.mosquitto.org -t mytopic --auth secret123 -m 'echo "{}"'

# Sender - message delivered
crier send --relay test.mosquitto.org -t mytopic --auth secret123 -m "Hello"

# Sender with wrong auth - listener ignores it
crier send --relay test.mosquitto.org -t mytopic --auth wrongpass -m "Hello"
```

## Examples

### Build notifications
```bash
# Dev machine (run first)
crier listen --relay test.mosquitto.org -t ci/project --auth mytoken -m 'notify-send "CI" "{}"'

# Build server
make && crier send --relay test.mosquitto.org -t ci/project --auth mytoken -m "✓ Build passed"
```

### Custom port
```bash
crier listen --relay broker.example.com --port 8883 -t topic -m 'echo "{}"'
crier send --relay broker.example.com --port 8883 -t topic -m "Hello"
```

### Custom commands
```bash
# Play sound
crier listen 0.0.0.0:5555 -m 'paplay /usr/share/sounds/complete.oga'

# Log to file  
crier listen 0.0.0.0:5555 -m 'echo "[$(date)] {}" >> ~/crier.log'

# Run script
crier listen 0.0.0.0:5555 -m './on-message.sh "{}"'
```

## Options

```
SUBCOMMANDS:
  listen              Listen for messages
  send                Send a message

COMMON OPTIONS:
  -m, --message <M>   Command template (listen) or message (send)
  -a, --auth <A>      Authentication token

RELAY MODE:
  --relay <BROKER>    MQTT broker address
  --port <PORT>       Broker port (default: 1883)
  -t, --topic <T>     MQTT topic

DIRECT MODE:
  <ADDR>              Bind/target address (e.g., 0.0.0.0:5555)
```

## License

MIT
