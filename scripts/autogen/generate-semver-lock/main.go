package main

import (
	"encoding/hex"
	"fmt"
	"os"
	"strings"

	"github.com/base/contracts/scripts/checks/common"
	"github.com/ethereum-optimism/optimism/op-chain-ops/solc"
	"github.com/ethereum/go-ethereum/crypto"
)

const semverLockFile = "snapshots/semver-lock.json"

type SemverLockOutput struct {
	InitCodeHash   string `json:"initCodeHash"`
	SourceCodeHash string `json:"sourceCodeHash"`
}

type SemverLockResult struct {
	ContractKey      string
	SemverLockOutput SemverLockOutput
}

func main() {
	results, err := common.ProcessFilesGlob(
		[]string{"forge-artifacts/**/*.json"},
		[]string{},
		processFile,
	)
	if err != nil {
		fmt.Printf("Failed to generate semver lock: %v\n", err)
		os.Exit(1)
	}

	output := make(map[string]SemverLockOutput)
	for _, result := range results {
		if result == nil {
			continue
		}
		output[result.ContractKey] = result.SemverLockOutput
	}

	if err := common.WriteJSON(output, semverLockFile); err != nil {
		panic(err)
	}

	fmt.Printf("Wrote semver lock file to \"%s\".\n", semverLockFile)
}

func processFile(file string) (*SemverLockResult, []error) {
	artifact, err := common.ReadForgeArtifact(file)
	if err != nil {
		return nil, []error{fmt.Errorf("failed to read artifact: %w", err)}
	}

	var sourceFilePath, contractName string
	for path, name := range artifact.Metadata.Settings.CompilationTarget {
		sourceFilePath, contractName = path, name
		break
	}
	contractKey := sourceFilePath + ":" + contractName
	if strings.HasSuffix(file, ".dispute.json") {
		// The "dispute" compiler profile can produce different bytecode for
		// the same source file, so forge emits both <contract>.sol.json and
		// <contract>.dispute.json with identical CompilationTargets. Suffix
		// the key to keep initCode hashes deterministic across the pair.
		contractKey += ":dispute"
	}

	// Only apply to files in the src directory.
	if !strings.HasPrefix(sourceFilePath, "src/") {
		return nil, nil
	}

	if !hasSemverVersion(artifact, contractName) {
		return nil, nil
	}

	initCodeBytes, err := hex.DecodeString(strings.TrimPrefix(artifact.Bytecode.Object, "0x"))
	if err != nil {
		return nil, []error{fmt.Errorf("failed to decode hex: %w", err)}
	}

	sourceCode, err := os.ReadFile(sourceFilePath)
	if err != nil {
		return nil, []error{fmt.Errorf("failed to read source file: %w", err)}
	}

	trimmedSourceCode := []byte(strings.TrimSuffix(string(sourceCode), "\n"))
	initCodeHash := fmt.Sprintf("0x%x", crypto.Keccak256Hash(initCodeBytes))
	sourceCodeHash := fmt.Sprintf("0x%x", crypto.Keccak256Hash(trimmedSourceCode))

	return &SemverLockResult{
		ContractKey: contractKey,
		SemverLockOutput: SemverLockOutput{
			InitCodeHash:   initCodeHash,
			SourceCodeHash: sourceCodeHash,
		},
	}, nil
}

func hasSemverVersion(artifact *solc.ForgeArtifact, contractName string) bool {
	for _, node := range artifact.Ast.Nodes {
		if node.NodeType != "ContractDefinition" || node.Name != contractName {
			continue
		}
		for _, subNode := range node.Nodes {
			if (subNode.NodeType != "FunctionDefinition" &&
				subNode.NodeType != "VariableDeclaration") ||
				subNode.Name != "version" {
				continue
			}
			if strings.Contains(docText(subNode.Documentation), "@custom:semver") {
				return true
			}
		}
		return false
	}
	return false
}

func docText(doc interface{}) string {
	switch d := doc.(type) {
	case string:
		return d
	case map[string]interface{}:
		if text, ok := d["text"].(string); ok {
			return text
		}
	}
	return ""
}
