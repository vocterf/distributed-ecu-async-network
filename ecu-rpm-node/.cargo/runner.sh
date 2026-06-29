ELF_PATH=$1
BIN_PATH="target/app.bin"

esptool --chip esp32 elf2image -o "$BIN_PATH" "$ELF_PATH" > /dev/null

esptool --port /dev/ttyUSB0 --baud 230400 --no-stub write_flash 0x10000 "$BIN_PATH"