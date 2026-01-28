# Consolidated Gap Analysis Report

**Generated:** 2026-01-27
**Scope:** Chart Release Workflow Test Plan (Phases 1-6)

---

## Executive Summary

Sub-agent review identified **72+ gaps** across all test phases:

| Phase | High | Medium | Low | Total |
|-------|------|--------|-----|-------|
| Phase 1 (Auto-Merge) | 4 | 5 | 8 | 17 |
| Phase 2 (W2 Filter) | 4 | 5 | 3 | 12 |
| Phase 3 (W5 Validate) | 6 | 4 | 3 | 13 |
| Phase 4 (Release) | 3 | 2 | 0 | 5 |
| Phase 5 (Attestation) | 5 | 5 | 3 | 13 |
| Phase 6 (E2E) | 5 | 10 | 10 | 25 |
| **Total** | **27** | **31** | **27** | **85** |

**Critical Blocking Issues:** 1 (2 resolved)
**Recommended Test Order:** E2E-1 first (creates artifacts for later tests)

---

## Critical Blocking Issues

These must be resolved before testing begins:

### 1. ~~Action Output Field Name Mismatch (Phase 1)~~ ✅ RESOLVED
- **Location:** `verify-signatures` action / `auto-merge-integration.yaml`
- **Issue:** Test plan referenced `all-verified` but action outputs `all-signed`
- **Impact:** All signature verification tests would fail
- **Resolution:** Updated phase-1-auto-merge.md to use `all-signed` (matches workflow line 207)

### 2. ~~Dispatch Trigger Authentication (Phase 2)~~ ✅ VERIFIED
- **Location:** `dispatch/trigger` action
- **Issue:** Was concern that `GITHUB_TOKEN` may be insufficient
- **Finding:** `GITHUB_TOKEN` CAN trigger `repository_dispatch` in same repo
- **Actor:** `github-actions[bot]` - matches W5's `ALLOWED_DISPATCH_ACTORS`
- **Resolution:** No action needed - configuration is correct

### 3. Attestation Map Propagation (Phase 5/6)
- **Location:** `create-pr` action
- **Issue:** No tests verify attestation map survives PR updates
- **Impact:** Lineage chain may break on PR updates
- **Resolution:** Add AT-T9 test for PR update attestation preservation

---

## Phase 1: Auto-Merge Workflow Gaps

### High Severity

| ID | Gap | Recommended Fix |
|----|-----|-----------------|
| ~~AM-H1~~ | ~~Output field name mismatch (`all-verified` vs `all-signed`)~~ | ✅ RESOLVED - Updated test plan |
| AM-H2 | No test for CODEOWNERS file not found | Add AM-T11: missing CODEOWNERS test |
| AM-H3 | Signature check edge cases untested (mixed signed/unsigned) | Add AM-T12: partial signature test |
| AM-H4 | Bot author detection not explicitly tested | Add AM-T13: bot author (dependabot) test |

### Medium Severity

| ID | Gap | Recommended Fix |
|----|-----|-----------------|
| AM-M1 | CODEOWNERS pattern matching not tested (wildcards, teams) | Add tests for `@org/team` patterns |
| AM-M2 | No test for expired GPG keys | Add AM-T14: expired key test |
| AM-M3 | Concurrent PR handling not tested | Add AM-T15: race condition test |
| AM-M4 | `ALLOWED_BASE_BRANCHES` edge cases (empty, malformed) | Add variable validation tests |
| AM-M5 | No performance baseline established | Document expected workflow duration |

### Low Severity

| ID | Gap | Recommended Fix |
|----|-----|-----------------|
| AM-L1 | Multiple matching CODEOWNERS patterns not tested | Document expected behavior |
| AM-L2 | Cleanup procedures incomplete for partial failures | Add rollback procedures |
| AM-L3 | No test log capture automation | Add script to save workflow logs |

---

## Phase 2: W2 Filter Charts Gaps

### High Severity

| ID | Gap | Recommended Fix |
|----|-----|-----------------|
| W2-H1 | Dispatch trigger not explicitly tested | Add W2-T8: verify W5 triggers via dispatch |
| W2-H2 | Attestation map format not validated | Add W2-T9: attestation map structure test |
| W2-H3 | PR body update on re-push not verified | Add test for attestation map preservation |
| W2-H4 | Branch conflict resolution untested | Add W2-T10: concurrent branch update test |

