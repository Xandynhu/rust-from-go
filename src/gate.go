package gate

// #cgo LDFLAGS: -L./../lib/ -lrust_from_go
// #include "bridge.h"
import "C"

// Add missing import

func Print(str string) {
	cstr := C.CString(str)
	defer C.free_string(cstr)

	C.print(cstr)
}

func ProcessJSON(str string) string {
	cstr := C.CString(str)
	defer C.free_string(cstr)

	result := C.process_json(cstr)

	return C.GoString(result)
}
