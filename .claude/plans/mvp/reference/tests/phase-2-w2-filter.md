# Phase 2: W2 Filter Charts Tests

**Workflow:** `create-atomic-chart-pr.yaml`
**Purpose:** Validate chart filtering and atomic branch/PR creation

---

## Controls

| ID | Control | Implementation | Notes |
|----|---------|----------------|-------|
| W2-C1 | Trigger on integration push | `push.branches: [integration]` | Workflow trigger |
| W2-C2 | Only process charts/** changes | `paths: ['charts/**']` | Path filter |
| W2-C3 | Chart must have Chart.yaml | `detect-changes` action with `manifest-file: Chart.yaml` | **Updated**: Uses action |
| W2-C4 | Concurrency control | `group: w2-filter-charts` | Prevents race conditions |
| W2-C5 | Create charts/<chart> branch | `arustydev/gha/actions/atomic-branch/create-branch@main` | **Updated**: Uses action |
| W2-C6 | Create PR to main | `arustydev/gha/actions/atomic-branch/create-pr@main` | **Updated**: Uses action |

---

## Test Matrix

| Test ID | Control | Scenario | Expected | Cleanup |
|---------|---------|----------|----------|---------|
| W2-T1 | W2-C1 | Push to integration (not charts/) | Workflow doesn't run | N/A |
| W2-T2 | W2-C2 | Push to main (not integration) | Workflow doesn't run | N/A |
| W2-T3 | W2-C3 | Change file in charts/ but not a chart | Skipped with notice | N/A |
| W2-T4 | W2-C4 | Concurrent pushes to integration | Second waits or cancels | N/A |
| W2-T5 | W2-C5 | Single chart change | Creates `charts/<chart>` branch | Delete branch |
| W2-T6 | W2-C5 | Multiple chart changes | Creates multiple branches | Delete branches |
| W2-T7 | W2-C6 | PR already exists for branch | Updates existing PR | N/A |

---

## Test Procedures

### W2-T1: Non-Chart Push Ignored

**Setup:**
```bash
# Merge a non-chart change to integration
git checkout integration
echo "# Test" >> README.md
git add . && git commit -S -m "docs: update readme"
git push origin integration
```

**Verify:**
- W2 workflow does NOT trigger (path filter excludes non-charts/)
- Check Actions tab - no W2 run

**Cleanup:** None needed

---

### W2-T3: Non-Chart Directory Skipped

**Setup:**
```bash
# Create a file in charts/ but not in a chart directory
git checkout integration
mkdir -p charts/.hidden
echo "test" > charts/.hidden/test.txt
git add . && git commit -S -m "test: non-chart file"
git push origin integration
```

**Verify:**
- W2 triggers (path filter matches charts/)
- detect-changes action finds no charts (no Chart.yaml)
- `charts_count == '0'`
- No branches or PRs created

**Cleanup:** None needed

---

### W2-T5: Single Chart Creates Branch/PR

**Setup:**
```bash
# Make a change to test-workflow chart and merge to integration
git checkout -b test/w2-t5 integration
echo "# W2-T5 test" >> charts/test-workflow/README.md
git add . && git commit -S -m "test(test-workflow): w2-t5 single chart"
git push origin test/w2-t5
gh pr create --base integration --title "test(w2-t5): single chart change"
# Wait for W1 + auto-merge, or merge manually
```

**Verify:**
- W2 triggers after merge to integration
- detect-changes outputs: `charts=test-workflow`, `count=1`
- create-branch action outputs: `branch=charts/test-workflow`, `updated=true`
- create-pr action outputs: `pr-number=<N>`, `action=created`
- PR created to main with attestation map in body

**Verification Commands:**
```bash
# Check branch exists
git ls-remote --heads origin charts/test-workflow

# Check PR exists
gh pr list --head charts/test-workflow --base main

# Check PR body has attestation map
gh pr view <N> --json body | jq -r '.body' | grep "ATTESTATION_MAP"
```

**Cleanup:**
```bash
git push origin --delete charts/test-workflow
gh pr close <N>
```

---

### W2-T6: Multiple Charts Create Multiple Branches

**Setup:**
```bash
git checkout -b test/w2-t6 integration
echo "# W2-T6" >> charts/test-workflow/README.md
echo "# W2-T6" >> charts/cloudflared/README.md
git add . && git commit -S -m "test: w2-t6 multiple charts"
git push origin test/w2-t6
gh pr create --base integration --title "test(w2-t6): multiple charts"
# Merge to integration
```

**Verify:**
- W2 triggers
- detect-changes outputs: `charts=test-workflow cloudflared`, `count=2`
- Matrix creates two jobs
- Two branches: `charts/test-workflow`, `charts/cloudflared`
- Two PRs to main

**Cleanup:**
```bash
git push origin --delete charts/test-workflow charts/cloudflared
# Close both PRs
```

---

### W2-T7: Existing PR Updated

**Setup:**
```bash
# First, run W2-T5 to create initial PR
# Then make another change to same chart
git checkout -b test/w2-t7 integration
echo "# W2-T7 update" >> charts/test-workflow/README.md
git add . && git commit -S -m "test(test-workflow): w2-t7 update"
git push origin test/w2-t7
gh pr create --base integration --title "test(w2-t7): update existing"
# Merge to integration
```

**Verify:**
- W2 triggers
- create-pr action outputs: `action=updated` (not `created`)
- Same PR number as W2-T5
- PR body updated with new attestation map

**Cleanup:**
```bash
git push origin --delete charts/test-workflow
gh pr close <N>
```

---

## Action Output Reference

### detect-changes action
```yaml
outputs:
  artifacts: 'test-workflow cloudflared'  # Space-separated
  artifacts-json: '["test-workflow","cloudflared"]'  # JSON array
  count: '2'
```

### create-branch action
```yaml
outputs:
  branch: 'charts/test-workflow'
  updated: 'true' | 'false'
  sha: '<commit SHA>'
```

### create-pr action
```yaml
outputs:
  pr-number: '123'
  action: 'created' | 'updated'
  url: 'https://github.com/.../pull/123'
```

---

## Dispatch Trigger

W2 triggers W5 via repository dispatch:

```yaml
- uses: arustydev/gha/actions/dispatch/trigger@main
  with:
    event-type: chart-pr-created
    client-payload: |
      {
        "pr": ${{ steps.pr.outputs.pr-number }},
        "chart": "${{ matrix.chart }}"
      }
```

**Verify W5 Trigger:**
```bash
# Check W5 workflow runs with repository_dispatch trigger
gh run list --workflow=validate-atomic-chart-pr.yaml --event=repository_dispatch
```

---

## Gaps and Considerations

| Gap ID | Description | Severity | Status |
|--------|-------------|----------|--------|
| W2-G1 | Dependencies branch handled separately | Info | By design |
| W2-G2 | Dispatch may not trigger W5 with GITHUB_TOKEN | Medium | Uses dispatch action |

---

## Related Documentation

- [W2 Workflow](../../../../.github/workflows/create-atomic-chart-pr.yaml)
- [Create Branch Action](https://github.com/arustydev/gha/tree/main/actions/atomic-branch/create-branch)
- [Create PR Action](https://github.com/arustydev/gha/tree/main/actions/atomic-branch/create-pr)
- [Dispatch Trigger Action](https://github.com/arustydev/gha/tree/main/actions/dispatch/trigger)
