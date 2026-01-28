# Phase 3: W5 Validate Atomic Chart PR Tests

**Workflow:** `validate-atomic-chart-pr.yaml`
**Purpose:** Validate chart quality, run tests, and bump versions

---

## Controls

| ID | Control | Implementation | Notes |
|----|---------|----------------|-------|
| W5-C1 | PR targets main | `pull_request.branches: [main]` | Workflow trigger |
| W5-C2 | Dispatch actor validation | `ALLOWED_DISPATCH_ACTORS` check | For repository_dispatch |
| W5-C3 | Source branch pattern | `arustydev/gha/actions/trust-check/validate-source-branch@main` | **Updated**: Uses action with regex |
| W5-C4 | Chart must exist | `has_charts == 'true'` from detect-changes | Skip if no charts |
| W5-C5 | ArtifactHub lint pass | `ah lint --kind helm` | ArtifactHub metadata validation |
| W5-C6 | Helm lint pass | `ct lint` | Chart-testing lint |
| W5-C7 | K8s matrix tests pass | `ct install` on v1.32, v1.33, v1.34 | Install tests |
| W5-C8 | Version bump logic | `version-bump/*` + `changelog/*` actions | **Updated**: Uses action chain |
| W5-C9 | Cleanup on merge | Delete source branch | Post-merge cleanup |

---

## Test Matrix

| Test ID | Control | Scenario | Expected | Cleanup |
|---------|---------|----------|----------|---------|
| W5-T1 | W5-C1 | PR targets integration | Workflow doesn't run | Close PR |
| W5-T2 | W5-C2 | Dispatch from unauthorized actor | "Unauthorized actor" error | N/A |
| W5-T3 | W5-C2 | Dispatch from github-actions[bot] | Actor validated | Continue |
| W5-T4 | W5-C3 | PR from `feature/` branch | `valid=false`, warning logged | Close PR |
| W5-T5 | W5-C3 | PR from `charts/test` branch | `valid=true` | Continue |
| W5-T6 | W5-C4 | PR with no chart changes | Skip validation jobs | Close PR |
| W5-T7 | W5-C5 | Chart missing ArtifactHub metadata | Lint fails | Fix and retry |
| W5-T8 | W5-C6 | Chart with Helm lint errors | ct lint fails | Fix and retry |
| W5-T9 | W5-C7 | Chart install fails on K8s 1.32 | Matrix job fails | Fix and retry |
| W5-T10 | W5-C8 | `fix(chart):` commit | Patch version bump | Verify Chart.yaml |
| W5-T11 | W5-C8 | `feat(chart):` commit | Minor version bump | Verify Chart.yaml |
| W5-T12 | W5-C8 | `feat(chart)!:` commit | Major version bump | Verify Chart.yaml |
| W5-T13 | W5-C9 | Merge PR to main | Source branch deleted | Verify deletion |

---

## Test Procedures

### W5-T4: Invalid Branch Pattern Rejected

**Setup:**
```bash
# Create PR from non-standard branch
git checkout -b feature/test-w5-t4 main
echo "# Test" >> charts/test-workflow/README.md
git add . && git commit -S -m "test: invalid branch"
git push origin feature/test-w5-t4
gh pr create --base main --title "test(w5-t4): invalid branch pattern"
```

**Verify:**
- W5 triggers
- validate-source-branch action: `valid=false`
- Warning logged: "does not match expected pattern"
- Version bump skipped (source_valid check fails)

**Cleanup:**
```bash
gh pr close --delete-branch
```

---

### W5-T5: Valid Branch Accepted

**Setup:**
```bash
# This is typically created by W2, but can be simulated
git checkout -b charts/test-workflow main
echo "# Test" >> charts/test-workflow/README.md
git add . && git commit -S -m "test(test-workflow): valid branch"
git push origin charts/test-workflow
gh pr create --base main --title "test(w5-t5): valid branch pattern"
```

**Verify:**
- W5 triggers
- validate-source-branch action: `valid=true`
- Validation jobs proceed

**Cleanup:**
```bash
gh pr close --delete-branch
```

---

### W5-T10: Patch Version Bump

