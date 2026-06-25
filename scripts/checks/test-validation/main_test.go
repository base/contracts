package main

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/ethereum-optimism/optimism/op-chain-ops/solc"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/stretchr/testify/require"
)

func artifactWithTarget(path, contract string) *solc.ForgeArtifact {
	return &solc.ForgeArtifact{
		Metadata: solc.ForgeCompilerMetadata{
			Settings: solc.CompilerSettings{
				CompilationTarget: map[string]string{path: contract},
			},
		},
	}
}

func artifactWithMethods(names ...string) *solc.ForgeArtifact {
	methods := make(map[string]abi.Method, len(names))
	for _, name := range names {
		methods[name] = abi.Method{Name: name}
	}
	return &solc.ForgeArtifact{
		Abi: solc.AbiType{Parsed: abi.ABI{Methods: methods}},
	}
}

func setExclusions(t *testing.T, paths, tests []string) {
	t.Helper()
	prevPaths, prevTests := excludedPaths, excludedTests
	excludedPaths, excludedTests = paths, tests
	t.Cleanup(func() { excludedPaths, excludedTests = prevPaths, prevTests })
}

func TestProcessFile(t *testing.T) {
	tmpFile := filepath.Join(t.TempDir(), "test.json")
	require.NoError(t, os.WriteFile(tmpFile, []byte(`{"abi":[{"name":"IS_TEST"}],"metadata":{"settings":{"compilationTarget":{"test.sol":"Test"}}}}`), 0644))
	_, errors := processFile(tmpFile)
	require.NotEmpty(t, errors, "expected error for invalid test name")
}

func TestValidateTestName(t *testing.T) {
	artifact := artifactWithMethods("IS_TEST", "test_valid_succeeds", "test_invalid_bad")
	require.Len(t, validateTestName(artifact), 1)
}

func TestExtractTestNames(t *testing.T) {
	tests := []struct {
		name     string
		artifact *solc.ForgeArtifact
		want     []string
	}{
		{
			name:     "valid test contract",
			artifact: artifactWithMethods("IS_TEST", "test_something_succeeds", "test_other_fails", "not_a_test", "testFuzz_something_works"),
			want:     []string{"test_something_succeeds", "test_other_fails", "testFuzz_something_works"},
		},
		{
			name:     "non-test contract",
			artifact: artifactWithMethods("test_something_succeeds", "not_a_test"),
			want:     nil,
		},
		{
			name:     "empty contract",
			artifact: artifactWithMethods(),
			want:     nil,
		},
		{
			name:     "test contract with no test methods",
			artifact: artifactWithMethods("IS_TEST", "not_a_test", "another_method"),
			want:     []string{},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.ElementsMatch(t, tt.want, extractTestNames(tt.artifact))
		})
	}
}

func TestCheckTestName(t *testing.T) {
	tests := []struct {
		testName      string
		shouldSucceed bool
	}{
		{"test_something_succeeds", true},
		{"testFuzz_something_reason_fails", true},
		{"testDiff_something_reason_reverts", true},
		{"test_something_benchmark_123", true},
		{"", false},
		{"Test_something_succeeds", false},
		{"test_Something_succeeds", false},
		{"test_something_succeed", false},
		{"test_something_fails", false},
		{"test__succeeds", false},
	}

	for _, tt := range tests {
		t.Run(tt.testName, func(t *testing.T) {
			err := checkTestName(tt.testName)
			require.Equal(t, tt.shouldSucceed, err == nil, "err = %v", err)
		})
	}
}

func TestValidateTestStructure(t *testing.T) {
	setExclusions(t, []string{"test/excluded/"}, nil)
	artifact := artifactWithTarget("test/excluded/Contract.t.sol", "Contract_Test")
	require.Empty(t, validateTestStructure(artifact))
}

func TestCheckTestStructure(t *testing.T) {
	require.Empty(t, checkTestStructure(artifactWithTarget("test.sol", "Contract_TestInit")))
	require.NotEmpty(t, checkTestStructure(artifactWithTarget("test.sol", "Invalid_Pattern")))
}

