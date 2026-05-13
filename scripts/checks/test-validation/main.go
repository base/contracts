package main

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"slices"
	"strconv"
	"strings"
	"unicode"

	"github.com/BurntSushi/toml"
	"github.com/base/contracts/scripts/checks/common"
	"github.com/ethereum-optimism/optimism/op-chain-ops/solc"
)

// Validates test function naming conventions and structure in Forge test artifacts.
func main() {
	scriptDir := filepath.Dir(os.Args[0])
	exclusionsPath := filepath.Join(scriptDir, "exclusions.toml")
	if err := loadExclusions(exclusionsPath); errors.Is(err, fs.ErrNotExist) {
		// Fall back to repo-relative path when running with `go run`.
		err = loadExclusions("scripts/checks/test-validation/exclusions.toml")
		if err != nil {
			fmt.Printf("error loading exclusions: %v\n", err)
			os.Exit(1)
		}
	} else if err != nil {
		fmt.Printf("error loading exclusions: %v\n", err)
		os.Exit(1)
	}

	if _, err := common.ProcessFilesGlob(
		[]string{"forge-artifacts/**/*.t.sol/*.json"},
		[]string{},
		processFile,
	); err != nil {
		fmt.Printf("error: %v\n", err)
		os.Exit(1)
	}

	fmt.Println("✅ All contract test validations passed")
}

func processFile(path string) (*common.Void, []error) {
	artifact, err := common.ReadForgeArtifact(path)
	if err != nil {
		return nil, []error{err}
	}

	var errs []error
	errs = append(errs, validateTestName(artifact)...)
	errs = append(errs, validateTestStructure(artifact)...)
	return nil, errs
}

// Test name validation

func validateTestName(artifact *solc.ForgeArtifact) []error {
	var errs []error
	for _, name := range extractTestNames(artifact) {
		if err := checkTestName(name); err != nil {
			errs = append(errs, err)
		}
	}
	return errs
}

func extractTestNames(artifact *solc.ForgeArtifact) []string {
	isTest := false
	names := []string{}
	for _, entry := range artifact.Abi.Parsed.Methods {
		if entry.Name == "IS_TEST" {
			isTest = true
			continue
		}
		if strings.HasPrefix(entry.Name, "test") {
			names = append(names, entry.Name)
		}
	}
	if !isTest {
		return nil
	}
	return names
}

func checkTestName(name string) error {
	parts := strings.Split(name, "_")
	for _, check := range checks {
		if !check.check(parts) {
			return fmt.Errorf("%s: %s", name, check.error)
		}
	}
	return nil
}

// Test structure validation

func validateTestStructure(artifact *solc.ForgeArtifact) []error {
	filePath, _, err := getCompilationTarget(artifact)
	if err != nil {
		return []error{err}
	}

	if isExcluded(filePath) {
		return nil
	}

	var errs []error
	if !checkSrcPath(artifact) {
		errs = append(errs, fmt.Errorf("test file path does not match src path"))
	}
	if !checkContractNameFilePath(artifact) {
		errs = append(errs, fmt.Errorf("contract name does not match file path"))
	}
	errs = append(errs, checkTestStructure(artifact)...)
	return errs
}

// Allowed function-name placeholders for the <Contract>_<FunctionName>_Test pattern.
var allowedFunctionNames = []string{"Uncategorized", "Integration"}

func checkTestStructure(artifact *solc.ForgeArtifact) []error {
	var errs []error

	for _, contractName := range artifact.Metadata.Settings.CompilationTarget {
		if isExcludedTest(contractName) {
			continue
		}

		parts := strings.Split(contractName, "_")
		switch {
		case len(parts) == 2 && parts[1] == "TestInit":
			// <ContractName>_TestInit
		case len(parts) == 2 && parts[1] == "Harness":
			// <ContractName>_Harness
		case len(parts) == 3 && parts[2] == "Harness":
			// <ContractName>_<Descriptor>_Harness (e.g. OPContractsManager_Upgrade_Harness)
		case (len(parts) == 3 || len(parts) == 4) && parts[len(parts)-1] == "Test":
			errs = append(errs, checkTestMethodName(artifact, contractName, parts[1])...)
		default:
			errs = append(errs, fmt.Errorf("contract '%s': invalid naming pattern. Expected patterns: <ContractName>_TestInit, <ContractName>_<FunctionName>_Test, or <ContractName>_Uncategorized_Test", contractName))
		}
	}

	return errs
}

