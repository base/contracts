# Nested Multisig Configuration Guide

Complete guide for configuring and testing the `NestedMultiSigDeploy.s.sol` script with various JSON configurations.

## 🚀 Quick Start

1. **Choose a test file** based on your needs from the table below
2. **Update the script** to use your chosen config:
   ```solidity
   string memory json = vm.readFile("config/test/YOUR_CONFIG.json");
   // The script automatically reads the safeCount field
   ```
3. **Run the deployment** script

## 📋 Available Test Files

| File | SafeCount | Purpose | Best For |
|------|-----------|---------|----------|
| `test-minimal.json` | 2 | Quick smoke test | Development, debugging |
| `test-simple.json` | 3 | No nesting baseline | Testing basic deployment |
| `test-one-level.json` | 3 | Single level nesting | Basic nested functionality |
| `test-multi-level.json` | 6 | Complex hierarchy | Advanced hierarchical structures |
| `test-edge-cases.json` | 4 | Boundary conditions | Edge case validation |
| `test-large-scale.json` | 8 | Performance testing | Scale and gas analysis |
| `../safes-nested.json` | 4 | Original example | Real-world scenarios |
| `../safes.json` | 2 | Original simple | Baseline comparison |

## ⚡ JSON Structure (Quick Reference)

```json
{
  "safeCount": 3,
  "safes": [
    {
      "label": "UniqueName",           // String: Safe identifier
      "threshold": 2,                  // Number: Required signatures
      "owners": ["0x123..."],          // Array: Direct address owners
      "ownerRefIndices": [0, 1]        // Array: References to other safes
    }
  ]
}
```

## ⚠️ Critical Rules

1. **SafeCount Match**: The `safeCount` field must match the actual number of safes in the array
2. **Order Matters**: Referenced safes must appear earlier in the array
3. **Threshold Math**: `threshold ≤ (direct_owners + referenced_safes)`
4. **Valid Addresses**: All addresses must be 42-char hex with `0x` prefix
5. **Array Bounds**: All `ownerRefIndices` must be valid array positions
6. **Unique Labels**: Each safe needs a unique identifier

## 🛠️ Common Issues & Quick Fixes

| Error | Cause | Fix |
|-------|-------|-----|
| "Reference not deployed" | Wrong array order | Move referenced safes earlier |
| Deployment fails | Threshold too high | Reduce threshold or add owners |
| JSON parse error | Invalid format | Check commas, quotes, addresses |
| Index out of bounds | Invalid reference | Check array indices are valid |
| SafeCount mismatch | Count ≠ array length | Update safeCount to match array |

## 🎯 Testing Strategy

### 🚀 Development Phase
1. `test-minimal.json` - Basic functionality
2. `test-simple.json` - Non-nested deployment  
3. `test-one-level.json` - Basic nesting

### 🔧 Integration Testing
1. `test-multi-level.json` - Complex scenarios
2. `test-edge-cases.json` - Boundary conditions
3. `test-large-scale.json` - Performance testing

### ✅ Production Validation
1. `safes-nested.json` - Realistic scenarios
2. Custom configs - Your specific needs

---

# 📖 Comprehensive Guide

## Overview

The NestedMultiSigDeploy script enables deployment of hierarchical Safe multisig wallets where higher-level safes can include lower-level safes as owners. This creates complex governance structures where groups of multisigs can collectively control other multisigs.

### Key Features
- **Sequential Deployment**: Safes are deployed in array order with automatic dependency resolution
- **Nested Ownership**: Previously deployed safes can be owners of new safes
- **Index-Based References**: Uses array indices to reference previously deployed safes as owners
- **Timestamp-Based Nonces**: Automatic CREATE2 nonce generation using timestamps
- **Console Logging**: Comprehensive deployment tracking and verification
- **SafeCount Field**: Explicit configuration of expected safe count

## Script Architecture

### Core Components

```solidity
struct SafeWallet {
    string label;              // Unique identifier for the safe
    uint256 threshold;         // Required signatures (1 to total owners)
    address[] owners;          // Direct address owners
    uint256[] ownerRefIndices; // References to other safes by array index
}
```

