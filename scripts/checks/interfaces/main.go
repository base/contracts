package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"regexp"
	"slices"
	"strings"

	"github.com/base/contracts/scripts/checks/common"
)

var internalTypeRegex = regexp.MustCompile(`(contract|struct|enum)\s+([^I]\w+|I[a-z]\w*)`)

func normalizeInternalType(internalType string) string {
	return internalTypeRegex.ReplaceAllString(internalType, "$1 I$2")
}

// excludeContracts is a list of contracts whose interfaces do not need to match perfectly.
var excludeContracts = []string{
	// External dependencies
	"IERC20", "IERC721", "IERC5267", "IERC721Enumerable", "IERC721Upgradeable", "IERC721Metadata",
	"IERC165", "IERC165Upgradeable", "ERC721TokenReceiver", "ERC1155TokenReceiver",
	"ERC777TokensRecipient", "Guard", "IProxy", "Vm", "VmSafe", "IMulticall3",
	"IERC721TokenReceiver", "IProxyCreationCallback", "IBeacon", "IEIP712",

	// Generic interfaces
	"IHasSuperchainConfig",

	// EAS
	"IEAS", "ISchemaResolver", "ISchemaRegistry",

	// TODO: Interfaces that need to be fixed
	"IInitializable", "IOptimismMintableERC20",
	"KontrolCheatsBase", "IResolvedDelegateProxy",
}

// excludeSourceContracts is a list of contracts that are allowed to not have interfaces
var excludeSourceContracts = []string{
	// Base contracts with no external functions
	"CrossDomainMessengerLegacySpacer0", "CrossDomainMessengerLegacySpacer1",

	// FIXME
	"WETH", "MIPS64",
}

type ContractDefinition struct {
	ContractKind string `json:"contractKind"`
	Name         string `json:"name"`
}

type ASTNode struct {
	NodeType string   `json:"nodeType"`
	Literals []string `json:"literals,omitempty"`
	ContractDefinition
}

type ArtifactAST struct {
	AbsolutePath string    `json:"absolutePath"`
	Nodes        []ASTNode `json:"nodes"`
}

type Artifact struct {
	AST ArtifactAST     `json:"ast"`
	ABI json.RawMessage `json:"abi"`
}

var (
	cwd          string
	artifactsDir string
)

func main() {
	var err error
	cwd, err = os.Getwd()
	if err != nil {
		fmt.Printf("error: %v\n", err)
		os.Exit(1)
	}
	artifactsDir = filepath.Join(cwd, "forge-artifacts")

	if _, err := common.ProcessFilesGlob(
		[]string{"forge-artifacts/**/*.json"},
		[]string{},
		processFile,
	); err != nil {
		fmt.Printf("error: %v\n", err)
		os.Exit(1)
	}
}

func processFile(artifactPath string) (*common.Void, []error) {
	contractName := strings.Split(filepath.Base(artifactPath), ".")[0]
	if slices.Contains(excludeContracts, contractName) {
		return nil, nil
	}

	artifact, err := readArtifact(artifactPath)
	if err != nil {
		return nil, []error{fmt.Errorf("failed to read artifact: %w", err)}
	}

	contractDef := getContractDefinition(artifact, contractName)
	if contractDef == nil {
		return nil, nil
	}

	if contractDef.ContractKind != "interface" {
		if contractDef.ContractKind != "contract" {
			return nil, nil
		}

		absPath := artifact.AST.AbsolutePath
		if !strings.HasPrefix(absPath, "src/") {
			return nil, nil
		}

		for _, folder := range []string{"src/libraries", "src/vendor"} {
			if strings.HasPrefix(absPath, folder) {
				return nil, nil
			}
		}

		if slices.Contains(excludeSourceContracts, contractName) {
			return nil, nil
		}

		dirPath := filepath.Dir(strings.TrimPrefix(absPath, "src/"))
		interfacePath := filepath.Join(cwd, "interfaces", dirPath, "I"+contractName+".sol")
		if _, err := os.Stat(interfacePath); errors.Is(err, os.ErrNotExist) {
			return nil, []error{fmt.Errorf("Contract %s in %s does not have a corresponding interface at %s",
				contractName, absPath, interfacePath)}
		}
		return nil, nil
	}

	if !strings.HasPrefix(contractName, "I") {
		return nil, []error{fmt.Errorf("%s: Interface does not start with 'I'", contractName)}
	}

	semver, err := getContractSemver(artifact)
	if err != nil {
		return nil, []error{fmt.Errorf("failed to get contract semver: %w", err)}
	}

	if semver != "solidity^0.8.0" {
		return nil, []error{fmt.Errorf("%s: Interface does not have correct compiler version (MUST be exactly solidity ^0.8.0)", contractName)}
	}

	contractBasename := contractName[1:]
	correspondingContractFile := filepath.Join(artifactsDir, contractBasename+".sol", contractBasename+".json")

	contractArtifact, err := readArtifact(correspondingContractFile)
	if errors.Is(err, os.ErrNotExist) {
		return nil, nil
	}
	if err != nil {
		return nil, []error{fmt.Errorf("failed to read corresponding contract artifact: %w", err)}
	}

	interfaceABI := artifact.ABI
	contractABI := contractArtifact.ABI

	normalizedInterfaceABI, err := normalizeABI(interfaceABI)
	if err != nil {
		return nil, []error{fmt.Errorf("failed to normalize interface ABI: %w", err)}
	}

	normalizedContractABI, err := normalizeABI(contractABI)
	if err != nil {
		return nil, []error{fmt.Errorf("failed to normalize contract ABI: %w", err)}
	}

	match, err := compareABIs(normalizedInterfaceABI, normalizedContractABI)
	if err != nil {
		return nil, []error{fmt.Errorf("failed to compare ABIs: %w", err)}
	}
	if !match {
		return nil, []error{fmt.Errorf("differences found")}
	}

	return nil, nil
}