func checkTestMethodName(artifact *solc.ForgeArtifact, contractName string, functionName string) []error {
	if slices.Contains(allowedFunctionNames, functionName) {
		return nil
	}
	if !checkFunctionExists(artifact, functionName) {
		camelCaseFunctionName := strings.ToLower(functionName[:1]) + functionName[1:]
		return []error{fmt.Errorf("contract '%s': function '%s' does not exist in source contract", contractName, camelCaseFunctionName)}
	}
	return nil
}

// Artifact and path validation helpers

func getCompilationTarget(artifact *solc.ForgeArtifact) (string, string, error) {
	for filePath, contractName := range artifact.Metadata.Settings.CompilationTarget {
		return filePath, contractName, nil
	}
	return "", "", fmt.Errorf("no compilation target found")
}

func checkSrcPath(artifact *solc.ForgeArtifact) bool {
	for testFilePath := range artifact.Metadata.Settings.CompilationTarget {
		if !strings.HasPrefix(testFilePath, "test/") || !strings.HasSuffix(testFilePath, ".t.sol") {
			return false
		}

		srcPath := "src/" + strings.TrimPrefix(testFilePath, "test/")
		srcPath = strings.TrimSuffix(srcPath, ".t.sol") + ".sol"

		if _, err := os.Stat(srcPath); err != nil {
			return false
		}
	}
	return true
}

func checkContractNameFilePath(artifact *solc.ForgeArtifact) bool {
	for filePath, contractName := range artifact.Metadata.Settings.CompilationTarget {
		if isExcludedTest(contractName) {
			continue
		}

		fileName := strings.TrimSuffix(filepath.Base(filePath), ".t.sol")
		baseContractName := strings.SplitN(contractName, "_", 2)[0]
		if fileName != baseContractName {
			return false
		}
	}
	return true
}

func findArtifactPath(contractFileName, contractName string) (string, error) {
	pattern := "forge-artifacts/" + contractFileName + "/" + contractName + "*.json"
	files, err := filepath.Glob(pattern)
	if err != nil || len(files) == 0 {
		return "", fmt.Errorf("no artifact found for %s", contractName)
	}
	return files[0], nil
}

func isLibrary(artifact *solc.ForgeArtifact) bool {
	for _, node := range artifact.Ast.Nodes {
		if node.NodeType == "ContractDefinition" && node.ContractKind == "library" {
			return true
		}
	}
	return false
}

// Extracts function names from the AST — needed for libraries whose internal
// functions are absent from the ABI.
func extractFunctionsFromAST(artifact *solc.ForgeArtifact) []string {
	var functions []string
	for _, node := range artifact.Ast.Nodes {
		if node.NodeType != "ContractDefinition" {
			continue
		}
		for _, child := range node.Nodes {
			if child.NodeType == "FunctionDefinition" && child.Name != "" {
				functions = append(functions, child.Name)
			}
		}
	}
	return functions
}

