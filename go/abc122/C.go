package main

import (
	"fmt"
)

func main() {
	var N, Q int
	var S string
	fmt.Scan(&N, &Q)
	fmt.Scan(&S)

	current := 0
	cnt := make([]int, N)
	for i := 1; i < N; i++ {
		if S[i-1:i+1] == "AC" {
			current++
		}
		cnt[i] = current
	}

	var l, r int
	for i := 0; i < Q; i++ {
		fmt.Scan(&l, &r)
		fmt.Println(cnt[r - 1] - cnt[l - 1])
	}
}
