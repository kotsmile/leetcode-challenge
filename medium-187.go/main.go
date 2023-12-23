package main

import (
	"fmt"
)

// AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT
//
// O(N**2) - take all possible 10-long-substrings and try to find it in string
// O(N)

func findRepeatedDnaSequences(s string) []string {
	const S = 10

	findings := make(map[string]int)
	answer := make([]string, 0, 10)

	for i := 0; i <= len(s)-S; i++ {
		substring := s[i : i+S]
		if findings[substring] == 1 {
			answer = append(answer, substring)
		}
		findings[substring] += 1

	}
	return answer
}

func main() {
	fmt.Println(findRepeatedDnaSequences("AAAAAAAAAAA"))
}
