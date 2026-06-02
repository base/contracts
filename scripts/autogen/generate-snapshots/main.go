package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/base/contracts/scripts/checks/common"
	"github.com/ethereum-optimism/optimism/op-chain-ops/solc"
)

const (
	storageLayoutDir = "snapshots/storageLayout"
	abiDir           = "snapshots/abi"
)

func main() {
	if err := resetDirectory(storageLayoutDir); err != nil {
		fmt.Printf("Failed to reset storage layout directory: %v\n", err)
		os.Exit(1)
	}
	if err := resetDirectory(abiDir); err != nil {
		fmt.Printf("Failed to reset abi directory: %v\n", err)
		os.Exit(1)
	}

	if _, err := common.ProcessFilesGlob(
		[]string{"forge-artifacts/**/*.json"},
		[]string{},
		processFile,
	); err != nil {
		fmt.Printf("Failed to generate snapshots: %v\n", err)
		os.Exit(1)
	}
}

func processFile(file string) (common.Void, []error) {
	artifact, err := common.ReadForgeArtifact(file)
	if err != nil {
		return common.Void{}, []error{err}
	}

	contractName := parseArtifactName(file)

	// Skip anything that isn't in the src directory, with the exception of
	// GnosisSafe because it's used for decoding storage changes in superchain-ops.
	if !strings.HasPrefix(artifact.Ast.AbsolutePath, "src/") && contractName != "GnosisSafe" {
		return common.Void{}, nil
	}

	// Skip anything that isn't a proper contract.
	isContract := false
	for _, node := range artifact.Ast.Nodes {
		if node.NodeType == "ContractDefinition" &&
			node.Name == contractName &&
			node.ContractKind == "contract" &&
			!node.Abstract {
			isContract = true
			break
		}
	}
	if !isContract {
		return common.Void{}, nil
	}

	storageLayout := make([]solc.AbiSpecStorageLayoutEntry, 0, len(artifact.StorageLayout.Storage))
	for _, storageEntry := range artifact.StorageLayout.Storage {
		typ, ok := artifact.StorageLayout.Types[storageEntry.Type]
		if !ok {
			return common.Void{}, []error{fmt.Errorf("undefined type for %s:%s", contractName, storageEntry.Label)}
		}

		storageLayout = append(storageLayout, solc.AbiSpecStorageLayoutEntry{
			Label:  storageEntry.Label,
			Bytes:  typ.NumberOfBytes,
			Offset: storageEntry.Offset,
			Slot:   storageEntry.Slot,
			Type:   typ.Label,
		})
	}

	if err := common.WriteJSON(artifact.Abi.Raw, filepath.Join(abiDir, contractName+".json")); err != nil {
		return common.Void{}, []error{fmt.Errorf("failed to write abi: %w", err)}
	}
	if err := common.WriteJSON(storageLayout, filepath.Join(storageLayoutDir, contractName+".json")); err != nil {
		return common.Void{}, []error{fmt.Errorf("failed to write storage layout: %w", err)}
	}

	return common.Void{}, nil
}

// parseArtifactName extracts the contract name from a forge artifact filename.
// e.g. "ContractName.0.9.8.json" or "ContractName.json" -> "ContractName".
func parseArtifactName(artifactVersionFile string) string {
	name, _, _ := strings.Cut(filepath.Base(artifactVersionFile), ".")
	return name
}

func resetDirectory(dir string) error {
	if err := os.RemoveAll(dir); err != nil {
		return fmt.Errorf("failed to remove directory %q: %w", dir, err)
	}
	if err := os.MkdirAll(dir, os.ModePerm); err != nil {
		return fmt.Errorf("failed to create directory %q: %w", dir, err)
	}
	return nil
}