**Setup:**
```bash
# Ensure chart version is known (e.g., 0.1.0)
git checkout -b charts/test-workflow main
# Fix commit triggers patch bump
echo "# Bug fix" >> charts/test-workflow/README.md
git add . && git commit -S -m "fix(test-workflow): resolve issue"
git push origin charts/test-workflow
gh pr create --base main --title "fix(test-workflow): patch bump test"
```

**Verify:**
- determine-bump action: `bump-type=patch`
- calculate-version action: `0.1.0 → 0.1.1`
- update-manifest action: Chart.yaml updated
- Commit pushed to PR branch

**Verification:**
```bash
# Check Chart.yaml version
gh pr diff <N> -- charts/test-workflow/Chart.yaml | grep "^+version:"
# Should show: +version: 0.1.1
```

**Cleanup:**
```bash
gh pr close --delete-branch
```

---

### W5-T11: Minor Version Bump

**Setup:**
```bash
git checkout -b charts/test-workflow main
# Feat commit triggers minor bump
cat >> charts/test-workflow/values.yaml << 'EOF'
# New feature
newFeature:
  enabled: false
EOF
git add . && git commit -S -m "feat(test-workflow): add new feature"
git push origin charts/test-workflow
gh pr create --base main --title "feat(test-workflow): minor bump test"
```

**Verify:**
- determine-bump action: `bump-type=minor`
- calculate-version action: `0.1.0 → 0.2.0`
- Chart.yaml and CHANGELOG.md updated

**Cleanup:**
```bash
gh pr close --delete-branch
```

---

### W5-T12: Major Version Bump

**Setup:**
```bash
git checkout -b charts/test-workflow main
# Breaking change triggers major bump
echo "# BREAKING: API changed" >> charts/test-workflow/README.md
git add . && git commit -S -m "feat(test-workflow)!: breaking API change"
git push origin charts/test-workflow
gh pr create --base main --title "feat(test-workflow)!: major bump test"
```

**Verify:**
- determine-bump action: `bump-type=major`
- calculate-version action: `0.1.0 → 1.0.0`

**Cleanup:**
```bash
gh pr close --delete-branch
```

---

### W5-T13: Branch Cleanup on Merge

**Setup:**
```bash
# Complete a full W5 run and merge the PR
# (Use a valid PR from previous tests)
gh pr merge <N> --squash
```

**Verify:**
- Post-merge job triggers
- Source branch `charts/test-workflow` deleted

**Verification:**
```bash
git ls-remote --heads origin charts/test-workflow
# Should return empty
```

---

## Action Output Reference

### validate-source-branch action
```yaml
inputs:
  source-branch: 'charts/test-workflow'
  allowed-pattern: '^charts/[a-z0-9-]+$,^integration/[a-z0-9-]+$'
  use-regex: true
  fail-on-invalid: false
outputs:
  valid: 'true' | 'false'
  matched-pattern: '<pattern that matched>'
```

### determine-bump action
```yaml
inputs:
  artifact: 'test-workflow'
  artifact-path: 'charts'
  base-ref: 'origin/main'
outputs:
  bump-type: 'major' | 'minor' | 'patch'
```

### calculate-version action
```yaml
inputs:
  current-version: '0.1.0'
  bump-type: 'minor'
outputs:
  next-version: '0.2.0'
```

### update-manifest action
```yaml
inputs:
  file: 'charts/test-workflow/Chart.yaml'
  version-key: 'version'
  new-version: '0.2.0'
```

### changelog/generate action
```yaml
inputs:
  artifact: 'test-workflow'
  artifact-path: 'charts'
  version: '0.2.0'
  base-ref: 'origin/main'
  config: 'cliff.toml'
outputs:
  changelog: '<markdown content>'
```

---

## Gaps and Considerations

| Gap ID | Description | Severity | Status |
|--------|-------------|----------|--------|
| W5-G1 | Multi-chart PRs use first chart only | Medium | Atomic PRs typically have one chart |
| W5-G2 | git-cliff installation adds time | Low | Only runs when bump needed |

---

## Related Documentation

- [W5 Workflow](../../../../.github/workflows/validate-atomic-chart-pr.yaml)
- [Validate Source Branch Action](https://github.com/arustydev/gha/tree/main/actions/trust-check/validate-source-branch)
- [Version Bump Actions](https://github.com/arustydev/gha/tree/main/actions/version-bump)
- [Changelog Actions](https://github.com/arustydev/gha/tree/main/actions/changelog)
