package main

import (
	"fmt"
)

func main() {
	var N, M, C int
	_, _ = fmt.Scan(&N, &M, &C)
	B := make([]int, M)
	for m := 0; m < M; m++ {
		_, _ = fmt.Scan(&B[m])
	}
	cnt := 0
	for n := 0; n < N; n++ {
		sum := 0
		for m := 0; m < M; m++ {
			var a int
			_, _ = fmt.Scan(&a)
			sum += a * B[m]
		}
		if sum + C > 0 {
			cnt++
		}
	}
	fmt.Println(cnt)
}
