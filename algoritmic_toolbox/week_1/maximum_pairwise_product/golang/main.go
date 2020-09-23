package main

import (
	"fmt"
)

func main() {
	var n int

	fmt.Scan(&n)

	numbers := make([]int, n)

	for i := range numbers {
		fmt.Scan(&numbers[i])
	}

	fmt.Println(maxPairwiseProduct(numbers))
}

func maxPairwiseProduct(numbers []int) int64 {
	maxIndex1 := -1

	for i := range numbers {
		if maxIndex1 == -1 || numbers[i] > numbers[maxIndex1] {
			maxIndex1 = i
		}
	}

	maxIndex2 := -1

	for j := range numbers {
		if j == maxIndex1 {
			continue
		}

		if maxIndex2 == -1 || numbers[j] > numbers[maxIndex2] {
			maxIndex2 = j
		}
	}

	return int64(numbers[maxIndex1]) * int64(numbers[maxIndex2])
}
