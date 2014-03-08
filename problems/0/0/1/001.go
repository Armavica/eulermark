package main

import "fmt"

func main() {
    acc := 0

    for i := 0; i < 1000; i++ {
        if i % 3 == 0 || i % 5 == 0 {
            acc += i
        }
    }

    fmt.Println(acc)
}

