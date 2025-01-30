package main

import "core:bytes"
import "core:fmt"
import "core:os"
import "core:strconv"
import "core:strings"


Error :: enum {
	None,
	MultipleDecimal,
	InvalidNumber,
}

numbers_from_str :: proc(s: string) -> ([dynamic]f64, Error) {
	vals: [dynamic]f64
	buf: bytes.Buffer
	bytes.buffer_init(&buf, {})
	have_decimal := false
	have_number := false
	for ch in s {
		switch ch {
		case '0' ..< '9':
			fmt.println("..adding num")
			bytes.buffer_write_rune(&buf, ch)
			have_number = true
		case '.':
			fmt.println("..found decimal")
			if have_decimal {
				return nil, .MultipleDecimal
			}
			bytes.buffer_write_rune(&buf, ch)
		case:
			if bytes.buffer_length(&buf) > 0 {
				val, ok := strconv.parse_f64(bytes.buffer_to_string(&buf))
				if !ok {
					return nil, .InvalidNumber
				}
				append(&vals, val)
				bytes.buffer_truncate(&buf, 0)
			}
		}
	}

	val, ok := strconv.parse_f64(bytes.buffer_to_string(&buf))
	append(&vals, val)
	return vals, .None
}

main :: proc() {
	contents, _ := os.read_entire_file("example")
	contentss := strings.clone_from_bytes(contents)
	lefts: [dynamic]f64
	rights: [dynamic]f64
	for line in strings.split(contentss, "\n") {
		// fmt.println(strings.split(line, " "))
		vals, ok := numbers_from_str(line)
		left := vals[0]
		right := vals[1]
		fmt.println(left, right)
		append(&lefts, left)
		append(&rights, right)
	}
}
