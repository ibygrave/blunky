#!/bin/bash
set -ex

exe=$1

ravedude uno -cb 460800 $exe && exit

echo "Falling back to QEMU"

QEMU_CMD="qemu-system-avr -machine uno -bios $exe -nographic"

# gdb
QEMU_CMD="$QEMU_CMD -s -S"
fg_cmd () {
    cgdb -d avr-gdb -- -ex 'target remote :1234' $exe
}

# serial
#QEMU_CMD="$QEMU_CMD -serial tcp::5678,server=on -nographic"
#fg_cmd () {
#    sleep 1
#    telnet localhost 5678
#}

finish () {
    kill -9 $QEMU_PID
}

$QEMU_CMD &
QEMU_PID=$!
trap finish EXIT
fg_cmd