### Deployment Process
1. **Parse JSON**: Read configuration and safeCount field
2. **Sequential Deploy**: Deploy safes in array order (0, 1, 2, ...)
3. **Resolve Owners**: Combine direct addresses + referenced safe addresses
4. **Initialize**: Set up each safe with resolved owners and threshold

## JSON Configuration Structure

### Complete Template

```json
{
  "safeCount": 4,
  "safes": [
    {
      "label": "Level1_Safe1",
      "threshold": 2,
      "owners": [
        "0x2f6A7573933b8534B9BBA414dDfec8bde7ae11E2",
        "0xb892fd5Bee06BCb97cf32E33071260b84ca7F9b6",
        "0xF8a830fDE2D5890802AD7479436dE12Be83010D5"
      ],
      "ownerRefIndices": []
    },
    {
      "label": "Level1_Safe2", 
      "threshold": 3,
      "owners": [
        "0x3f6051c46b34Ae8df47F27662863d60ae07A9ae2",
        "0xc8af0a5D7b93019a0F67c141a64C5c23d532e02a",
        "0x1234567890123456789012345678901234567890"
      ],
      "ownerRefIndices": []
    },
    {
      "label": "Treasury_Main",
      "threshold": 2,
      "owners": [
        "0x2f6A7573933b8534B9BBA414dDfec8bde7ae11E2"
      ],
      "ownerRefIndices": [0, 1]
    },
    {
      "label": "Operations_Main",
      "threshold": 1,
      "owners": [
        "0xb892fd5Bee06BCb97cf32E33071260b84ca7F9b6"
      ],
      "ownerRefIndices": [2]
    }
  ]
}
```

### Field Specifications

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `safeCount` | number | ✅ | Total number of safes to deploy |
| `label` | string | ✅ | Unique identifier (no duplicates) |
| `threshold` | number | ✅ | Required signatures (1 ≤ threshold ≤ total_owners) |
| `owners` | address[] | ✅ | Array of Ethereum addresses (can be empty) |
| `ownerRefIndices` | number[] | ✅ | Array of safe indices (can be empty) |

## Validation Rules

### Critical Requirements

1. **SafeCount Match**: The `safeCount` field must match the actual number of safes in the array
2. **Deployment Order**: Referenced safes MUST appear earlier in the array
   ```json
   // ✅ CORRECT: Safe at index 2 references safes at indices 0,1
   {
     "safeCount": 3,
     "safes": [
       { "label": "TeamA", "ownerRefIndices": [] },      // Index 0
       { "label": "TeamB", "ownerRefIndices": [] },      // Index 1
       { "label": "Main", "ownerRefIndices": [0, 1] }    // Index 2
     ]
   }
   
   // ❌ WRONG: Safe at index 0 cannot reference index 1 (not deployed yet)
   {
     "safes": [
       { "label": "Main", "ownerRefIndices": [1] },      // Index 0
       { "label": "TeamA", "ownerRefIndices": [] }       // Index 1
     ]
   }
   ```

3. **Threshold Validation**: `threshold ≤ (direct owners + referenced safes)`
   ```json
   // ✅ CORRECT: 2 threshold with 3 total owners (1 direct + 2 referenced)
   {
     "threshold": 2,
     "owners": ["0x1234..."],
     "ownerRefIndices": [0, 1]
   }
   
   // ❌ WRONG: 4 threshold with only 2 total owners
   {
     "threshold": 4,
     "owners": ["0x1234..."],
     "ownerRefIndices": [0]
   }
   ```

4. **Address Format**: All addresses must be valid 42-character hex strings
   ```json
   // ✅ CORRECT
   "owners": ["0x2f6A7573933b8534B9BBA414dDfec8bde7ae11E2"]
   
   // ❌ WRONG
   "owners": ["2f6A7573933b8534B9BBA414dDfec8bde7ae11E2"]    // Missing 0x
   "owners": ["0x2f6A7573933b8534B9BBA414dDfec8bde7ae11E"]     // Too short
   ```

