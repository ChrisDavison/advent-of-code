session="53616c7465645f5f905d33cbbfd2ef87817a27d654b4fe7e3f421b24fe0f415afe9da8bea59b47983c81ccebe96c7f16"

if [ $# -lt 1 ]; then
    echo "usage: get_input DAY"
    exit
fi
day=$1
url="https://adventofcode.com/2020/day/$day/input"
SRC="/home/davison/code/advent-of-code/src/aoc2020/day$day.rs"
echo -n "const INPUT: &str = \"" >> $SRC
curl $url  --cookie "session=$session" >> $SRC
echo "\";" >> $SRC

echo "Data inserted into bottom of $SRC"
