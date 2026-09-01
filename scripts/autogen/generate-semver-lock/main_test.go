package main

import (
	"os"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestNormalizeSourceCode(t *testing.T) {
	tests := []struct {
		name   string
		source string
		want   string
	}{
		{
			name:   "Unix line endings",
			source: "contract Test {\n    function version() public pure returns (string memory) {}\n}\n",
			want:   "contract Test {\n    function version() public pure returns (string memory) {}\n}",
		},
		{
			name:   "Windows line endings",
			source: "contract Test {\r\n    function version() public pure returns (string memory) {}\r\n}\r\n",
			want:   "contract Test {\n    function version() public pure returns (string memory) {}\n}",
		},
		{
			name:   "No trailing newline",
			source: "contract Test {}",
			want:   "contract Test {}",
		},
		{
			name:   "Only one trailing newline is ignored",
			source: "contract Test {}\r\n\r\n",
			want:   "contract Test {}\n",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.want, string(normalizeSourceCode([]byte(tt.source))))
		})
	}
}

func TestProcessFile_LineEndingsProduceSameSourceHash(t *testing.T) {
	t.Chdir(t.TempDir())
	require.NoError(t, os.Mkdir("src", 0755))

	artifact := `{
  "ast": {
    "nodes": [{
      "nodeType": "ContractDefinition",
      "name": "Test",
      "nodes": [{
        "nodeType": "FunctionDefinition",
        "name": "version",
        "documentation": {"text": "@custom:semver 1.0.0"}
      }]
    }]
  },
  "bytecode": {"object": "0x00"},
  "metadata": {
    "settings": {
      "compilationTarget": {"src/Test.sol": "Test"}
    }
  }
}`
	require.NoError(t, os.WriteFile("Test.json", []byte(artifact), 0644))

	require.NoError(t, os.WriteFile("src/Test.sol", []byte("contract Test {\n}\n"), 0644))
	lf, errs := processFile("Test.json")
	require.Empty(t, errs)
	require.NotNil(t, lf)

	require.NoError(t, os.WriteFile("src/Test.sol", []byte("contract Test {\r\n}\r\n"), 0644))
	crlf, errs := processFile("Test.json")
	require.Empty(t, errs)
	require.NotNil(t, crlf)

	require.Equal(t, lf.SemverLockOutput.SourceCodeHash, crlf.SemverLockOutput.SourceCodeHash)
}
