package main

import (
	"fmt"
	"log"
	"os"
	"slices"
	"strconv"
	"strings"
)

func unoverlap_idranges(idranges [][]int) [][]int {
	var idranges_out [][]int
	slices.SortFunc(idranges, func(a, b []int) int {
		return a[0] - b[0]
	})

	nexti := 0
	for i, idrange := range idranges {
		if i < nexti {
			continue
		}

		nexti += 1
		for nexti < len(idranges) && idranges[nexti][0] <= idrange[1] {
			idrange[1] = max(idrange[1], idranges[nexti][1])
			nexti += 1
		}
		idranges_out = append(idranges_out, idrange)
	}

	return idranges_out
}

func idrangeCmp(idrange []int, target int) int {
	if idrange[0] <= target && idrange[1] >= target {
		return 0
	} else if idrange[1] < target {
		return -1
	} else {
		return 1
	}
}

func main() {
	content, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	var idranges [][]int
	totalfresh := 0

	lines := strings.Split(strings.TrimRight(string(content), "\n"), "\n")
	for _, line := range lines {
		if strings.Contains(line, "-") {
			rangevals := strings.Split(line, "-")
			low, err := strconv.Atoi(rangevals[0])
			if err != nil {
				log.Fatal(err)
			}
			high, err := strconv.Atoi(rangevals[1])
			if err != nil {
				log.Fatal(err)
			}
			idranges = append(idranges, []int{low, high})
		} else if line == "" {
			idranges = unoverlap_idranges(idranges)
			rangesizes := 0
			for _, idrange := range idranges {
				rangesize := (idrange[1] - idrange[0]) + 1
				fmt.Printf("%d-%d, size %d\n", idrange[0], idrange[1], rangesize)
				rangesizes += rangesize
			}
			fmt.Printf("Total sizes: %d\n", rangesizes)
			continue
		} else {
			id, err := strconv.Atoi(line)
			if err != nil {
				log.Fatal(err)
			}

			i, found := slices.BinarySearchFunc(idranges, id, idrangeCmp)
			if found {
				fmt.Printf("id %d is fresh because of range %d-%d\n", id, idranges[i][0], idranges[i][1])
				totalfresh += 1
			} else {
				fmt.Printf("id %d is spoiled\n", id)
			}
		}
	}
	fmt.Printf("Total fresh: %d\n", totalfresh)
}