### Medium Severity

| ID | Gap | Recommended Fix |
|----|-----|-----------------|
| W2-M1 | `detect-changes` action edge cases (renamed charts) | Add test for chart rename |
| W2-M2 | Matrix failure handling not tested | Add test for partial matrix failure |
| W2-M3 | Large number of charts not tested | Add stress test (10+ charts) |
| W2-M4 | Dependencies job interaction with chart jobs | Document expected behavior |
| W2-M5 | Chart.yaml parse errors not tested | Add test for malformed Chart.yaml |

### Low Severity

| ID | Gap | Recommended Fix |
|----|-----|-----------------|
| W2-L1 | Concurrency group collision with other workflows | Verify group naming uniqueness |
| W2-L2 | No cleanup for failed matrix jobs | Add matrix cleanup step |
| W2-L3 | Source PR metadata propagation not verified | Add metadata verification |

---

## Phase 3: W5 Validate Gaps

### High Severity

| ID | Gap | Recommended Fix |
|----|-----|-----------------|
| W5-H1 | `repository_dispatch` trigger untested | Add W5-T14: dispatch trigger test |
| W5-H2 | `ALLOWED_DISPATCH_ACTORS` validation untested | Add W5-T15: unauthorized actor test |
| W5-H3 | GitHub App token authentication untested | Add W5-T16: token permission test |
| W5-H4 | Version bump commit signing untested | Verify bump commits are signed |
| W5-H5 | Changelog generation failure handling untested | Add W5-T17: git-cliff failure test |
| W5-H6 | Already-bumped detection logic untested | Add W5-T18: re-run detection test |

### Medium Severity

| ID | Gap | Recommended Fix |
|----|-----|-----------------|
| W5-M1 | K8s version matrix edge cases (deprecated APIs) | Add API deprecation test |
| W5-M2 | ct lint configuration coverage unclear | Document ct.yaml settings tested |
| W5-M3 | ArtifactHub metadata validation scope unclear | Document ah-lint coverage |
| W5-M4 | Branch deletion failure handling | Add cleanup failure test |

### Low Severity

| ID | Gap | Recommended Fix |
|----|-----|-----------------|
| W5-L1 | git-cliff installation time impact | Add timing metrics |
| W5-L2 | Multi-chart PR behavior documented but untested | Add integration test |
| W5-L3 | Workflow run duration not baselined | Add performance tracking |

---

## Phase 4: Release Workflow Gaps

### High Severity

| ID | Gap | Recommended Fix |
|----|-----|-----------------|
| R-H1 | GHCR authentication failure not tested | Add R-T10: GHCR auth failure test |
| R-H2 | Cosign signing failure not tested | Add R-T11: Cosign failure test |
| R-H3 | Release branch update failure not tested | Add R-T12: release branch conflict test |

### Medium Severity

| ID | Gap | Recommended Fix |
|----|-----|-----------------|
| R-M1 | `extract-version` action failure modes not tested | Add R-T13: version extraction failure |
| R-M2 | GitHub Release rate limiting not tested | Document rate limit handling |

---

## Phase 5: Attestation Gaps

### High Severity

| ID | Gap | Recommended Fix |
|----|-----|-----------------|
| AT-H1 | Attestation bundle format validation missing | Add AT-T9: bundle structure test |
| AT-H2 | Certificate chain verification untested | Add AT-T10: OIDC cert chain test |
| AT-H3 | Attestation expiry handling untested | Document attestation TTL |
| AT-H4 | Sigstore outage handling untested | Add AT-T11: Sigstore unavailable test |
| AL-H1 | Attestation map preservation on PR update untested | Add AL-T7: map update preservation |

### Medium Severity

| ID | Gap | Recommended Fix |
|----|-----|-----------------|
| AT-M1 | Multiple attestations on same artifact untested | Add multi-attestation test |
| AT-M2 | Attestation size limits untested | Document max attestation size |
| AT-M3 | Cosign verification with wrong identity untested | Add identity mismatch test |
| AL-M1 | Fork contribution lineage untested | Add fork-based lineage test |
| AL-M2 | Merge commit vs squash commit lineage difference | Document merge strategy impact |

### Low Severity

