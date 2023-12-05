#!/usr/bin/env bash
year=$(date +%Y)
day1=$(date +"%d" | sed -e 's/^0//')
day2=$(date +"%d")

cat template.rs > src/day$day2.rs
sed -i'' -e "s/DAYNUM2/$day2/g" src/day$day2.rs
sed -i'' -e "s/DAYNUM1/$day1/g" src/day$day2.rs

cat template_toml.toml >> Cargo.toml
sed -i'' -e "s/DAYNUM2/$day2/g" Cargo.toml
sed -i'' -e "s/DAYNUM1/$day1/g" Cargo.toml

# now get data
rm input/day$day2 2> /dev/null
SESSION=53616c7465645f5f72b48167ee6787c93709caa950b829051c0dce2d17a41654f74667d4a5b65ce0e32980dc766772ab75b6f78f30f6dfd9a2227325cc22a31e
URL="https://adventofcode.com/"$year"/day/"$day1"/input"
curl --cookie "session=$SESSION" $URL > input/day$day2

git add src/day$day2.rs input/$day2 Cargo.toml
git commit -m "Start day$day2"

nvim src/day$day2.rs
