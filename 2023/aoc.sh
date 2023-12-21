#!/bin/bash
set -eou pipefail
year=$(date +%Y)
day2=$(date +%d)
day1=$(echo $day2 | sed 's/^0*//')
aocdir="$HOME/code/advent-of-code/$year"


all() {
    for day in day*.py; do
        echo $day
        python3 $day
        echo "------------------"
    done
}

day(){ 
    python3 "day$(printf '%02d' $1).py"
}

today(){ 
    if [[ ! -e "day$day2.py" ]]; then
        setup_day
    fi
    python3 $(date +"day%02d.py")
}

watchtoday(){
    echo $(date +"day%02d.py")
    echo $(date +"day%02d.py") | entr python3 $(date +"day%02d.py")
}

setup_day() {
    filename="day$day2.py"
    filename_in="input/$day2"

    cd $aocdir
    echo $aocdir
    cat template.py > $filename
    sed -i'' "s/DAYNUM2/$day2/g" $filename
    [[ -f $filename_in ]] && rm $filename_in
    
    SESSION=$(cat $HOME/.aoc_token)
    if [[ -z $SESSION ]]; then
        echo "AOC token empty. Put it in ~/.aoc_token"
        exit -1
    fi
    URL="https://adventofcode.com/$year/day/$day1/input"
    echo $URL
    curl --cookie "session=$SESSION" $URL > $filename_in

    git add $filename $filename_in
    git commit -m "Start day$day2"
    firefox "https://adventofcode.com/$year/day/$day1"
    nvim "$day2.py"
}

mambaenv() {
    MAMBA_EXE="$HOME/.local/bin/micromamba"
    MAMBA_ROOT_PREFIX="$HOME/.mamba"
    source <($MAMBA_EXE shell hook --shell bash --root-prefix $MAMBA_ROOT_PREFIX)

    micromamba activate ml
}

pushd $aocdir > /dev/null
# switch statement for all, day, and today
case ${1:-} in
    all)
        mambaenv
        all
        ;;
    day)
        mambaenv
        day $2
        ;;
    today)
        mambaenv
        today
        ;;
    watch)
        mambaenv
        watchtoday
        ;;
    new)
        setup_day
        ;;
    [1-25]*)
        mambaenv
        day $(printf "%02d" $1)
        ;;
    *)
        echo "Usage: $0 {all|day|today|watch|new|1-25}"
        exit 1
esac
