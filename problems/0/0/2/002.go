// Copyright (C) 2014 Jorge Aparicio

package main

import "fmt"

func main() {
    ans := 0
    curr := 1
    next := 2

    for curr < 4000000 {
        if curr % 2 == 0 {
            ans += curr
        }

        tmp := next
        next += curr
        curr = tmp
    }

    fmt.Println(ans)
}
