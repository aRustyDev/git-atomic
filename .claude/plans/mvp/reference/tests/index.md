# Chart Release Workflow Test Plan

## Overview

This test plan validates all controls in the chart release workflow pipeline:

```
Developer PR → W1 (Validate) → Auto-Merge → integration
                                    ↓
                              W2 (Filter Charts)
                                    ↓
                              charts/<chart> branch + PR to main
                                    ↓
                              W5 (Validate Atomic Chart PR)
                                    ↓
                              Human Review → Merge to main
                                    ↓
                              Release (Tag + Package + Publish)
```

## Phase Documents

| Phase | Document | Focus | Tests |
|-------|----------|-------|-------|
| 1 | [phase-1-auto-merge.md](./phase-1-auto-merge.md) | Auto-Merge workflow | AM-T1 to AM-T10 |
| 2 | [phase-2-w2-filter.md](./phase-2-w2-filter.md) | W2: Create Atomic Chart PR | W2-T1 to W2-T7 |
| 3 | [phase-3-w5-validate.md](./phase-3-w5-validate.md) | W5: Validate Atomic Chart PR | W5-T1 to W5-T13 |
| 4 | [phase-4-release.md](./phase-4-release.md) | Release workflow | R-T1 to R-T9 |
| 5 | [phase-5-attestation.md](./phase-5-attestation.md) | Attestation & Lineage | AT-T1 to AT-T8, AL-T1 to AL-T6 |
| 6 | [phase-6-e2e.md](./phase-6-e2e.md) | End-to-End scenarios | E2E-1 to E2E-10 |

Supporting documents:
- [gap-analysis.md](./gap-analysis.md) - Consolidated gap analysis from sub-agent review
- [cleanup.md](./cleanup.md) - Shared cleanup procedures
- [plan-archive.md](./plan-archive.md) - Original monolithic plan (reference)

---

## Critical Blocking Issues

**Must resolve before testing begins.** See [gap-analysis.md](./gap-analysis.md) for full details.

| Issue | Location | Impact | Status |
|-------|----------|--------|--------|
| ~~Action output field names~~ | `verify-signatures` action | ~~All signature tests fail~~ | ✅ Resolved - Updated test plan |
| ~~Dispatch trigger auth~~ | `dispatch/trigger` action | ~~W5 never triggered~~ | ✅ Verified - GITHUB_TOKEN works |
| Attestation map preservation | `create-pr` action | Lineage chain breaks on PR updates | Add preservation test |

---

## Prerequisites

### Repository Configuration

| Requirement | Location | Value |
|-------------|----------|-------|
| Auto-merge enabled | Settings → General → Pull Requests | ✓ Allow auto-merge |
| `AUTO_MERGE_ALLOWED_BRANCHES` | Settings → Secrets and variables → Actions → Variables | `integration` |
| Branch protection (integration) | Settings → Branches | Require PR, status checks |
| Branch protection (main) | Settings → Branches | Require PR, status checks, review |
| CODEOWNERS | `.github/CODEOWNERS` | Lists trusted contributors |

### Test User Accounts

| Account | Purpose | Requirements |
|---------|---------|--------------|
| Trusted User | Tests pass scenarios | Listed in CODEOWNERS, has signing key |
| Untrusted User | Tests fail scenarios | NOT in CODEOWNERS |

### Test Chart

The `test-workflow` chart is used for all testing:

```yaml
# charts/test-workflow/Chart.yaml
apiVersion: v2
name: test-workflow
description: Test chart for workflow validation
type: application
version: 0.1.0
appVersion: "1.0.0"
```

---

## Test Execution Order

### Dependencies

Some tests must run in sequence:

1. AM tests should complete before E2E tests
2. W2 tests require integration branch access
3. W5 tests require W2 to create the PR first
4. Release tests require W5 to complete

### Recommended Execution Order

```
Phase 1 (Auto-Merge) → Phase 2 (W2) → Phase 3 (W5) → Phase 4 (Release)
                                                            ↓
                                                    Phase 5 (Attestation)
                                                            ↓
                                                    Phase 6 (E2E)
```

---

## Test Isolation

To avoid interference:

- Use unique chart names for parallel tests
- Use unique branch names with test ID prefix
- Clean up immediately after each test

---

## Failure Investigation

When a test fails:

1. Check workflow run logs
2. Check branch protection rules
3. Check repository variables
4. Check CODEOWNERS file
5. Document in test results

---

## Execution Checklist

### Phase 1: Auto-Merge Tests
- [ ] AM-T1: W1 failure prevents trigger
- [ ] AM-T5: Untrusted author blocked
- [ ] AM-T7: Unsigned commits blocked
- [ ] AM-T9: Happy path works

### Phase 2: W2 Tests
- [ ] W2-T1: Path filtering works
- [ ] W2-T5: Single chart creates branch/PR
- [ ] W2-T6: Multiple charts handled

### Phase 3: W5 Tests
- [ ] W5-T4: Invalid branch pattern rejected
- [ ] W5-T5: Valid branch accepted
- [ ] W5-T10: Patch version bump
- [ ] W5-T11: Minor version bump
- [ ] W5-T13: Cleanup on merge

### Phase 4: Release Tests
- [ ] R-T3: Duplicate tag handling
- [ ] R-T7: GHCR push + signing
- [ ] R-T8: GitHub Release created

### Phase 5: Attestation Tests
- [ ] AT-T2: Package has attestation
- [ ] AT-T4: Verify package attestation
- [ ] AT-T5: Verify Cosign signature
- [ ] AT-T6: Attestation contains build metadata
- [ ] AT-T7: Tampered package fails verification
- [ ] AL-T1: Trace release to merge commit
- [ ] AL-T5: Full lineage audit

### Phase 6: End-to-End
- [ ] E2E-1: Happy path complete
- [ ] E2E-2: Untrusted user flow
- [ ] E2E-3: Unsigned commit flow
- [ ] E2E-4: Multiple charts
- [ ] E2E-5: Missing attestation map handling
- [ ] E2E-6: K8s test failure blocks release
- [ ] E2E-7: Lint failure blocks release
- [ ] E2E-8: Version bump failure handling
- [ ] E2E-9: Attestation verification failure detection
- [ ] E2E-10: Full lineage trace verification
