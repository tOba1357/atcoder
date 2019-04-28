package main

import (
	"fmt"
	"math"
	"math/rand"
)

func add(a, b int) int {
	return a + b
}

func addSub(a, b int) (int, int) {
	return a + b, a - b
}

func main() {
	fmt.Println(addSub(12, 23))
	fmt.Println(add(23, 43))
	fmt.Println(rand.Intn(10))
	fmt.Println(math.Sqrt(7))

	sum := 0
	for i := 0; i < 10; i++ {
		sum += i
	}
	fmt.Println("sum; ", sum)
}