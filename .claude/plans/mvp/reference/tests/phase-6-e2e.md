# Phase 6: End-to-End Test Scenarios

**Purpose:** Validate complete workflow pipeline from contribution to release

---

## E2E-1: Happy Path (Full Pipeline)

**Objective:** Verify complete workflow from contribution to release.

### Steps

1. Create feature branch from `integration`
2. Add minor feature to `charts/test-workflow` (bump minor)
3. Commit with `feat(test-workflow): add test feature` (signed)
4. Push and create PR to `integration`
5. Verify W1 passes
6. Verify auto-merge enables (if trusted + verified)
7. PR merges to `integration`
8. Verify W2 creates `charts/test-workflow` branch + PR to `main`
9. Verify W5 runs validation + bumps version
10. Approve and merge PR to `main`
11. Verify Release workflow creates tag + publishes

### Expected

- Version bumped from 0.1.0 → 0.2.0
- Tag `test-workflow-v0.2.0` created
- Package in GHCR with Cosign signature
- GitHub Release created with attestation
- Attestation verifiable via `gh attestation verify`

### Post-Release Verification

```bash
# Verify GitHub attestation
gh attestation verify <release-asset>.tgz --repo aRustyDev/helm-charts

# Verify Cosign signature
cosign verify ghcr.io/arustydev/helm-charts/test-workflow:0.2.0 \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com

# Verify lineage (attestation contains correct commit SHA)
gh attestation verify <release-asset>.tgz --repo aRustyDev/helm-charts --format json | \
  jq '.attestations[0].bundle.dsseEnvelope.payload' | base64 -d | \
  jq '.predicate.invocation.configSource.digest.sha1'
```

### Cleanup

- Delete `charts/test-workflow` branch
- Keep tag and release (document in test log)

---

## E2E-2: Untrusted Contributor Path

**Objective:** Verify untrusted contributors require manual review at integration.

### Steps

1. Fork repo as untrusted user
2. Create same feature change
3. Create PR from fork to `integration`
4. Verify W1 passes
5. Verify auto-merge does NOT enable
6. Manual review and merge required
7. Verify rest of pipeline works normally

### Expected

- Auto-merge skipped (trust check: `is-owner=false`)
- Manual merge works
- W2 → W5 → Release functions normally

### Cleanup

- Close PR if not merged
- Delete any test branches

---

## E2E-3: Unsigned Commit Path

**Objective:** Verify unsigned commits require manual review.

### Steps

1. Create feature branch
2. Commit with `--no-gpg-sign`
3. Create PR to `integration`
4. Verify W1 passes
5. Verify auto-merge does NOT enable (unverified commits)
6. Manually merge
7. Verify rest of pipeline works

### Expected

- Auto-merge skipped due to unverified commits (`all-verified=false`)
- Pipeline continues after manual merge

### Cleanup

- Delete test branches

---

## E2E-4: Multiple Charts Changed

**Objective:** Verify multiple charts are processed independently.

### Steps

1. Change both `charts/test-workflow` and `charts/cloudflared`
2. Merge to `integration`
3. Verify W2 creates TWO branches and TWO PRs
4. Verify W5 runs on each independently
5. Merge both PRs
6. Verify Release creates separate tags/releases

### Expected

- Two independent pipelines
- Each chart versioned separately
- Each chart released separately

### Cleanup

- Delete test branches
- Revert test changes if needed

---

## E2E-5: Missing Attestation Map Blocks Release

**Objective:** Verify that releases cannot proceed without valid attestation maps.

**Scenario:** A PR is merged to main that bypasses W5 validation (e.g., manually created PR without going through W2).

### Steps