func readArtifact(path string) (*Artifact, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("failed to open artifact file: %w", err)
	}
	defer file.Close()

	var artifact Artifact
	if err := json.NewDecoder(file).Decode(&artifact); err != nil {
		return nil, fmt.Errorf("failed to parse artifact file: %w", err)
	}

	return &artifact, nil
}

func getContractDefinition(artifact *Artifact, contractName string) *ContractDefinition {
	for _, node := range artifact.AST.Nodes {
		if node.NodeType == "ContractDefinition" && node.Name == contractName {
			return &node.ContractDefinition
		}
	}
	return nil
}

func getContractSemver(artifact *Artifact) (string, error) {
	for _, node := range artifact.AST.Nodes {
		if node.NodeType == "PragmaDirective" {
			return strings.Join(node.Literals, ""), nil
		}
	}
	return "", errors.New("semver not found")
}

func normalizeABI(abi json.RawMessage) (json.RawMessage, error) {
	var abiData []map[string]interface{}
	if err := json.Unmarshal(abi, &abiData); err != nil {
		return nil, err
	}

	hasConstructor := false
	for i := range abiData {
		normalizeABIItem(abiData[i])
		if abiData[i]["type"] == "constructor" {
			hasConstructor = true
		}
	}

	if !hasConstructor {
		emptyConstructor := map[string]interface{}{
			"type":            "constructor",
			"stateMutability": "nonpayable",
			"inputs":          []interface{}{},
		}
		abiData = append(abiData, emptyConstructor)
	}

	return json.Marshal(abiData)
}

func normalizeABIItem(item map[string]interface{}) {
	for key, value := range item {
		switch v := value.(type) {
		case string:
			if key == "internalType" {
				item[key] = normalizeInternalType(v)
			}
		case map[string]interface{}:
			normalizeABIItem(v)
		case []interface{}:
			for _, elem := range v {
				if elemMap, ok := elem.(map[string]interface{}); ok {
					normalizeABIItem(elemMap)
				}
			}
		}
	}

	if item["type"] == "function" && item["name"] == "__constructor__" {
		item["type"] = "constructor"
		delete(item, "name")
		delete(item, "outputs")
	}
}

func compareABIs(abi1, abi2 json.RawMessage) (bool, error) {
	var interfaceABI, contractABI []map[string]interface{}

	if err := json.Unmarshal(abi1, &interfaceABI); err != nil {
		return false, fmt.Errorf("error unmarshalling interface ABI: %w", err)
	}

	if err := json.Unmarshal(abi2, &contractABI); err != nil {
		return false, fmt.Errorf("error unmarshalling contract ABI: %w", err)
	}

	interfaceItems := make(map[string]map[string]interface{})
	contractItems := make(map[string]map[string]interface{})

	makeKey := func(item map[string]interface{}) string {
		itemType := getString(item, "type")
		itemName := getString(item, "name")
		inputs, _ := json.Marshal(item["inputs"])
		outputs, _ := json.Marshal(item["outputs"])
		return fmt.Sprintf("%s_%s_%s_%s", itemType, itemName, inputs, outputs)
	}

	for _, item := range interfaceABI {
		interfaceItems[makeKey(item)] = item
	}
	for _, item := range contractABI {
		contractItems[makeKey(item)] = item
	}

	isMatch := true
	for key, item := range interfaceItems {
		if _, exists := contractItems[key]; !exists {
			log.Printf("REMOVE %s from interface: %s", getString(item, "type"), formatABIItem(item))
			isMatch = false
		}
	}
	for key, item := range contractItems {
		if _, exists := interfaceItems[key]; !exists {
			log.Printf("ADD %s to interface: %s", getString(item, "type"), formatABIItem(item))
			isMatch = false
		}
	}

	return isMatch, nil
}

func formatABIItem(item map[string]interface{}) string {
	itemType := getString(item, "type")
	itemName := getString(item, "name")
	inputStr := formatABIParams(item["inputs"])
	outputStr := formatABIParams(item["outputs"])

	switch itemType {
	case "function":
		returnStr := ""
		if len(outputStr) > 0 {
			returnStr = fmt.Sprintf(" returns (%s)", strings.Join(outputStr, ", "))
		}
		return fmt.Sprintf("function %s(%s)%s", itemName, strings.Join(inputStr, ", "), returnStr)
	case "event":
		return fmt.Sprintf("event %s(%s)", itemName, strings.Join(inputStr, ", "))
	case "constructor":
		return fmt.Sprintf("constructor(%s)", strings.Join(inputStr, ", "))
	default:
		return fmt.Sprintf("%s %s(%s)", itemType, itemName, strings.Join(inputStr, ", "))
	}
}

func formatABIParams(raw interface{}) []string {
	params, _ := raw.([]interface{})
	out := make([]string, 0, len(params))
	for _, p := range params {
		paramMap, ok := p.(map[string]interface{})
		if !ok {
			continue
		}
		paramType := getString(paramMap, "internalType")
		if parts := strings.Fields(paramType); len(parts) == 2 {
			paramType = parts[1]
		}
		if paramName := getString(paramMap, "name"); paramName != "" {
			out = append(out, fmt.Sprintf("%s %s", paramType, paramName))
		} else {
			out = append(out, paramType)
		}
	}
	return out
}

func getString(m map[string]interface{}, key string) string {
	if val, ok := m[key]; ok {
		if str, ok := val.(string); ok {
			return str
		}
	}
	return ""
}
