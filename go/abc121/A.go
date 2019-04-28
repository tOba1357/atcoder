package main

import "fmt"

func main() {
	var H, W, h, w int
	_, _ = fmt.Scan(&H, &W, &h, &w)
	fmt.Println((H - h) * (W- w))
}