1. Create `charts/test-workflow` branch directly from main (bypassing integration/W2)
2. Make a chart change with version bump
3. Create PR to main from this branch
4. PR passes W5 lint tests but has no attestation map (wasn't created by W2)
5. Attempt to merge and observe release workflow

### Expected

- W5 validation jobs run and pass
- PR description has NO `<!-- ATTESTATION_MAP -->` section
- Release workflow runs but logs warning about missing attestation data
- Release proceeds BUT tag annotation shows "No attestation data available"
- **Future improvement:** Block release if attestation map missing

### Current Behavior (Documents Gap)

- Release currently proceeds without attestation map
- This is a known limitation

### Cleanup

- Delete test branch
- Delete tag if created
- Delete release if created

---

## E2E-6: K8s Test Failure Blocks Release

**Objective:** Verify that a K8s test failure in W5 prevents the chart from being released.

### Steps

1. Create chart change that passes lint but fails install tests
   ```yaml
   # Example: Invalid resource that passes lint but fails install
   apiVersion: v1
   kind: ConfigMap
   metadata:
     name: {{ .Release.Name }}-test
   data:
     key: {{ required "value is required" .Values.nonexistent }}
   ```
2. Merge to integration (W1 passes, only lint checks)
3. W2 creates atomic PR to main
4. W5 runs: lint passes, K8s matrix tests FAIL
5. PR cannot merge (required checks fail)

### Expected

- W5 k8s-test jobs fail
- Required status checks block merge
- No release is created
- Attestation map shows test failures

### Verification

```bash
# Check PR status
gh pr view <pr-number> --json statusCheckRollup \
  --jq '.statusCheckRollup[] | select(.conclusion != "SUCCESS")'

# Verify no tag was created
git tag -l "test-workflow-*" | tail -1
```

### Cleanup

- Fix the chart or close PR
- Delete test branches

---

## E2E-7: Lint Failure Blocks Release

**Objective:** Verify that lint failures in W5 prevent release.

### Steps

1. Create chart change with lint errors (but valid YAML)
   ```yaml
   # Missing required Chart.yaml fields
   apiVersion: v2
   name: test-workflow
   # Missing: version, description
   ```
2. Merge to integration
3. W2 creates atomic PR
4. W5 runs: ArtifactHub lint or ct lint FAIL
5. PR cannot merge

### Expected

- artifacthub-lint or helm-lint jobs fail
- Required status checks block merge
- No release is created

### Cleanup

- Fix chart or close PR
- Delete test branches

---

## E2E-8: Version Bump Failure Blocks Release

**Objective:** Verify that if version bump fails, the release has appropriate handling.

### Steps

1. Create chart change with invalid conventional commit
   ```
   "update something"  # Not conventional commit format
   ```
2. Merge to integration
3. W2 creates atomic PR
4. W5 runs: version-bump determines no bump needed (non-conventional)
5. If Chart.yaml version already exists as tag...

### Expected

- Version bump job logs "no version bump needed" or determines bump type
- If tag already exists at different commit → Release workflow errors
- Release workflow checks for tag collision before creating

### Verification

```bash
# Check if tag exists at different commit
TAG="test-workflow-v0.1.0"
EXISTING=$(git rev-list -n 1 "$TAG" 2>/dev/null || echo "none")
CURRENT="${{ github.sha }}"
if [[ "$EXISTING" != "none" && "$EXISTING" != "$CURRENT" ]]; then
  echo "ERROR: Tag exists at different commit!"
fi
```

### Cleanup

- Close PR if test-only
- Delete test branches

---

## E2E-9: Attestation Verification Failure Detection

**Objective:** Verify that consumers can detect attestation verification failures.

### Steps

1. Complete E2E-1 to create a valid release
2. Download the released package
3. Modify the package (simulate tampering)
4. Attempt to verify the tampered package

### Expected

- `gh attestation verify` fails on tampered package
- `cosign verify` fails on modified OCI image
- Error message clearly indicates verification failure

### Verification Commands

```bash
# Download valid package
gh release download test-workflow-v0.2.0 \
  --repo aRustyDev/helm-charts \
  --pattern "test-workflow-0.2.0.tgz"

# Verify original (should pass)
gh attestation verify test-workflow-0.2.0.tgz --repo aRustyDev/helm-charts
echo "Original verification: PASSED"

# Tamper with package
cp test-workflow-0.2.0.tgz test-workflow-0.2.0-tampered.tgz
echo "malicious" >> test-workflow-0.2.0-tampered.tgz

# Verify tampered (should fail)
if gh attestation verify test-workflow-0.2.0-tampered.tgz \
    --repo aRustyDev/helm-charts 2>&1; then
  echo "ERROR: Tampered package verified (unexpected)"
  exit 1
else
  echo "Tampered verification: CORRECTLY FAILED"
fi
```

### Cleanup

- Remove downloaded test files

---

## E2E-10: Full Lineage Trace Verification

**Objective:** Verify complete lineage can be traced from release to original contributor.

### Steps

1. Complete E2E-1 with a trusted contributor
2. After release, perform full lineage trace
3. Verify each link in the chain

### Verification Script

```bash
#!/usr/bin/env bash
set -euo pipefail

CHART="test-workflow"
VERSION="0.2.0"
TAG="${CHART}-v${VERSION}"
REPO="aRustyDev/helm-charts"

echo "=== Full Lineage Trace for $TAG ==="

# Step 1: Get release commit
echo "1. Getting release commit..."
RELEASE_SHA=$(gh api "/repos/${REPO}/git/ref/tags/${TAG}" --jq '.object.sha')
echo "   Release SHA: $RELEASE_SHA"

# Step 2: Find merge PR
echo "2. Finding merge PR..."
MERGE_PR=$(gh api "/repos/${REPO}/commits/${RELEASE_SHA}/pulls" --jq '.[0].number')
echo "   Merge PR: #$MERGE_PR"

# Step 3: Get PR source branch
echo "3. Getting PR source branch..."
SOURCE_BRANCH=$(gh pr view "$MERGE_PR" --repo "$REPO" --json headRefName -q '.headRefName')
echo "   Source branch: $SOURCE_BRANCH"

# Step 4: Extract attestation map
echo "4. Extracting attestation map..."
ATTESTATION=$(gh pr view "$MERGE_PR" --repo "$REPO" --json body -q '.body' | \
  grep -A5 "ATTESTATION_MAP" || echo "Not found")
if [[ "$ATTESTATION" != "Not found" ]]; then
  echo "   Attestation map: PRESENT"
else
  echo "   Attestation map: MISSING (gap in lineage)"
fi

# Step 5: Verify Cosign signature
echo "5. Verifying Cosign signature..."
if cosign verify "ghcr.io/arustydev/helm-charts/${CHART}:${VERSION}" \
    --certificate-oidc-issuer https://token.actions.githubusercontent.com \
    --certificate-identity-regexp "github.com/aRustyDev/helm-charts" \
    --output text 2>/dev/null; then
  echo "   Cosign: VALID"
else
  echo "   Cosign: INVALID"
fi

echo ""
echo "=== Lineage Trace Complete ==="
```

### Expected

- All 5 steps complete successfully
- Each link in chain verifiable
- Attestation map present in merge PR
- Cosign signature valid

### Cleanup

- None (uses existing release from E2E-1)

---

## Summary Matrix

| Test | Focus | Critical Path | Estimated Time |
|------|-------|---------------|----------------|
| E2E-1 | Happy path | Yes | 15-20 min |
| E2E-2 | Untrusted contributor | Yes | 10-15 min |
| E2E-3 | Unsigned commits | Yes | 10-15 min |
| E2E-4 | Multiple charts | No | 20-30 min |
| E2E-5 | Missing attestation | No | 10-15 min |
| E2E-6 | K8s test failure | Yes | 15-20 min |
| E2E-7 | Lint failure | Yes | 10-15 min |
| E2E-8 | Version bump failure | No | 10-15 min |
| E2E-9 | Tamper detection | Yes | 5-10 min |
| E2E-10 | Lineage trace | Yes | 5-10 min |

**Recommended execution order:** E2E-1 first (creates artifacts for E2E-9, E2E-10), then E2E-6, E2E-7 (critical blockers), then remaining tests.
