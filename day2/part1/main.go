package main

import (
	"fmt"
	"io"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	input := "day2/part1/input2_1"
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

	parts := strings.Split(lists, "\n")
	var count int
	for i, part := range parts {
		p := strings.Fields(part)
		digits := convertToDigits(p)
		if !(checkAscension(digits) || checkDescent(digits)) {
			continue
		}

		if !checkDifferance(digits) {
			continue
		}

		fmt.Printf("%d asc test: %t\n", i, checkAscension(digits))
		fmt.Printf("%d desc test: %t\n", i, checkDescent(digits))
		fmt.Printf("%d diff test: %t\n", i, checkDifferance(digits))

		count++
	}

	fmt.Println(count)
}

func checkAscension(row []int) bool {
	for i := 0; i < len(row)-1; i++ {
		if row[i] < row[i+1] {
			return false
		}
	}

	return true
}

func checkDescent(row []int) bool {
	for i := 0; i < len(row)-1; i++ {
		if row[i] > row[i+1] {
			return false
		}
	}

	return true
}

func checkDifferance(row []int) bool {
	for i := 0; i < len(row)-1; i++ {
		if math.Abs(float64(row[i]-row[i+1])) > 3.0 || math.Abs(float64(row[i]-row[i+1])) < 1.0 {
			return false
		}
	}

	return true
}

func convertToDigits(row []string) []int {
	var res []int
	for _, d := range row {
		digit, err := strconv.Atoi(d)
		if err != nil {
			log.Fatal(err)
		}

		res = append(res, digit)
	}
	return res
}
