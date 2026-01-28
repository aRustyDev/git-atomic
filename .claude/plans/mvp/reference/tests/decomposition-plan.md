# Test Plan Decomposition Plan

## Objective

Break the monolithic `plan.md` (778 lines) into focused phase documents for easier maintenance, review, and execution.

## Target Structure

```
tests/
├── index.md                 # Overview, prerequisites, execution order
├── phase-1-auto-merge.md    # AM-* tests (Auto-Merge workflow)
├── phase-2-w2-filter.md     # W2-* tests (Create Atomic Chart PR)
├── phase-3-w5-validate.md   # W5-* tests (Validate Atomic Chart PR)
├── phase-4-release.md       # R-* tests (Release workflow)
├── phase-5-attestation.md   # AT-*, AL-* tests (Attestation & Lineage)
├── phase-6-e2e.md           # E2E-* tests (End-to-End scenarios)
├── cleanup.md               # Cleanup procedures (shared)
└── plan.md                  # Original (archive/reference)
```

## Decomposition Steps

### Step 1: Create index.md
Extract from plan.md:
- Overview section
- Prerequisites (Repository Configuration, Test User Accounts, Test Chart)
- Test Order Dependencies
- Test Isolation guidelines
- Failure Investigation checklist
- Link to each phase document

### Step 2: Create phase-1-auto-merge.md
Extract from plan.md:
- Auto-Merge Workflow section (lines 58-85)
- Controls table (AM-C1 through AM-C6)
- Test Matrix (AM-T1 through AM-T10)
- Update code locations for action-based implementation

### Step 3: Create phase-2-w2-filter.md
Extract from plan.md:
- W2: Create Atomic Chart PR section (lines 88-112)
- Controls table (W2-C1 through W2-C6)
- Test Matrix (W2-T1 through W2-T7)
- Update code locations for action-based implementation

### Step 4: Create phase-3-w5-validate.md
Extract from plan.md:
- W5: Validate Atomic Chart PR section (lines 115-148)
- Controls table (W5-C1 through W5-C9)
- Test Matrix (W5-T1 through W5-T13)
- Update code locations for action-based implementation

### Step 5: Create phase-4-release.md
Extract from plan.md:
- Release Workflow section (lines 151-180)
- Controls table (R-C1 through R-C9)
- Test Matrix (R-T1 through R-T9)

### Step 6: Create phase-5-attestation.md
Extract from plan.md:
- Attestation and Provenance section (lines 183-247)
- Controls table (AT-C1 through AT-C6)
- Test Matrix (AT-T1 through AT-T8)
- Attestation Lineage Tests (AL-T1 through AL-T6)
- Verification Commands Reference

### Step 7: Create phase-6-e2e.md
Extract from plan.md:
- End-to-End Test Scenarios section (lines 251-651)
- E2E-1 through E2E-10
- Each with Steps, Expected, Verification, Cleanup

### Step 8: Create cleanup.md
Extract from plan.md:
- Cleanup Procedures section (lines 654-693)
- After Each Test procedures
- Artifacts to Preserve
- Artifacts to Remove

### Step 9: Update index.md with execution checklist
Move Test Execution Checklist from plan.md to index.md

### Step 10: Archive original plan.md
Rename to `plan-archive.md` or add deprecation notice

## Code Location Updates

Each phase document should update control tables with current implementation:

| Phase | Controls to Update |
|-------|-------------------|
| Phase 1 | AM-C5 (codeowners action), AM-C6 (verify-signatures action) |
| Phase 2 | W2-C5 (create-branch action), W2-C6 (create-pr action) |
| Phase 3 | W5-C3 (validate-source-branch action), W5-C8 (version-bump actions) |
| Phase 4 | No changes (release workflow unchanged) |
| Phase 5 | No changes (attestation verification unchanged) |
| Phase 6 | Reference updated phase docs |

## Execution Order

1. Create index.md (skeleton)
2. Create phase documents (1-6)
3. Create cleanup.md
4. Update index.md with links and checklist
5. Add deprecation notice to original plan.md
6. Review each phase with sub-agents
