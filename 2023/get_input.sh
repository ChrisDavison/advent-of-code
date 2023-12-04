#!/usr/bin/env bash
SESSION=53616c7465645f5f72b48167ee6787c93709caa950b829051c0dce2d17a41654f74667d4a5b65ce0e32980dc766772ab75b6f78f30f6dfd9a2227325cc22a31e
year=$(date +%Y)
day1=$(date +"%d" | sed -e 's/^0//')
day2=$(date +"%d")
URL="https://adventofcode.com/"$year"/day/"$day1"/input"
curl --cookie "session=$SESSION" $URL > input/day$day2
