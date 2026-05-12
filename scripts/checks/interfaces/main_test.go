package main

import (
	"encoding/json"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestGetContractDefinition(t *testing.T) {
	artifact := &Artifact{
		AST: ArtifactAST{
			Nodes: []ASTNode{
				{NodeType: "ContractDefinition", ContractDefinition: ContractDefinition{ContractKind: "interface", Name: "ITest"}},
				{NodeType: "ContractDefinition", ContractDefinition: ContractDefinition{ContractKind: "contract", Name: "Test"}},
				{NodeType: "ContractDefinition", ContractDefinition: ContractDefinition{ContractKind: "library", Name: "TestLib"}},
			},
		},
	}

	tests := []struct {
		name         string
		contractName string
		want         *ContractDefinition
	}{
		{"Find interface", "ITest", &ContractDefinition{ContractKind: "interface", Name: "ITest"}},
		{"Find contract", "Test", &ContractDefinition{ContractKind: "contract", Name: "Test"}},
		{"Find library", "TestLib", &ContractDefinition{ContractKind: "library", Name: "TestLib"}},
		{"Not found", "NonExistent", nil},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.want, getContractDefinition(artifact, tt.contractName))
		})
	}
}

func TestGetContractSemver(t *testing.T) {
	tests := []struct {
		name     string
		artifact *Artifact
		want     string
		wantErr  bool
	}{
		{
			name: "Valid semver",
			artifact: &Artifact{
				AST: ArtifactAST{
					Nodes: []ASTNode{
						{NodeType: "PragmaDirective", Literals: []string{"solidity", "^", "0.8.0"}},
					},
				},
			},
			want: "solidity^0.8.0",
		},
		{
			name: "Returns first pragma directive",
			artifact: &Artifact{
				AST: ArtifactAST{
					Nodes: []ASTNode{
						{NodeType: "PragmaDirective", Literals: []string{"solidity", "^", "0.8.0"}},
						{NodeType: "PragmaDirective", Literals: []string{"abicoder", "v2"}},
					},
				},
			},
			want: "solidity^0.8.0",
		},
		{
			name: "No semver",
			artifact: &Artifact{
				AST: ArtifactAST{
					Nodes: []ASTNode{
						{NodeType: "ContractDefinition"},
					},
				},
			},
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := getContractSemver(tt.artifact)
			if tt.wantErr {
				require.Error(t, err)
				return
			}
			require.NoError(t, err)
			require.Equal(t, tt.want, got)
		})
	}
}

func TestNormalizeABI(t *testing.T) {
	tests := []struct {
		name string
		abi  string
		want string
	}{
		{
			name: "Replace interface types and add constructor",
			abi:  `[{"inputs":[{"internalType":"contract Test","name":"test","type":"address"}],"type":"function"}]`,
			want: `[{"inputs":[{"internalType":"contract ITest","name":"test","type":"address"}],"type":"function"},{"inputs":[],"stateMutability":"nonpayable","type":"constructor"}]`,
		},
		{
			name: "Convert __constructor__",
			abi:  `[{"type":"function","name":"__constructor__","inputs":[],"stateMutability":"nonpayable","outputs":[]}]`,
			want: `[{"type":"constructor","inputs":[],"stateMutability":"nonpayable"}]`,
		},
		{
			name: "Keep existing constructor",
			abi:  `[{"type":"constructor","inputs":[{"name":"param","type":"uint256"}]},{"type":"function","name":"test"}]`,
			want: `[{"type":"constructor","inputs":[{"name":"param","type":"uint256"}]},{"type":"function","name":"test"}]`,
		},
		{
			name: "Replace multiple interface types",
			abi:  `[{"inputs":[{"internalType":"contract Test1","name":"test1","type":"address"},{"internalType":"contract ITest2","name":"test2","type":"address"}],"type":"function"}]`,
			want: `[{"inputs":[{"internalType":"contract ITest1","name":"test1","type":"address"},{"internalType":"contract ITest2","name":"test2","type":"address"}],"type":"function"},{"inputs":[],"stateMutability":"nonpayable","type":"constructor"}]`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := normalizeABI(json.RawMessage(tt.abi))
			require.NoError(t, err)
			gotJSON, err := json.Marshal(got)
			require.NoError(t, err)
			require.JSONEq(t, tt.want, string(gotJSON))
		})
	}
}

func TestCompareABIs(t *testing.T) {
	tests := []struct {
		name string
		abi1 string
		abi2 string
		want bool
	}{
		{
			name: "Identical ABIs",
			abi1: `[{"type":"function","name":"test","inputs":[],"outputs":[]}]`,
			abi2: `[{"type":"function","name":"test","inputs":[],"outputs":[]}]`,
			want: true,
		},
		{
			name: "Different ABIs",
			abi1: `[{"type":"function","name":"test1","inputs":[],"outputs":[]}]`,
			abi2: `[{"type":"function","name":"test2","inputs":[],"outputs":[]}]`,
			want: false,
		},
		{
			name: "Different order, same content",
			abi1: `[{"type":"function","name":"test1","inputs":[],"outputs":[]},{"type":"function","name":"test2","inputs":[],"outputs":[]}]`,
			abi2: `[{"type":"function","name":"test2","inputs":[],"outputs":[]},{"type":"function","name":"test1","inputs":[],"outputs":[]}]`,
			want: true,
		},
		{
			name: "Different input types",
			abi1: `[{"type":"function","name":"test","inputs":[{"type":"uint256"}],"outputs":[]}]`,
			abi2: `[{"type":"function","name":"test","inputs":[{"type":"uint128"}],"outputs":[]}]`,
			want: false,
		},
		{
			name: "Different output types",
			abi1: `[{"type":"function","name":"test","inputs":[],"outputs":[{"type":"uint256"}]}]`,
			abi2: `[{"type":"function","name":"test","inputs":[],"outputs":[{"type":"uint128"}]}]`,
			want: false,
		},
		{
			name: "Interface is strict subset of contract",
			abi1: `[{"type":"function","name":"a","inputs":[],"outputs":[]}]`,
			abi2: `[{"type":"function","name":"a","inputs":[],"outputs":[]},{"type":"function","name":"b","inputs":[],"outputs":[]}]`,
			want: false,
		},
		{
			name: "Contract is strict subset of interface",
			abi1: `[{"type":"function","name":"a","inputs":[],"outputs":[]},{"type":"function","name":"b","inputs":[],"outputs":[]}]`,
			abi2: `[{"type":"function","name":"a","inputs":[],"outputs":[]}]`,
			want: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var abi1, abi2 []map[string]interface{}
			require.NoError(t, json.Unmarshal([]byte(tt.abi1), &abi1))
			require.NoError(t, json.Unmarshal([]byte(tt.abi2), &abi2))
			require.Equal(t, tt.want, compareABIs(abi1, abi2))
		})
	}
}

func TestNormalizeInternalType(t *testing.T) {
	tests := []struct {
		name         string
		internalType string
		want         string
	}{
		{"Replace contract X", "contract Test", "contract ITest"},
		{"Replace enum X", "enum MyEnum", "enum IMyEnum"},
		{"Replace struct I", "struct Whatever.MyStruct", "struct IWhatever.MyStruct"},
		{"Don't replace II", "contract IInternet", "contract IInternet"},
		{"Don't replace already-prefixed enum", "enum IMyEnum", "enum IMyEnum"},
		{"Don't replace already-prefixed dotted struct", "struct IWhatever.MyStruct", "struct IWhatever.MyStruct"},
		{"No replacement needed", "uint256", "uint256"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.want, normalizeInternalType(tt.internalType))
		})
	}
}