5. **Index Bounds**: All `ownerRefIndices` must be valid array indices
   ```json
   // ✅ CORRECT: For a 3-safe array, valid indices are 0, 1, 2
   "ownerRefIndices": [0, 1]
   
   // ❌ WRONG: Index 3 doesn't exist in a 3-safe array
   "ownerRefIndices": [0, 3]
   ```

6. **Label Uniqueness**: Each safe must have a unique label
   ```json
   // ❌ WRONG: Duplicate labels
   {
     "safes": [
       { "label": "Team" },
       { "label": "Team" }  // Duplicate!
     ]
   }
   ```

## Step-by-Step Examples

### Example 1: Simple Independent Safes

```json
{
  "safeCount": 4,
  "safes": [
    {
      "label": "Treasury",
      "threshold": 2,
      "owners": [
        "0x2f6A7573933b8534B9BBA414dDfec8bde7ae11E2",
        "0xb892fd5Bee06BCb97cf32E33071260b84ca7F9b6",
        "0xF8a830fDE2D5890802AD7479436dE12Be83010D5"
      ],
      "ownerRefIndices": []
    },
    {
      "label": "Operations",
      "threshold": 1,
      "owners": [
        "0x3f6051c46b34Ae8df47F27662863d60ae07A9ae2",
        "0xc8af0a5D7b93019a0F67c141a64C5c23d532e02a"
      ],
      "ownerRefIndices": []
    },
    {
      "label": "Marketing",
      "threshold": 1,
      "owners": [
        "0x1234567890123456789012345678901234567890"
      ],
      "ownerRefIndices": []
    },
    {
      "label": "Development",
      "threshold": 2,
      "owners": [
        "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd",
        "0x9876543210987654321098765432109876543210"
      ],
      "ownerRefIndices": []
    }
  ]
}
```

**Analysis**: 4 independent safes, no nesting, different thresholds.

### Example 2: Single-Level Nesting

```json
{
  "safeCount": 4,
  "safes": [
    {
      "label": "DevTeam",
      "threshold": 2,
      "owners": [
        "0x2f6A7573933b8534B9BBA414dDfec8bde7ae11E2",
        "0xb892fd5Bee06BCb97cf32E33071260b84ca7F9b6",
        "0xF8a830fDE2D5890802AD7479436dE12Be83010D5"
      ],
      "ownerRefIndices": []
    },
    {
      "label": "SecurityTeam", 
      "threshold": 2,
      "owners": [
        "0x3f6051c46b34Ae8df47F27662863d60ae07A9ae2",
        "0xc8af0a5D7b93019a0F67c141a64C5c23d532e02a"
      ],
      "ownerRefIndices": []
    },
    {
      "label": "EmptyPlaceholder",
      "threshold": 1,
      "owners": [
        "0x1234567890123456789012345678901234567890"
      ],
      "ownerRefIndices": []
    },
    {
      "label": "MainTreasury",
      "threshold": 2,
      "owners": [
        "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd"
      ],
      "ownerRefIndices": [0, 1]
    }
  ]
}
```

**Analysis**: 
- Safes 0,1,2: Independent teams
- Safe 3: References safes 0,1 as owners
- Total owners for MainTreasury: 3 (1 direct + 2 referenced)
- Threshold 2 is valid (≤ 3 total owners)

### Example 3: Multi-Level Hierarchy

```json
{
  "safeCount": 4,
  "safes": [
    {
      "label": "Level1_TeamA",
      "threshold": 2,
      "owners": [
        "0x2f6A7573933b8534B9BBA414dDfec8bde7ae11E2",
        "0xb892fd5Bee06BCb97cf32E33071260b84ca7F9b6",
        "0xF8a830fDE2D5890802AD7479436dE12Be83010D5"
      ],
      "ownerRefIndices": []
    },
    {
      "label": "Level1_TeamB",
      "threshold": 2,
      "owners": [
        "0x3f6051c46b34Ae8df47F27662863d60ae07A9ae2",
        "0xc8af0a5D7b93019a0F67c141a64C5c23d532e02a"
      ],
      "ownerRefIndices": []
    },
    {
      "label": "Level2_Department",
      "threshold": 2,
      "owners": [
        "0x1234567890123456789012345678901234567890"
      ],
      "ownerRefIndices": [0, 1]
    },
    {
      "label": "Level3_Organization",
      "threshold": 1,
      "owners": [
        "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd"
      ],
      "ownerRefIndices": [2]
    }
  ]
}
```

