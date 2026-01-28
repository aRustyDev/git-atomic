# Phase 1: Auto-Merge Workflow Tests

**Workflow:** `auto-merge-integration.yaml`
**Purpose:** Validate trust controls before enabling auto-merge on PRs to integration

---

## Controls

| ID | Control | Implementation | Notes |
|----|---------|----------------|-------|
| AM-C1 | W1 must succeed | `workflow_run.conclusion == 'success'` | Workflow trigger condition |
| AM-C2 | Must be PR event | `workflow_run.event == 'pull_request'` | Skip non-PR triggers |
| AM-C3 | PR targets allowed branch | `ALLOWED_BASE_BRANCHES` variable check | Configurable via repo variable |
| AM-C4 | PR must be open | `--state open` filter in gh pr list | Skip closed/merged PRs |
| AM-C5 | Author in CODEOWNERS | `arustydev/gha/actions/trust-check/codeowners@main` | **Updated**: Uses action instead of inline grep |
| AM-C6 | All commits signed | `arustydev/gha/actions/trust-check/verify-signatures@main` | **Updated**: Uses action with git verification |

---

## Test Matrix

| Test ID | Control | Scenario | Expected | Cleanup |
|---------|---------|----------|----------|---------|
| AM-T1 | AM-C1 | W1 fails (invalid Chart.yaml) | Workflow doesn't trigger | Delete branch |
| AM-T2 | AM-C2 | W1 via workflow_dispatch | Job skips (`event != 'pull_request'`) | N/A |
| AM-T3 | AM-C3 | PR targets `main` | "branch_not_allowed" warning | Delete branch, close PR |
| AM-T4 | AM-C4 | Close PR before workflow runs | "No open PR found" | Delete branch |
| AM-T5 | AM-C5 | Author NOT in CODEOWNERS | Trust check fails, `is-owner=false` | Delete branch, close PR |
| AM-T6 | AM-C5 | Author IN CODEOWNERS | Trust check passes, `is-owner=true` | Continue to AM-T7 |
| AM-T7 | AM-C6 | Unsigned commits | Verification fails, `all-signed=false` | Delete branch, close PR |
| AM-T8 | AM-C6 | All commits signed | Verification passes, `all-signed=true` | Continue to merge |
| AM-T9 | ALL | Trusted + Verified | Auto-merge ENABLED | Merge completes |
| AM-T10 | ALL | Untrusted + Verified | Auto-merge NOT enabled | Manual merge required |

---

## Test Procedures

### AM-T1: W1 Failure Prevents Trigger

**Setup:**
```bash
git checkout -b test/am-t1 integration
# Create invalid Chart.yaml
echo "invalid: yaml: content" > charts/test-workflow/Chart.yaml
git add . && git commit -S -m "test(am-t1): invalid chart"
git push origin test/am-t1
gh pr create --base integration --title "test(am-t1): W1 failure test"
```

**Verify:**
- W1 workflow fails
- Auto-merge workflow does NOT trigger (check Actions tab)

**Cleanup:**
```bash
gh pr close --delete-branch
```

---

### AM-T5: Untrusted Author Blocked

**Setup:** (Run as user NOT in CODEOWNERS)
```bash
git checkout -b test/am-t5 integration
echo "test: value" >> charts/test-workflow/values.yaml
git add . && git commit -S -m "test(am-t5): untrusted author"
git push origin test/am-t5
gh pr create --base integration --title "test(am-t5): untrusted author"
```

**Verify:**
- W1 passes
- Auto-merge triggers but trust check shows `is-owner=false`
- Auto-merge NOT enabled on PR

**Cleanup:**
```bash
gh pr close --delete-branch
```

---

### AM-T7: Unsigned Commits Blocked

**Setup:**
```bash
git checkout -b test/am-t7 integration
echo "test: value" >> charts/test-workflow/values.yaml
git add . && git commit --no-gpg-sign -m "test(am-t7): unsigned commit"
git push origin test/am-t7
gh pr create --base integration --title "test(am-t7): unsigned commits"
```

**Verify:**
- W1 passes
- Auto-merge triggers
- Trust check passes (if in CODEOWNERS)
- Signature verification fails, `all-signed=false`
- Auto-merge NOT enabled

**Cleanup:**
```bash
gh pr close --delete-branch
```

---

### AM-T9: Happy Path (Trusted + Verified)

**Setup:** (Run as user IN CODEOWNERS with GPG key)
```bash
git checkout -b test/am-t9 integration
echo "# AM-T9 test" >> charts/test-workflow/README.md
git add . && git commit -S -m "test(am-t9): happy path"
git push origin test/am-t9
gh pr create --base integration --title "test(am-t9): happy path"
```

**Verify:**
- W1 passes
- Auto-merge triggers
- Trust check: `is-owner=true`
- Signature verification: `all-verified=true`
- Auto-merge ENABLED on PR
- PR merges automatically when checks pass

**Cleanup:**
- Branch deleted automatically on merge

---

## Action Output Reference

### codeowners action
```yaml
outputs:
  is-owner: 'true' | 'false'
  matched-pattern: '<pattern from CODEOWNERS>'
```

### verify-signatures action
```yaml
outputs:
  all-signed: 'true' | 'false'
  verified-count: '<number>'
  total-count: '<number>'
  unverified-commits: '<comma-separated SHAs>'
```

> **Note:** The workflow uses `all-signed` (not `all-verified`) as the output field name.

---

## Gaps and Considerations

| Gap ID | Description | Severity | Status |
|--------|-------------|----------|--------|
| AM-G1 | CODEOWNERS action requires pre-find step | Low | Resolved in workflow |
| AM-G2 | Bot detection separate from CODEOWNERS | Info | By design |

---

## Related Documentation

- [Auto-Merge Workflow](../../../../.github/workflows/auto-merge-integration.yaml)
- [CODEOWNERS Action](https://github.com/arustydev/gha/tree/main/actions/trust-check/codeowners)
- [Verify Signatures Action](https://github.com/arustydev/gha/tree/main/actions/trust-check/verify-signatures)
