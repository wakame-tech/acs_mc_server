# acs_mc_server
## install plugins
```
cd spigot_plugin_manager
cargo build --release
cd ../minecraft/plugins
../../spigot_plugin_manager/target/release/spigot_plugin_manager install
```

## start

```bash
cd minecraft
docker compose up -d
```

## systemd
- `/etc/systemd/system/minecraft.service`

```ini
[Unit]
Description=spigot
After=docker.service
Requires=docker.service

[Service]
WorkingDirectory=/home/ubuntu/minecraft
ExecStart=/home/ubuntu/minecraft/start.sh
TimeoutStopSec=0
RestartSec=5
Restart=on-failure

[Install]
WantedBy=default.target
```