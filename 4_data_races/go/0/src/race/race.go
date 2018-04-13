package race

import (
    "sync"
)

const N = 100

func Race() int {
    var i = 0
    var wg sync.WaitGroup
    // To simulate concurrent updates, we'll start 100
    // goroutines that each increment the counter simultaneously
    for t := 0; t < N; t++ {
        wg.Add(1)
        go func() {
            i += 1
            defer wg.Done()
        }()
    }
    wg.Wait()
    return i
}
