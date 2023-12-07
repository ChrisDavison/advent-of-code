#!/usr/bin/env bash
year=$(date +%Y)
day1=$(date +"%d" | sed -e 's/^0//')
day2=$(date +"%d")

cat template.py > day$day2.py
sed -i'' -e "s/DAYNUM2/$day2/g" day$day2.py
sed -i'' -e "s/DAYNUM1/$day1/g" day$day2.py

# now get data
rm input/day$day2 2> /dev/null
SESSION=$(cat ~/.aoc_token)
if [[ -z "$SESSION" ]]; then
    echo "AOC token empty. Put into ~/.aoc_token"
    exit -1
fi
URL="https://adventofcode.com/"$year"/day/"$day1"/input"
curl --cookie "session=$SESSION" $URL > input/$day2

git add day$day2.py input/$day2
git commit -m "Start day$day2"

nvim day$day2.py