func TestGetCompilationTarget(t *testing.T) {
	tests := []struct {
		name         string
		artifact     *solc.ForgeArtifact
		wantPath     string
		wantContract string
		wantErr      bool
	}{
		{
			name:         "single target",
			artifact:     artifactWithTarget("path/file.sol", "Contract"),
			wantPath:     "path/file.sol",
			wantContract: "Contract",
		},
		{
			name:     "no targets",
			artifact: &solc.ForgeArtifact{},
			wantErr:  true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gotPath, gotContract, err := getCompilationTarget(tt.artifact)
			if tt.wantErr {
				require.Error(t, err)
				return
			}
			require.NoError(t, err)
			require.Equal(t, tt.wantPath, gotPath)
			require.Equal(t, tt.wantContract, gotContract)
		})
	}
}

func TestCheckSrcPath(t *testing.T) {
	tmpDir := t.TempDir()
	require.NoError(t, os.MkdirAll(filepath.Join(tmpDir, "src"), 0755))
	require.NoError(t, os.WriteFile(filepath.Join(tmpDir, "src", "Contract.sol"), []byte(""), 0644))
	t.Chdir(tmpDir)

	require.True(t, checkSrcPath(artifactWithTarget("test/Contract.t.sol", "Contract_Test")))
	require.False(t, checkSrcPath(artifactWithTarget("test/Missing.t.sol", "Missing_Test")))
}

func TestCheckContractNameFilePath(t *testing.T) {
	require.True(t, checkContractNameFilePath(artifactWithTarget("test/Contract.t.sol", "Contract_Test")))
	require.False(t, checkContractNameFilePath(artifactWithTarget("test/Contract.t.sol", "Other_Test")))
}

func TestFindArtifactPath(t *testing.T) {
	tmpDir := t.TempDir()
	require.NoError(t, os.MkdirAll(filepath.Join(tmpDir, "forge-artifacts", "Contract.sol"), 0755))
	require.NoError(t, os.WriteFile(filepath.Join(tmpDir, "forge-artifacts", "Contract.sol", "Contract.json"), []byte("{}"), 0644))
	t.Chdir(tmpDir)

	_, err := findArtifactPath("Contract.sol", "Contract")
	require.NoError(t, err)
	_, err = findArtifactPath("Missing.sol", "Missing")
	require.Error(t, err)
}

func TestIsLibrary(t *testing.T) {
	library := &solc.ForgeArtifact{Ast: solc.Ast{Nodes: []solc.AstNode{{NodeType: "ContractDefinition", ContractKind: "library"}}}}
	contract := &solc.ForgeArtifact{Ast: solc.Ast{Nodes: []solc.AstNode{{NodeType: "ContractDefinition", ContractKind: "contract"}}}}
	require.True(t, isLibrary(library))
	require.False(t, isLibrary(contract))
}

func TestExtractFunctionsFromAST(t *testing.T) {
	artifact := &solc.ForgeArtifact{
		Ast: solc.Ast{
			Nodes: []solc.AstNode{
				{
					NodeType: "ContractDefinition",
					Nodes: []solc.AstNode{
						{NodeType: "FunctionDefinition", Name: "add"},
						{NodeType: "FunctionDefinition", Name: "subtract"},
						{NodeType: "VariableDeclaration", Name: "ignored"},
					},
				},
			},
		},
	}
	require.Equal(t, []string{"add", "subtract"}, extractFunctionsFromAST(artifact))
}

func TestCheckFunctionExists(t *testing.T) {
	artifact := artifactWithTarget("test/Contract.t.sol", "Contract_Test")
	require.True(t, checkFunctionExists(artifact, "constructor"))
	require.False(t, checkFunctionExists(artifact, "nonexistent"))
}

func TestLoadExclusions(t *testing.T) {
	tmpFile := filepath.Join(t.TempDir(), "test.toml")
	require.NoError(t, os.WriteFile(tmpFile, []byte(`[excluded_paths]
src_validation = ["path1"]
[excluded_tests]
contracts = ["Test1"]`), 0644))

	setExclusions(t, nil, nil)
	require.NoError(t, loadExclusions(tmpFile))
	require.Len(t, excludedPaths, 1)
	require.Len(t, excludedTests, 1)
}

func TestIsExcluded(t *testing.T) {
	setExclusions(t, []string{"test/excluded/", "other/path/"}, nil)

	tests := []struct {
		filePath string
		want     bool
	}{
		{"test/excluded/file.sol", true},
		{"other/path/file.sol", true},
		{"test/normal/file.sol", false},
	}

	for _, tt := range tests {
		t.Run(tt.filePath, func(t *testing.T) {
			require.Equal(t, tt.want, isExcluded(tt.filePath))
		})
	}
}

