package common

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"runtime"
	"sync"
	"sync/atomic"

	"github.com/bmatcuk/doublestar/v4"
	"github.com/ethereum-optimism/optimism/op-chain-ops/solc"
	"golang.org/x/sync/errgroup"
)

// EnvSuppressErrorReporter, when set, silences ErrorReporter stderr output. Used by tests.
const EnvSuppressErrorReporter = "SUPPRESS_ERROR_REPORTER"

type ErrorReporter struct {
	hasErr atomic.Bool
	outMtx sync.Mutex
}

func NewErrorReporter() *ErrorReporter {
	return &ErrorReporter{}
}

func (e *ErrorReporter) Fail(msg string, args ...any) {
	e.outMtx.Lock()
	if os.Getenv(EnvSuppressErrorReporter) == "" {
		_, _ = fmt.Fprintf(os.Stderr, "❌  "+msg+"\n", args...)
	}
	e.outMtx.Unlock()
	e.hasErr.Store(true)
}

func (e *ErrorReporter) HasError() bool {
	return e.hasErr.Load()
}

type Void struct{}

type FileProcessor[T any] func(path string) (T, []error)

func ProcessFiles[T any](files []string, processor FileProcessor[T]) (map[string]T, error) {
	g := errgroup.Group{}
	g.SetLimit(runtime.NumCPU())

	reporter := NewErrorReporter()
	results := make(map[string]T, len(files))
	var mtx sync.Mutex

	for _, path := range files {
		g.Go(func() error {
			result, errs := processor(path)
			if len(errs) > 0 {
				for _, err := range errs {
					reporter.Fail("%s: %v", path, err)
				}
				return nil
			}
			mtx.Lock()
			results[path] = result
			mtx.Unlock()
			return nil
		})
	}
	_ = g.Wait()

	if reporter.HasError() {
		return nil, fmt.Errorf("processing failed")
	}
	return results, nil
}

func ProcessFilesGlob[T any](includes, excludes []string, processor FileProcessor[T]) (map[string]T, error) {
	files, err := FindFiles(includes, excludes)
	if err != nil {
		return nil, err
	}
	return ProcessFiles(files, processor)
}

func FindFiles(includes, excludes []string) ([]string, error) {
	included, err := globAll(includes)
	if err != nil {
		return nil, err
	}
	excluded, err := globAll(excludes)
	if err != nil {
		return nil, err
	}

	files := make([]string, 0, len(included))
	for path := range included {
		if _, skip := excluded[path]; !skip {
			files = append(files, path)
		}
	}
	return files, nil
}

func globAll(patterns []string) (map[string]struct{}, error) {
	out := make(map[string]struct{})
	for _, pattern := range patterns {
		matches, err := doublestar.Glob(os.DirFS("."), pattern)
		if err != nil {
			return nil, fmt.Errorf("glob pattern error: %w", err)
		}
		for _, match := range matches {
			out[match] = struct{}{}
		}
	}
	return out, nil
}

func ReadForgeArtifact(path string) (*solc.ForgeArtifact, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("failed to read artifact: %w", err)
	}

	var artifact solc.ForgeArtifact
	if err := json.Unmarshal(data, &artifact); err != nil {
		return nil, fmt.Errorf("failed to parse artifact: %w", err)
	}

	return &artifact, nil
}

func WriteJSON(data any, path string) error {
	var out bytes.Buffer
	enc := json.NewEncoder(&out)
	enc.SetEscapeHTML(false)
	enc.SetIndent("", "  ")
	if err := enc.Encode(data); err != nil {
		return fmt.Errorf("failed to encode data: %w", err)
	}
	jsonData := bytes.TrimRight(out.Bytes(), "\n")
	if err := os.WriteFile(path, jsonData, 0644); err != nil {
		return fmt.Errorf("failed to write file: %w", err)
	}
	return nil
}
