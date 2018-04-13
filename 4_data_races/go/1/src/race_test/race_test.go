package main_test

import (
    "testing"
    "race"
)

func TestRace(t *testing.T) {
    race.Race()
}