func TestIsExcludedTest(t *testing.T) {
	setExclusions(t, nil, []string{"ExcludedContract", "AnotherExcluded"})

	tests := []struct {
		contractName string
		want         bool
	}{
		{"ExcludedContract", true},
		{"AnotherExcluded", true},
		{"NormalContract", false},
	}

	for _, tt := range tests {
		t.Run(tt.contractName, func(t *testing.T) {
			require.Equal(t, tt.want, isExcludedTest(tt.contractName))
		})
	}
}

func TestCamelCaseCheck(t *testing.T) {
	tests := []struct {
		name     string
		parts    []string
		expected bool
	}{
		{"valid single part", []string{"test"}, true},
		{"valid multiple parts", []string{"test", "something", "succeeds"}, true},
		{"invalid uppercase", []string{"Test"}, false},
		{"invalid middle uppercase", []string{"test", "Something", "succeeds"}, false},
		{"empty parts", []string{}, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.expected, checks["camelCase"].check(tt.parts))
		})
	}
}

func TestPartsCountCheck(t *testing.T) {
	tests := []struct {
		name     string
		parts    []string
		expected bool
	}{
		{"three parts", []string{"test", "something", "succeeds"}, true},
		{"four parts", []string{"test", "something", "reason", "fails"}, true},
		{"too few parts", []string{"test", "fails"}, false},
		{"too many parts", []string{"test", "a", "b", "c", "fails"}, false},
		{"empty parts", []string{}, false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.expected, checks["partsCount"].check(tt.parts))
		})
	}
}

func TestPrefixCheck(t *testing.T) {
	tests := []struct {
		name     string
		parts    []string
		expected bool
	}{
		{"valid test", []string{"test", "something", "succeeds"}, true},
		{"valid testFuzz", []string{"testFuzz", "something", "succeeds"}, true},
		{"valid testDiff", []string{"testDiff", "something", "succeeds"}, true},
		{"invalid prefix", []string{"testing", "something", "succeeds"}, false},
		{"empty parts", []string{}, false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.expected, checks["prefix"].check(tt.parts))
		})
	}
}

func TestSuffixCheck(t *testing.T) {
	tests := []struct {
		name     string
		parts    []string
		expected bool
	}{
		{"valid succeeds", []string{"test", "something", "succeeds"}, true},
		{"valid reverts", []string{"test", "something", "reverts"}, true},
		{"valid fails", []string{"test", "something", "fails"}, true},
		{"valid works", []string{"test", "something", "works"}, true},
		{"valid benchmark", []string{"test", "something", "benchmark"}, true},
		{"valid benchmark_num", []string{"test", "something", "benchmark", "123"}, true},
		{"invalid suffix", []string{"test", "something", "invalid"}, false},
		{"invalid benchmark_text", []string{"test", "something", "benchmark", "abc"}, false},
		{"empty parts", []string{}, false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.expected, checks["suffix"].check(tt.parts))
		})
	}
}

func TestFailurePartsCheck(t *testing.T) {
	tests := []struct {
		name     string
		parts    []string
		expected bool
	}{
		{"valid failure with reason fails", []string{"test", "something", "reason", "fails"}, true},
		{"valid failure with reason reverts", []string{"test", "something", "reason", "reverts"}, true},
		{"invalid failure without reason fails", []string{"test", "something", "fails"}, false},
		{"invalid failure without reason reverts", []string{"test", "something", "reverts"}, false},
		{"valid non-failure with three parts", []string{"test", "something", "succeeds"}, true},
		{"empty parts", []string{}, false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.expected, checks["failureParts"].check(tt.parts))
		})
	}
}

func TestDoubleUnderscoresCheck(t *testing.T) {
	tests := []struct {
		name     string
		parts    []string
		expected bool
	}{
		{"valid no empty", []string{"test", "something", "succeeds"}, true},
		{"invalid empty part", []string{"test", "", "succeeds"}, false},
		{"invalid multiple empty", []string{"test", "", "", "succeeds"}, false},
		{"empty parts", []string{}, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.expected, checks["doubleUnderscores"].check(tt.parts))
		})
	}
}