| ID | Gap | Recommended Fix |
|----|-----|-----------------|
| AT-L1 | Attestation JSON schema not documented | Add schema reference |
| AT-L2 | Cosign tree output format not documented | Add output examples |
| AL-L1 | Lineage trace script error handling incomplete | Add error handling |

---

## Phase 6: E2E Gaps

### High Severity

| ID | Gap | Recommended Fix |
|----|-----|-----------------|
| E2E-H1 | Race conditions between W2 matrix jobs | Add concurrency tests |
| E2E-H2 | Attestation validation in E2E-1 is post-hoc | Move to inline verification |
| E2E-H3 | E2E-4 multiple charts may cause timing issues | Add synchronization logic |
| E2E-H4 | E2E-5 bypass detection is reactive only | Add proactive blocking test |
| E2E-H5 | No rollback scenario tested | Add E2E-11: rollback test |

### Medium Severity

| ID | Gap | Recommended Fix |
|----|-----|-----------------|
| E2E-M1 | E2E-2 fork-based testing infrastructure lacking | Set up test fork |
| E2E-M2 | E2E-3 unsigned test depends on user without GPG | Document test user setup |
| E2E-M3 | E2E-6 K8s failure simulation may be fragile | Add reliable failure trigger |
| E2E-M4 | E2E-7 lint failure may block other tests | Add test isolation |
| E2E-M5 | E2E-8 version collision detection untested | Add collision scenario |
| E2E-M6 | E2E-9 tampering detection needs automation | Add tamper detection script |
| E2E-M7 | E2E-10 lineage script has hardcoded values | Parameterize script |
| E2E-M8 | No test for workflow timeout scenarios | Add timeout test |
| E2E-M9 | No test for GitHub API rate limiting | Document rate limit handling |
| E2E-M10 | Cross-workflow data passing reliability untested | Add data persistence test |

### Low Severity

| ID | Gap | Recommended Fix |
|----|-----|-----------------|
| E2E-L1-L10 | Various documentation and cleanup gaps | See detailed phase docs |

---

## Prioritized Action Items

### Immediate (Before Testing Begins)

1. **Verify action output field names** in `arustydev/gha`
   - `verify-signatures` → `all-verified` vs `all-signed`
   - `codeowners` → `is-owner` field confirmation

2. **Confirm dispatch trigger permissions**
   - Verify `dispatch/trigger` action uses correct token
   - Test dispatch manually if needed

3. **Set up test infrastructure**
   - Ensure test-workflow chart exists at version 0.1.0
   - Verify CODEOWNERS includes test user
   - Confirm GPG key configured for test user

### Short-term (First Test Iteration)

4. **Add missing high-severity tests**
   - W2-T8: Dispatch trigger verification
   - W5-T14: repository_dispatch test
   - R-T10: GHCR auth failure
   - AT-T9: Attestation bundle validation

5. **Document expected outputs**
   - Create output reference for each action
   - Add sample workflow logs

### Medium-term (Second Iteration)

6. **Add medium-severity tests**
   - Edge case testing for all actions
   - Error handling verification
   - Performance baseline establishment

7. **Improve automation**
   - Log capture automation
   - Cleanup scripts
   - Parameterized test scripts

---

## Test Execution Recommendation

Based on gap analysis, recommended test order:

1. **Manual Verification First**
   - Verify action output field names
   - Test dispatch trigger manually
   - Confirm token permissions

2. **Happy Path (E2E-1)**
   - Creates artifacts needed for later tests
   - Validates end-to-end flow

3. **Critical Path Tests**
   - AM-T5, AM-T7, AM-T9 (trust controls)
   - W2-T5 (branch/PR creation)
   - W5-T10-T12 (version bumping)
   - R-T6-T8 (release creation)

4. **Attestation Tests**
   - AT-T2, AT-T4, AT-T5 (verification)
   - AT-T7 (tamper detection)
   - AL-T5 (lineage)

5. **Failure Scenario Tests**
   - E2E-6, E2E-7 (blocking tests)
   - E2E-2, E2E-3 (trust failures)

---

## Related Issues

- [Remove deprecated scripts](https://github.com/arustydev/gh/issues/49)
- [Tag gha actions as v1](https://github.com/arustydev/gh/issues/50)

---

## Next Steps

1. Review this gap analysis
2. Address critical blocking issues
3. Execute E2E-1 happy path
4. Iterate based on findings
