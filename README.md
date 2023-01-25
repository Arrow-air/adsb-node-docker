# adsb-node-docker

:construction: This is a temporary repository.

## Raspberry Pi Instructions

:warning: The Raspberry Pi must have a network connection.

Build:
```bash
git clone https://github.com/arrow-air/adsb-node-docker.git
cd adsb-node-docker/
make docker-build
```

Enable RTL-SDR device access:
1) Create [rtl-sdr.rules](https://github.com/osmocom/rtl-sdr/blob/master/rtl-sdr.rules) at `/etc/udev/rules.d/rtl-sdr.rules`
2) Reload udev rules
```bash
rmmod rtl2832_sdr || \
rmmod dvb_usb_rtl28xxu || \
rmmod rtl2832 || \
rmmod rtl8xxxu || \
udevadm control --reload-rules
```

Run:
```bash
docker compose up
```
