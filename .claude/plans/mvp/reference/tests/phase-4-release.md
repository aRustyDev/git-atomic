# Phase 4: Release Workflow Tests

**Workflow:** `tag-atomic-chart.yaml` + `release-atomic-chart.yaml`
**Purpose:** Validate tagging, packaging, and publishing of charts

---

## Controls

| ID | Control | Implementation | Notes |
|----|---------|----------------|-------|
| R-C1 | Trigger on main push | `push.branches: [main]` | Tag workflow trigger |
| R-C2 | Only process charts/** | `paths: ['charts/**']` | Path filter |
| R-C3 | Tag doesn't already exist | `git rev-parse "$TAG_NAME"` check | Prevent duplicates |
| R-C4 | Tag points to correct commit | Compare existing tag SHA | Idempotency check |
| R-C5 | Chart.yaml has version | `extract-version` action | **Updated**: Uses action |
| R-C6 | Package attestation | `attest-build-provenance` | GitHub attestation |
| R-C7 | GHCR push | `helm push` + Cosign sign | OCI registry |
| R-C8 | GitHub Release created | `gh release create` | Release with assets |
| R-C9 | Release branch updated | Push to `release` branch | Helm repo index |

---

## Test Matrix

| Test ID | Control | Scenario | Expected | Cleanup |
|---------|---------|----------|----------|---------|
| R-T1 | R-C1 | Push to integration (not main) | Workflow doesn't run | N/A |
| R-T2 | R-C2 | Push to main (non-chart files) | Workflow doesn't run | N/A |
| R-T3 | R-C3 | Tag already exists (same commit) | Skip with notice | N/A |
| R-T4 | R-C4 | Tag exists at different commit | Error - version not bumped | Investigate |
| R-T5 | R-C5 | Chart.yaml missing version | Error extracting version | Fix Chart.yaml |
| R-T6 | R-C6 | Package created | Attestation generated | Verify attestation |
| R-T7 | R-C7 | GHCR push | Chart in registry + signed | Verify with cosign |
| R-T8 | R-C8 | Release created | GitHub Release exists | Verify assets |
| R-T9 | R-C9 | Release branch updated | index.yaml updated | Verify content |

---

## Test Procedures

### R-T3: Duplicate Tag Handling (Same Commit)

**Setup:**
```bash
# After a release, push the same chart without version bump
# (This simulates re-running the workflow)
git checkout main
# Make non-version change and push
```

**Verify:**
- Tag workflow triggers
- Tag already exists at same commit
- Workflow logs: "Tag already exists at correct commit"
- No error, skips release

---

### R-T4: Tag at Different Commit (Version Not Bumped)

**Setup:**
```bash
# Manually create tag at wrong commit (simulates error)
git tag test-workflow-v0.1.0 HEAD~1
git push origin test-workflow-v0.1.0

# Then push chart change without version bump
git checkout main
echo "# test" >> charts/test-workflow/README.md
git add . && git commit -S -m "docs: update without bump"
git push origin main
```

**Verify:**
- Tag workflow triggers
- Detects version 0.1.0 in Chart.yaml
- Tag exists at different commit
- Error: "Version 0.1.0 already tagged at different commit"
- Workflow fails (prevents release of unbumped version)

**Cleanup:**
```bash
git push origin --delete test-workflow-v0.1.0
git tag -d test-workflow-v0.1.0
```

---

### R-T6: Attestation Generation

**Setup:**
```bash
# Complete a full release (merge bumped PR to main)
```

**Verify:**
- Release workflow runs
- `attest-build-provenance` step succeeds
- Attestation ID logged

**Verification:**
```bash
# Verify attestation exists
gh attestation verify charts/test-workflow-0.2.0.tgz \
  --repo aRustyDev/helm-charts

# Check attestation contents
gh attestation verify charts/test-workflow-0.2.0.tgz \
  --repo aRustyDev/helm-charts \
  --format json | jq '.attestations[0]'
```

---

### R-T7: GHCR Push and Cosign

**Setup:**
```bash
# Complete a full release
```

**Verify:**
- Chart pushed to GHCR
- Cosign signature attached

**Verification:**
```bash
# Check chart exists in GHCR
helm show chart oci://ghcr.io/arustydev/helm-charts/test-workflow --version 0.2.0

# Verify Cosign signature
cosign verify ghcr.io/arustydev/helm-charts/test-workflow:0.2.0 \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com \
  --certificate-identity-regexp "github.com/aRustyDev/helm-charts"

# View signature tree
cosign tree ghcr.io/arustydev/helm-charts/test-workflow:0.2.0
```

---

### R-T8: GitHub Release Creation

**Setup:**
```bash
# Complete a full release
```

**Verify:**
- GitHub Release created
- Tag matches chart version
- Assets attached

**Verification:**
```bash
# List releases
gh release list --repo aRustyDev/helm-charts

# View release details
gh release view test-workflow-v0.2.0 --repo aRustyDev/helm-charts

# List assets
gh release view test-workflow-v0.2.0 --repo aRustyDev/helm-charts --json assets
```

**Expected Assets:**
- `test-workflow-0.2.0.tgz` - Chart package
- `CHANGELOG.md` - Chart changelog
- Attestation reference

---

### R-T9: Release Branch Update

**Setup:**
```bash
# Complete a full release
```

**Verify:**
- `release` branch updated
- `index.yaml` includes new chart version

**Verification:**
```bash
# Check release branch
git fetch origin release
git log origin/release --oneline -5

# Verify index.yaml
git show origin/release:index.yaml | grep "test-workflow"
```

---

## Action Output Reference

### extract-version action
```yaml
inputs:
  file: 'charts/test-workflow/Chart.yaml'
  key: 'version'
outputs:
  version: '0.2.0'
```

---

## Gaps and Considerations

| Gap ID | Description | Severity | Status |
|--------|-------------|----------|--------|
| R-G1 | Release workflow may need manual trigger on failure | Low | Use workflow_dispatch |
| R-G2 | GHCR permissions require setup | Info | Documented in prerequisites |

---

## Related Documentation

- [Tag Workflow](../../../../.github/workflows/tag-atomic-chart.yaml)
- [Release Workflow](../../../../.github/workflows/release-atomic-chart.yaml)
- [Extract Version Action](https://github.com/arustydev/gha/tree/main/actions/version-bump/extract-version)
