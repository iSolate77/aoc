package main

import (
	"fmt"
	"io"
	"log"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	input := "day1/part1/input1_1"
	file, err := os.Open(input)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	content, err := io.ReadAll(file)
	if err != nil {
		log.Fatal(err)
	}

	lists := string(content)
	parts := strings.Fields(lists)

	var first, second []int
	for i, part := range parts {
		d, err := strconv.Atoi(part)
		if err != nil {
			log.Fatal(err)
		}

		if i%2 == 0 {
			first = append(first, d)
		}
		if i%2 == 1 {
			second = append(second, d)
		}
	}

	slices.Sort(first)
	slices.Sort(second)

	sum := 0.0
	for i := range len(first) {
		sum += math.Abs(float64(second[i] - first[i]))
	}

	fmt.Printf("No decimals: %.0f\n", sum)
}
