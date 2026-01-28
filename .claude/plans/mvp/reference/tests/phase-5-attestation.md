# Phase 5: Attestation and Lineage Tests

**Purpose:** Validate attestation generation, verification, and lineage tracing

---

## Attestation Controls

| ID | Control | Implementation | Notes |
|----|---------|----------------|-------|
| AT-C1 | Package attestation generated | `attest-build-provenance` action | GitHub Sigstore |
| AT-C2 | Attestation attached to artifact | `--subject-path` pointing to package | Links attestation to .tgz |
| AT-C3 | Cosign signature on OCI | `cosign sign` with OIDC | Keyless signing |
| AT-C4 | Attestation verifiable | `gh attestation verify` succeeds | Consumer verification |
| AT-C5 | Cosign signature verifiable | `cosign verify` succeeds | OCI verification |
| AT-C6 | Attestation includes build info | Contains workflow, commit SHA, repository | Provenance data |

---

## Attestation Test Matrix

| Test ID | Control | Scenario | Expected | Verification |
|---------|---------|----------|----------|--------------|
| AT-T1 | AT-C1 | Release workflow completes | Attestation step succeeds | Check workflow logs |
| AT-T2 | AT-C2 | Package has attestation | Attestation linked to .tgz artifact | `gh attestation verify <package>` |
| AT-T3 | AT-C3 | OCI image has Cosign signature | Signature exists in registry | `cosign tree ghcr.io/.../chart` |
| AT-T4 | AT-C4 | Verify package attestation | Returns valid attestation JSON | `gh attestation verify --format json` |
| AT-T5 | AT-C5 | Verify Cosign signature | Verification succeeds with OIDC issuer | `cosign verify --certificate-oidc-issuer` |
| AT-T6 | AT-C6 | Attestation contains build metadata | Includes repo, workflow, SHA, actor | Parse attestation JSON |
| AT-T7 | AT-C4 | Tampered package fails verification | Attestation verify fails | Modify .tgz, run verify |
| AT-T8 | AT-C5 | Wrong issuer fails verification | Cosign verify fails | Use wrong `--certificate-oidc-issuer` |

---

## Lineage Controls

| ID | Control | Description |
|----|---------|-------------|
| AL-C1 | Release → Merge Commit | Release attestation references merge commit SHA |
| AL-C2 | Merge Commit → Atomic PR | Merge commit matches PR merge SHA |
| AL-C3 | Atomic PR → Integration | PR source branch created from integration commit |
| AL-C4 | Integration → Contributor | Integration commit from merged contribution PR |
| AL-C5 | Full Chain | Can trace release back to original contributor PR |

---

## Lineage Test Matrix

| Test ID | Scenario | Expected | Verification |
|---------|----------|----------|--------------|
| AL-T1 | Trace release to merge commit | Release attestation references merge commit SHA | Compare attestation SHA with `git log main` |
| AL-T2 | Trace merge to atomic PR | Merge commit matches PR merge SHA | `gh pr view --json mergeCommit` |
| AL-T3 | Trace atomic PR to integration | PR source branch created from integration commit | `git log charts/<chart>..integration` |
| AL-T4 | Trace integration to contributor | Integration commit from merged contribution PR | `git log integration --oneline` |
| AL-T5 | Full lineage audit | Can trace release back to original contributor PR | Chain: Release → main → PR → charts/* → integration |
| AL-T6 | Attestation actor matches workflow | `github-actions[bot]` or workflow actor in claims | Parse attestation `predicate.invocation.actor` |

---

## Test Procedures

### AT-T2: Verify Package Attestation

**Prerequisite:** Complete release exists (from E2E-1 or Phase 4 tests)

**Verification:**
```bash
# Download package from release
gh release download test-workflow-v0.2.0 \
  --repo aRustyDev/helm-charts \
  --pattern "test-workflow-0.2.0.tgz"

# Verify attestation
gh attestation verify test-workflow-0.2.0.tgz \
  --repo aRustyDev/helm-charts

# Expected output:
# ✓ Verification succeeded!
# ...attestation details...
```

---

### AT-T4: Detailed Attestation Verification

```bash
# Verify with JSON output
gh attestation verify test-workflow-0.2.0.tgz \
  --repo aRustyDev/helm-charts \
  --format json | jq '.attestations[].verificationResult'

# Expected: "VERIFIED"
```

---

### AT-T5: Cosign Signature Verification

```bash
# Verify Cosign signature
cosign verify ghcr.io/arustydev/helm-charts/test-workflow:0.2.0 \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com \
  --certificate-identity-regexp "github.com/aRustyDev/helm-charts"

# View signature tree
cosign tree ghcr.io/arustydev/helm-charts/test-workflow:0.2.0
```

---

### AT-T6: Verify Build Metadata in Attestation

```bash
# Extract attestation predicate
gh attestation verify test-workflow-0.2.0.tgz \
  --repo aRustyDev/helm-charts \
  --format json | \
  jq '.attestations[0].bundle.dsseEnvelope.payload' | \
  base64 -d | jq '.predicate'

# Expected fields:
# - buildDefinition.resolvedDependencies (source repo)
# - runDetails.builder.id (GitHub Actions)
# - runDetails.metadata.invocationId (workflow run)
```

---

### AT-T7: Tampered Package Fails Verification

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

# Cleanup
rm -f test-workflow-0.2.0.tgz test-workflow-0.2.0-tampered.tgz
```

---

### AL-T5: Full Lineage Trace

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

# Step 4: Extract attestation map from PR
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

---

## Verification Commands Reference

```bash
# Verify GitHub attestation on package
gh attestation verify charts/test-workflow-0.2.0.tgz \
  --repo aRustyDev/helm-charts

# Verify with JSON output for detailed inspection
gh attestation verify charts/test-workflow-0.2.0.tgz \
  --repo aRustyDev/helm-charts \
  --format json | jq '.attestations[].verificationResult'

# Verify Cosign signature on OCI image
cosign verify ghcr.io/arustydev/helm-charts/test-workflow:0.2.0 \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com \
  --certificate-identity-regexp "github.com/aRustyDev/helm-charts"

# View Cosign signature tree
cosign tree ghcr.io/arustydev/helm-charts/test-workflow:0.2.0

# Extract attestation predicate for lineage verification
gh attestation verify charts/test-workflow-0.2.0.tgz \
  --repo aRustyDev/helm-charts \
  --format json | jq '.attestations[0].bundle.dsseEnvelope.payload' | \
  base64 -d | jq '.predicate'
```

---

## Gaps and Considerations

| Gap ID | Description | Severity | Status |
|--------|-------------|----------|--------|
| AT-G1 | Attestation map may be missing if PR created outside W2 | Medium | Document limitation |
| AT-G2 | Cosign requires OIDC issuer knowledge | Low | Document in verification guide |

---

## Related Documentation

- [GitHub Attestations](https://docs.github.com/en/actions/security-guides/using-artifact-attestations-to-establish-provenance-for-builds)
- [Cosign Keyless Signing](https://docs.sigstore.dev/signing/quickstart/)
- [SLSA Provenance](https://slsa.dev/provenance/)
