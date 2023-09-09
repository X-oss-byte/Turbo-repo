//go:build rust
// +build rust

package lockfile

import (
	mapset "github.com/deckarep/golang-set"
	"github.com/vercel/turbo/cli/internal/turbopath"
)

// TransitiveClosure the set of all lockfile keys that pkg depends on
func TransitiveClosure(
	workspaceDir turbopath.AnchoredUnixPath,
	unresolvedDeps map[string]string,
	lockFile Lockfile,
) (mapset.Set, error) {
	if lf, ok := lockFile.(*NpmLockfile); ok {
		// We special case as Rust implementations have their own dep crawl
		return npmTransitiveDeps(lf, workspaceDir, unresolvedDeps)
	}
	return transitiveClosure(workspaceDir, unresolvedDeps, lockFile)
}
