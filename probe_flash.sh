BIN=$1

echo "[probe-rs] flashing: $BIN"
probe-rs download --chip RP2040 $BIN
probe-rs reset --chip RP2040
