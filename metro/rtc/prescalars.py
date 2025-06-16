#! /usr/bin/env python3
import argparse
import pendulum as pend

parser = argparse.ArgumentParser(description="Calculates the required minimum timeout needed to require a certain RTC prescalar value.")
args = parser.parse_args()

# Possible RTC prescalar values
prescalars = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]

# RTC clock source rate
#f0 = 1024
f0 = 32768

for ps in prescalars :
    ticks = (ps - 1) << 16
    timeout = pend.Duration(seconds = ticks / f0)
    fits = timeout.in_seconds()*1_000_000_000 <= 0xFFFFFFFF

    print(f"prescalar = {ps}, ticks = {ticks}, min seconds = {timeout.in_seconds()}, min duration = {timeout}, fits in u32 = {fits}")

