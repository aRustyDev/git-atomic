# Research Findings: Existing ADR Skills

## Skills Analyzed

### 1. melodic-software/adr-management (14★, 43★ repo)
- **Approach**: Canonical ADR skill — create and manage lifecycle
- **Strengths**: Status lifecycle table, template reference, integration with architecture principles, visualization plugin integration
- **Directory**: `/architecture/adr/NNNN-title-in-kebab-case.md`
- **Takeaway**: Good lifecycle model (Proposed → Accepted → Deprecated → Superseded). Borrow status lifecycle and "one decision per ADR" principle.

### 2. rjmurillo/adr-review (9★)
- **Approach**: Multi-agent debate orchestration for ADR validation
- **Strengths**: 6-agent debate (architect, critic, independent-thinker, security, analyst, advisor), P0/P1/P2 issue resolution, strategic review checklist (Chesterton's Fence, Path Dependence, Core vs Context, Second-System Effect)
- **Takeaway**: Strategic validation checklist is excellent — adapt for our Review workflow. Too complex for our needs (multi-agent orchestration), but the checklist concepts are gold.

### 3. codenamev/create-adr (42★)
- **Approach**: Focused ADR creation with input sanitization
- **Strengths**: Filename sanitization, pragmatic mode with complexity scoring, status lifecycle, clear "when to create" vs "when not to create" guidance
- **Directory**: `.architecture/decisions/adrs/ADR-XXX-title.md`
- **Takeaway**: Best "when to create" guidance. Borrow pragmatic mode concept (necessity vs complexity assessment). Good security (input sanitization).

### 4. existential-birds/adr-writing (15★)
- **Approach**: MADR template with Definition of Done (E.C.A.D.R.)
- **Strengths**: E.C.A.D.R. quality criteria (Explicit problem, Comprehensive options, Actionable decision, Documented consequences, Reviewable), INVESTIGATE markers for gaps, YAML frontmatter requirement, parallel number allocation
- **Takeaway**: E.C.A.D.R. is an excellent review framework. INVESTIGATE markers are a smart pattern. Borrow both.

### 5. existential-birds/adr-decision-extraction (15★)
- **Approach**: Extract decisions from conversations and transcripts
- **Strengths**: Detection signals table, confidence levels (high/medium/low), structured JSON output, merge/consolidation rules
- **Takeaway**: Unique workflow — decision extraction from conversations. Include as "Extract" workflow in our skill.

### 6. liza-mas/adr-backfill (26★)
- **Approach**: Reconstruct ADRs from git history (archaeology)
- **Strengths**: File classification tiers (0-3), structural change signals, clustering, gap analysis, state management, chronological ordering
- **Takeaway**: Most comprehensive backfill approach. Include as "Backfill" workflow. Borrow file classification tiers and quality bar.

### 7. terrylica/adr-code-traceability (9★)
- **Approach**: Add ADR references to code
- **Strengths**: Language-specific patterns (Python, TS, Rust, Go), placement decision tree, "when NOT to add" guidance
- **Takeaway**: Include code traceability guidance in references. Good placement decision tree.

### 8. managedcode/mcaf-adr-writing (43★)
- **Approach**: Anti-guesswork ADR writing with mandatory diagrams
- **Strengths**: Decision quality checklist (no placeholders, no hand-waving), mandatory Mermaid diagram, MUST/MUST NOT invariants, stakeholder awareness
- **Takeaway**: "Anti-guesswork" principle is strong. Borrow mandatory diagram requirement and invariants concept.

### 9. lyndonkl/adr-architecture (15★)
- **Approach**: Most comprehensive single-skill with rubric
- **Strengths**: 5-step workflow with progress checklist, common patterns by decision type, quality rubric (score ≥ 3.5), guardrails (do/don't), resource files (template, methodology, examples)
- **Takeaway**: Best structured workflow. Borrow quality rubric concept and common patterns organization.

### 10. basher83/adr-methodology (13★)
- **Approach**: Structured assessment frameworks with state management
- **Strengths**: Two frameworks (Salesforce Well-Architected, Technical Trade-off), risk rating system (Low/Medium/High), YAML state file for multi-session, 3-stage workflow (criteria → matrix → generate), AI disclosure section
- **Takeaway**: Assessment frameworks are useful for complex decisions. State file for multi-session is practical.

## Gap Analysis

| Capability | Existing Skills | Our Differentiation |
|-----------|----------------|---------------------|
| Create ADR | All skills | Integrate with aRustyDev frontmatter + `docs/src/adr/` path |
| Review ADR | rjmurillo (over-complex) | Lightweight checklist using E.C.A.D.R. + strategic lenses |
| Update/Supersede | melodic-software (basic) | Full lifecycle with supersession workflow |
| Backfill from git | liza-mas (standalone) | Integrated workflow within unified skill |
| Extract from conversation | existential-birds (standalone) | Integrated workflow within unified skill |
| Plan (what needs an ADR?) | codenamev (partial) | Decision triggers table + "when NOT to create" |
| Code traceability | terrylica (standalone) | Reference file with language patterns |
| Assessment frameworks | basher83 | Simplified decision matrix in references |
| Quality rubric | lyndonkl | E.C.A.D.R. + quality checklist |

## Key Design Decisions

1. **Single unified skill** covering Author, Review, Plan, Update, Backfill — not separate skills
2. **aRustyDev conventions**: `docs/src/adr/` directory, frontmatter schema, `adr-NNN-title.md` naming
3. **E.C.A.D.R.** as review framework (from existential-birds)
4. **Strategic lenses** from rjmurillo (Chesterton's Fence, Path Dependence, Second-System Effect) — simplified
5. **INVESTIGATE markers** for incomplete sections
6. **Mandatory diagram** requirement (from managedcode)
7. **Code traceability** as reference file, not core workflow
