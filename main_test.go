package main

import (
	"fmt"
	"testing"
)

func Test(t *testing.T) {
	var yearI any
	yearI = 2021
	year := yearI.(string)
	fmt.Println("year", year)
}
