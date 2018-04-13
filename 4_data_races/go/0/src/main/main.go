package main

import (
    "fmt"
    "race"
)

func main() {
    for {
        var i = race.Race()
        if i != race.N {
            fmt.Printf("THAT'S WEIRD!!! (%d)\n", i)
        } else {
            fmt.Println("Everything is awesome!")
        }
    }
}