**Analysis**: 
- Level 1: TeamA, TeamB (independent)
- Level 2: Department (controlled by TeamA + TeamB)
- Level 3: Organization (controlled by Department + 1 direct address)

### Example 4: Edge Cases

```json
{
  "safeCount": 4,
  "safes": [
    {
      "label": "SingleOwner",
      "threshold": 1,
      "owners": [
        "0x2f6A7573933b8534B9BBA414dDfec8bde7ae11E2"
      ],
      "ownerRefIndices": []
    },
    {
      "label": "OnlyReferenced",
      "threshold": 1,
      "owners": [],
      "ownerRefIndices": [0]
    },
    {
      "label": "HighThreshold",
      "threshold": 4,
      "owners": [
        "0xb892fd5Bee06BCb97cf32E33071260b84ca7F9b6",
        "0xF8a830fDE2D5890802AD7479436dE12Be83010D5",
        "0x3f6051c46b34Ae8df47F27662863d60ae07A9ae2",
        "0xc8af0a5D7b93019a0F67c141a64C5c23d532e02a"
      ],
      "ownerRefIndices": []
    },
    {
      "label": "MixedOwnership",
      "threshold": 3,
      "owners": [
        "0x1234567890123456789012345678901234567890",
        "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd"
      ],
      "ownerRefIndices": [1]
    }
  ]
}
```

**Analysis**:
- SingleOwner: 1-of-1 multisig
- OnlyReferenced: No direct owners, only referenced safe
- HighThreshold: 4-of-4 multisig (requires all signatures)
- MixedOwnership: 2 direct + 1 referenced = 3 total, threshold 3

## Usage Examples

### Switching Configurations

```solidity
// Original
string memory json = vm.readFile("config/test/test-simple.json");
// Script automatically reads safeCount: 3

// Switch to minimal test
string memory json = vm.readFile("config/test/test-minimal.json");
// Script automatically reads safeCount: 2

// Switch to complex hierarchy
string memory json = vm.readFile("config/test/test-multi-level.json");
// Script automatically reads safeCount: 6
```

## Troubleshooting

### Common Errors and Solutions

#### 1. "Reference not deployed" Error

**Error**: `Reference not deployed: SafeName`

**Cause**: Trying to reference a safe that appears later in the array

**Solution**: Reorder your safes so referenced safes come first
```json
// ❌ WRONG
{
  "safes": [
    { "label": "Main", "ownerRefIndices": [1] },  // References index 1
    { "label": "Team", "ownerRefIndices": [] }    // Index 1
  ]
}

// ✅ CORRECT  
{
  "safes": [
    { "label": "Team", "ownerRefIndices": [] },   // Index 0
    { "label": "Main", "ownerRefIndices": [0] }   // References index 0
  ]
}
```

#### 2. SafeCount Mismatch Error

**Error**: Script deployment fails or behaves unexpectedly

**Cause**: `safeCount` field doesn't match actual number of safes in array

**Solution**: Update safeCount to match array length
```json
// ❌ WRONG: safeCount says 4 but only 2 safes in array
{
  "safeCount": 4,
  "safes": [
    { "label": "Safe1", ... },
    { "label": "Safe2", ... }
  ]
}

// ✅ CORRECT
{
  "safeCount": 2,
  "safes": [
    { "label": "Safe1", ... },
    { "label": "Safe2", ... }
  ]
}
```

#### 3. Threshold Too High Error

**Error**: Safe deployment fails silently or reverts

**Cause**: Threshold exceeds total number of owners

