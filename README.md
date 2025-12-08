# 📣 Crier

Simple push notification tool. Works on LAN (TCP) or across the internet (MQTT).

## Install

```bash
cargo build --release
# Binary: ./target/release/crier
```

## Usage

### Direct Mode (TCP)
```bash
crier listen 0.0.0.0:5555 -m 'notify-send "Alert" "{}"'
crier send 192.168.1.10:5555 -m "Build complete!"
```

### Relay Mode (MQTT)
Works across NAT/firewalls - no public IP needed!

```bash
crier listen --relay test.mosquitto.org -t mybuilds -m 'notify-send "Build" "{}"'
crier send --relay test.mosquitto.org -t mybuilds -m "Build complete!"
```

### Using Presets
Define presets in `~/.config/crier.yml`:

```yaml
mybuilds:
  relay: test.mosquitto.org
  topic: ci/myproject
  auth: secrettoken
  message: 'notify-send "Build" "{}"'

local:
  addr: "0.0.0.0:5555"
  auth: localpass
  message: 'notify-send "Local" "{}"'
```

Then use with `-p`:
```bash
# Listen using preset
crier listen -p mybuilds

# Send using preset (override message)
crier send -p mybuilds -m "Build passed!"
```

## Authentication

```bash
# TCP mode
crier listen 0.0.0.0:5555 -m 'echo "{}"' --auth secret123
crier send host:5555 -m "Hello" --auth secret123

# MQTT mode
crier listen --relay test.mosquitto.org -t topic --auth secret -m 'echo "{}"'
crier send --relay test.mosquitto.org -t topic --auth secret -m "Hello"
```

## Examples

### Build notifications
```bash
# Dev machine
crier listen -p mybuilds

# Build server
make && crier send -p mybuilds -m "✓ Build passed"
```

### Custom commands
```bash
crier listen 0.0.0.0:5555 -m 'paplay /usr/share/sounds/complete.oga'
crier listen 0.0.0.0:5555 -m 'echo "[$(date)] {}" >> ~/crier.log'
crier listen 0.0.0.0:5555 -m './on-message.sh "{}"'
```

## Config File

Location: `~/.config/crier.yml`

```yaml
preset_name:
  addr: "0.0.0.0:5555"      # TCP address (optional)
  relay: test.mosquitto.org  # MQTT broker (optional)
  port: 1883                 # MQTT port (default: 1883)
  topic: my/topic            # MQTT topic
  auth: secrettoken          # Auth token
  message: 'echo "{}"'       # Command template
```

## Options

```
SUBCOMMANDS:
  listen              Listen for messages
  send                Send a message

OPTIONS:
  -p, --preset <NAME>  Use preset from config
  -m, --message <M>    Command template or message
  -a, --auth <A>       Authentication token

MQTT MODE:
  --relay <BROKER>     MQTT broker address
  --port <PORT>        Broker port (default: 1883)
  -t, --topic <T>      MQTT topic

TCP MODE:
  <ADDR>               Bind/target address
```

## License

MIT
