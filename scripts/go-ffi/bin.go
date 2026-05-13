package main

import (
	"log"
	"os"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatal("Must pass a subcommand: diff, trie, or merkle")
	}
	switch os.Args[1] {
	case "diff":
		DiffTestUtils()
	case "trie":
		FuzzTrie()
	case "merkle":
		DiffMerkle()
	default:
		log.Fatalf("Unknown subcommand %q: must be diff, trie, or merkle", os.Args[1])
	}
}
