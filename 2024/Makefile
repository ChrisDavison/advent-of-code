src := $(wildcard day*.py)
today := $(shell date +%02d)

all: $(src)
	@for thing in $^; do echo $$thing; python3 $$thing; echo "----------"; done

day%: day%.py
	@python3 $^

today: day$(today).py
	@python3 $^

