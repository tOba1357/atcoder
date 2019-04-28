package main

import "fmt"

func main() {
	var S string
	fmt.Scan(&S)
	max, cnt := 0, 0
	for i := 0; i < len(S); i++ {
		s := S[i]
		if s == 'A' || s == 'C' || s == 'G' || s == 'T' {
			cnt++
		} else {
			if max < cnt {
				max = cnt
			}
			cnt = 0
		}
	}
	if max < cnt {
		max = cnt
	}
	fmt.Println(max)
}