**Solution**: Check your math
```json
// ❌ WRONG: 3 threshold with only 2 owners
{
  "threshold": 3,
  "owners": ["0x123..."],
  "ownerRefIndices": [0]
}

// ✅ CORRECT: 2 threshold with 2 owners
{
  "threshold": 2, 
  "owners": ["0x123..."],
  "ownerRefIndices": [0]
}
```

#### 4. JSON Parsing Errors

**Error**: Script fails to parse JSON

**Common Causes & Solutions**:
- **Missing commas**: Add commas between array elements and object properties
- **Extra commas**: Remove trailing commas
- **Quote mismatch**: Ensure all strings use double quotes
- **Invalid addresses**: Check address format (42 chars, starts with 0x)

#### 5. Array Index Out of Bounds

**Error**: Index references non-existent safe

**Solution**: Verify all indices in `ownerRefIndices` are valid
```json
// For a 4-safe array, valid indices are: 0, 1, 2, 3
// ❌ WRONG: Index 4 doesn't exist
"ownerRefIndices": [0, 4]

// ✅ CORRECT
"ownerRefIndices": [0, 1]
```

### Debugging Tips

1. **Start Small**: Begin with `test-minimal.json` and gradually increase complexity
2. **Check Console Output**: The script logs detailed deployment information
3. **Validate JSON**: Use a JSON validator before running the script
4. **Verify SafeCount**: Double-check that safeCount matches array length
5. **Count Carefully**: Double-check array indices and owner counts
6. **Test Locally**: Use a local network for testing before mainnet deployment

## Advanced Usage

### Creating Custom Configurations

1. **Plan Your Hierarchy**: Draw out the governance structure first
2. **Start with Base Layer**: Create independent safes first
3. **Add Levels Gradually**: Build up the hierarchy layer by layer
4. **Set SafeCount**: Always set safeCount to match your array length
5. **Validate Each Step**: Test each level before adding the next

### Example: Corporate Governance Structure

```json
{
  "safeCount": 4,
  "safes": [
    {
      "label": "Engineering_Team",
      "threshold": 3,
      "owners": [
        "0xEngineer1...",
        "0xEngineer2...", 
        "0xEngineer3...",
        "0xEngineer4..."
      ],
      "ownerRefIndices": []
    },
    {
      "label": "Marketing_Team",
      "threshold": 2,
      "owners": [
        "0xMarketer1...",
        "0xMarketer2...",
        "0xMarketer3..."
      ],
      "ownerRefIndices": []
    },
    {
      "label": "Executive_Board",
      "threshold": 2,
      "owners": [
        "0xCEO...",
        "0xCTO..."
      ],
      "ownerRefIndices": []
    },
    {
      "label": "Company_Treasury",
      "threshold": 2,
      "owners": [
        "0xCFO..."
      ],
      "ownerRefIndices": [0, 1, 2]
    }
  ]
}
```

This creates a structure where:
- Teams operate independently
- Company Treasury requires 2 of 4 signatures (CFO + any 2 teams/executives)
- Provides both operational flexibility and governance oversight

## Best Practices

1. **Test First**: Always test on testnets before mainnet
2. **Verify SafeCount**: Double-check that safeCount matches array length
3. **Verify Addresses**: Double-check all owner addresses
4. **Document Structure**: Keep clear records of your hierarchy
5. **Incremental Deployment**: Start simple, add complexity gradually
6. **Backup Configuration**: Version control your JSON files
7. **Use Meaningful Labels**: Choose descriptive names for easy identification
8. **Validate JSON**: Use online validators to check syntax before deployment

## Migration Guide

### From Old Format (without safeCount):

1. Add `"safeCount": X` at the top of your JSON where X is the number of safes
2. Ensure the count matches your actual safe array length
3. All existing `ownerRefIndices` arrays will continue to work

### Adding Nested Safes:

1. Add base safes first (with empty `ownerRefIndices`)
2. Add dependent safes with proper index references
3. Update `safeCount` to reflect total number of safes
4. Maintain array order for dependencies

---

**📋 Remember**: Always test on a local/testnet environment before mainnet deployment! 