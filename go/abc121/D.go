package main

import (
	"fmt"
)

func calc(A int64) int64 {
	ans := int64(0)
	cnt := (A + 1) / 2
	ans += cnt % 2
	if A % 2 == 0 {
		ans += A
	}
	return ans
}

func main() {
	A, B := int64(0), int64(0)
	_, _ = fmt.Scan(&A, &B)
	fmt.Println(calc(B) ^ calc(A - 1))
}
