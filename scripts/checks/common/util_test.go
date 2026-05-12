package common

import (
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"sync"
	"testing"

	"github.com/stretchr/testify/require"
)

func suppressErrorReporter(t *testing.T) {
	t.Helper()
	t.Setenv(EnvSuppressErrorReporter, "1")
}

func writeFiles(t *testing.T, files map[string]string) {
	t.Helper()
	for name, content := range files {
		require.NoError(t, os.WriteFile(name, []byte(content), 0644))
	}
}

func setupGlobFixture(t *testing.T) (includes, excludes []string) {
	t.Helper()
	t.Chdir(t.TempDir())
	writeFiles(t, map[string]string{
		"test1.txt": "content1",
		"test2.txt": "content2",
		"skip.txt":  "content3",
	})
	return []string{"*.txt"}, []string{"skip.txt"}
}

func TestErrorReporter(t *testing.T) {
	suppressErrorReporter(t)

	reporter := NewErrorReporter()
	require.False(t, reporter.HasError(), "new reporter should not have errors")

	reporter.Fail("test error")
	require.True(t, reporter.HasError(), "reporter should have error after Fail")
}

func TestProcessFiles(t *testing.T) {
	suppressErrorReporter(t)

	files := []string{"path1", "path2"}

	t.Run("void", func(t *testing.T) {
		_, err := ProcessFiles(files, func(path string) (*Void, []error) {
			return nil, nil
		})
		require.NoError(t, err)
	})

	t.Run("error", func(t *testing.T) {
		_, err := ProcessFiles(files, func(path string) (*Void, []error) {
			return nil, []error{os.ErrNotExist}
		})
		require.Error(t, err)
	})

	t.Run("string results", func(t *testing.T) {
		results, err := ProcessFiles(files, func(path string) (string, []error) {
			return "processed_" + path, nil
		})
		require.NoError(t, err)
		require.Equal(t, "processed_path1", results["path1"])
		require.Equal(t, "processed_path2", results["path2"])
	})

	t.Run("struct results", func(t *testing.T) {
		type testResult struct {
			Path    string
			Counter int
		}
		results, err := ProcessFiles(files, func(path string) (testResult, []error) {
			return testResult{Path: path, Counter: len(path)}, nil
		})
		require.NoError(t, err)
		require.Len(t, results, 2)
		require.Equal(t, 5, results["path1"].Counter)
	})
}

func TestProcessFilesGlob(t *testing.T) {
	suppressErrorReporter(t)
	includes, excludes := setupGlobFixture(t)

	t.Run("void", func(t *testing.T) {
		var mtx sync.Mutex
		processed := make(map[string]bool)
		_, err := ProcessFilesGlob(includes, excludes, func(path string) (*Void, []error) {
			mtx.Lock()
			defer mtx.Unlock()
			processed[filepath.Base(path)] = true
			return nil, nil
		})
		require.NoError(t, err)
		require.Equal(t, map[string]bool{"test1.txt": true, "test2.txt": true}, processed)
	})

	t.Run("struct results", func(t *testing.T) {
		type fileInfo struct{ Content string }
		results, err := ProcessFilesGlob(includes, excludes, func(path string) (fileInfo, []error) {
			content, err := os.ReadFile(path)
			if err != nil {
				return fileInfo{}, []error{err}
			}
			return fileInfo{Content: string(content)}, nil
		})
		require.NoError(t, err)
		require.Len(t, results, 2)
		require.Equal(t, "content1", results["test1.txt"].Content)
		require.Equal(t, "content2", results["test2.txt"].Content)
	})

	t.Run("error", func(t *testing.T) {
		_, err := ProcessFilesGlob(includes, excludes, func(path string) (struct{}, []error) {
			return struct{}{}, []error{fmt.Errorf("test error")}
		})
		require.Error(t, err)
	})
}

func TestFindFiles(t *testing.T) {
	includes, excludes := setupGlobFixture(t)

	found, err := FindFiles(includes, excludes)
	require.NoError(t, err)
	sort.Strings(found)
	require.Equal(t, []string{"test1.txt", "test2.txt"}, found)
}

func TestReadForgeArtifact(t *testing.T) {
	tmpFile := filepath.Join(t.TempDir(), "Test.json")
	artifactContent := `{
		"abi": [],
		"bytecode": {"object": "0x123"},
		"deployedBytecode": {"object": "0x456"}
	}`
	require.NoError(t, os.WriteFile(tmpFile, []byte(artifactContent), 0644))

	artifact, err := ReadForgeArtifact(tmpFile)
	require.NoError(t, err)
	require.Equal(t, "0x123", artifact.Bytecode.Object)
	require.Equal(t, "0x456", artifact.DeployedBytecode.Object)
}
