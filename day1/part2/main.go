package main

import (
	"fmt"
	"io"
	"log"
	"os"
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
	var sum []int
	for _, part := range first {
		var count int
		for _, p := range second {
			if part == p {
				count++
			}
		}

		sum = append(sum, count*part)
	}

	fmt.Println(sum)
	var total int
	for _, s := range sum {
		total += s
	}

	fmt.Println(total)
}
