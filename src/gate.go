package gate

// #cgo LDFLAGS: -L./../lib/ -lrust_from_go
// #include "bridge.h"
import "C"

/*
RunSvParser is a wrapper around the Rust function run_sv_parser

It takes a string in the formof a json object representing a
sequency of paths as an argument and returns a string in the
form of a JSON object

Example input:

	{
		"files": {
			"include": [
				"file1",
				"path/to/file2"
			],
			"source": [
				"file3",
				"path/to/dir1"
			]
		}
	}
*/
func RunSvParser(str string) string {
	cstr := C.CString(str)
	defer C.free_string(cstr)

	result := C.run_sv_parser(cstr)
	defer C.free_string(result)

	return C.GoString(result)
}
