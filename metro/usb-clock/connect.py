#!/usr/bin/env python3

import time
import serial
import threading
import argparse
from itertools import count


def reader(ser):
    """Continuously read from the serial port and print to screen."""
    while True:
        try:
            line = ser.readline()
            if line:
                print(line.decode(errors='ignore'), end='')
        except serial.SerialException:
            print("Serial port error.")
            break
        except KeyboardInterrupt:
            break


def writer(ser, interval=1.0):
    """Periodically send reset the clock."""
    ser.write(b"START\r\n")
    for i in count():
        try:
            time.sleep(interval)
            hour = i % 24
            min_sec = i % 60
            ser.write(
                f"time={hour:02}:{min_sec:02}:{min_sec:02}\r\n".encode("utf-8"))
        except serial.SerialException:
            print("Serial port error.")
            break
        except KeyboardInterrupt:
            break


def main():
    def_device = "/dev/ttyACM0"
    def_baud = 115200
    def_interval = 10.0

    parser = argparse.ArgumentParser(
        description="Connect to the Metro M0 and set the RTC every so often."
    )
    parser.add_argument(
        "-d", "--device",
        type=str,
        default=def_device,
        help=f"serial device path (default: {def_device})"
    )
    parser.add_argument(
        "-b", "--baud",
        type=int,
        default=def_baud,
        help=f"Baud rate (default: {def_baud})"
    )
    parser.add_argument(
        "-i", "--interval",
        type=float,
        default=def_interval,
        help=f"Interval in seconds between clock is set to incrementing hours (default: {def_interval})"
    )

    args = parser.parse_args()

    try:
        ser = serial.Serial(args.device, args.baud, timeout=1)
    except serial.SerialException as e:
        print(f"Could not open {args.device}: {e}")
        return

    # Start reader thread
    t = threading.Thread(target=reader, args=(ser,), daemon=True)
    t.start()

    # Run writer in main thread
    writer(ser, interval=args.interval)


if __name__ == "__main__":
    main()