func checkFunctionExists(artifact *solc.ForgeArtifact, functionName string) bool {
	if strings.EqualFold(functionName, "constructor") || strings.EqualFold(functionName, "receive") || strings.EqualFold(functionName, "fallback") {
		return true
	}

	for filePath := range artifact.Metadata.Settings.CompilationTarget {
		srcPath := strings.TrimPrefix(filePath, "test/")
		srcPath = strings.TrimSuffix(srcPath, ".t.sol") + ".sol"

		contractFileName := filepath.Base(srcPath)
		contractName := strings.TrimSuffix(contractFileName, ".sol")

		srcArtifactPath, err := findArtifactPath(contractFileName, contractName)
		if err != nil {
			return false
		}

		srcArtifact, err := common.ReadForgeArtifact(srcArtifactPath)
		if err != nil {
			fmt.Printf("Failed to read artifact: %s, error: %v\n", srcArtifactPath, err)
			return false
		}

		if isLibrary(srcArtifact) {
			for _, fn := range extractFunctionsFromAST(srcArtifact) {
				if strings.EqualFold(fn, functionName) {
					return true
				}
			}
			return false
		}

		for _, method := range srcArtifact.Abi.Parsed.Methods {
			if strings.EqualFold(method.Name, functionName) {
				return true
			}
		}
	}
	return false
}

// Exclusion configuration

var (
	excludedPaths []string
	excludedTests []string
)

type ExclusionsConfig struct {
	ExcludedPaths struct {
		SrcValidation          []string `toml:"src_validation"`
		ContractNameValidation []string `toml:"contract_name_validation"`
		FunctionNameValidation []string `toml:"function_name_validation"`
	} `toml:"excluded_paths"`
	ExcludedTests struct {
		Contracts []string `toml:"contracts"`
	} `toml:"excluded_tests"`
}

func loadExclusions(configPath string) error {
	var config ExclusionsConfig
	if _, err := toml.DecodeFile(configPath, &config); err != nil {
		return fmt.Errorf("failed to decode TOML file: %w", err)
	}

	excludedPaths = slices.Concat(
		config.ExcludedPaths.SrcValidation,
		config.ExcludedPaths.ContractNameValidation,
		config.ExcludedPaths.FunctionNameValidation,
	)
	excludedTests = config.ExcludedTests.Contracts
	return nil
}

func isExcluded(filePath string) bool {
	for _, excluded := range excludedPaths {
		if strings.HasPrefix(filePath, excluded) {
			return true
		}
	}
	return false
}

func isExcludedTest(contractName string) bool {
	return slices.Contains(excludedTests, contractName)
}

type CheckFunc func(parts []string) bool

type CheckInfo struct {
	error string
	check CheckFunc
}

var checks = map[string]CheckInfo{
	"doubleUnderscores": {
		error: "test names cannot have double underscores",
		check: func(parts []string) bool {
			for _, part := range parts {
				if len(strings.TrimSpace(part)) == 0 {
					return false
				}
			}
			return true
		},
	},
	"camelCase": {
		error: "test name parts should be in camelCase",
		check: func(parts []string) bool {
			for _, part := range parts {
				if len(part) > 0 && unicode.IsUpper(rune(part[0])) {
					return false
				}
			}
			return true
		},
	},
	"partsCount": {
		error: "test names should have either 3 or 4 parts, each separated by underscores",
		check: func(parts []string) bool {
			return len(parts) == 3 || len(parts) == 4
		},
	},
	"prefix": {
		error: "test names should begin with 'test', 'testFuzz', or 'testDiff'",
		check: func(parts []string) bool {
			return len(parts) > 0 && (parts[0] == "test" || parts[0] == "testFuzz" || parts[0] == "testDiff")
		},
	},
	"suffix": {
		error: "test names should end with either 'succeeds', 'reverts', 'fails', 'works', or 'benchmark[_num]'",
		check: func(parts []string) bool {
			if len(parts) == 0 {
				return false
			}
			last := parts[len(parts)-1]
			if last == "succeeds" || last == "reverts" || last == "fails" || last == "works" {
				return true
			}
			if len(parts) >= 2 && parts[len(parts)-2] == "benchmark" {
				_, err := strconv.Atoi(last)
				return err == nil
			}
			return last == "benchmark"
		},
	},
	// Failure tests must include a reason segment, so they need 4 parts not 3.
	"failureParts": {
		error: "failure tests should have 4 parts, third part should indicate the reason for failure",
		check: func(parts []string) bool {
			if len(parts) == 0 {
				return false
			}
			last := parts[len(parts)-1]
			return len(parts) == 4 || (last != "reverts" && last != "fails")
		},
	},
}
