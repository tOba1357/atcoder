package main

import "fmt"

func main() {
	N := 0
	fmt.Scan(&N)
	dps := make([][][][]int64, N+1)
	for i := 3; i <= N; i++ {
		dps[i] = make([][][]int64, 4)
		for j := 0; j <= 3; j++ {
			dps[i][j] = make([][]int64, 4)
			for k := 0; k <= 3; k++ {
				dps[i][j][k] = make([]int64, 4)
				for l := 0; l <= 3; l++ {
					dps[i][j][k][l] = 0
				}
			}
		}
	}
	for i := 3; i <= N; i++ {
		for j := 0; j <= 3; j++ {
			for k := 0; k <= 3; k++ {
				for l := 0; l <= 3; l++ {
					if i == 3 {
						if j == 0 && k == 1 && l == 2 {
							dps[i][j][k][l] = 0
						} else if j == 1 && k == 0 && l == 2 {
							dps[i][j][k][l] = 0
						} else if j == 0 && k == 2 && l == 1 {
							dps[i][j][k][l] = 0
						} else {
							dps[i][j][k][l] = 1
						}
						continue
					}
					for m := 0; m <= 3; m++ {
						enable := true
						if m == 2 && k == 0 && l == 1 {
							enable = false
						} else if m == 2 && j == 0 && k == 1 {
							enable = false
						} else if m == 2 && j == 0 && l == 1 {
							enable = false
						} else if m == 2 && k == 1 && l == 0 {
							enable = false
						} else if m == 1 && k == 0 && l == 2 {
							enable = false
						}
						if enable {
							dps[i][k][l][m] += dps[i-1][j][k][l] % 1000000007
						}
					}
				}
			}
		}
	}

	ans := int64(0)
	for j := 0; j <= 3; j++ {
		for k := 0; k <= 3; k++ {
			for l := 0; l <= 3; l++ {
				ans += dps[N][j][k][l]
			}
		}
	}
	fmt.Println(ans % 1000000007)
}
