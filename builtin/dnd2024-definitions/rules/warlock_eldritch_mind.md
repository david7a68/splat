---
splat_schema: 1
splat_id: 7dc0e8db-1a18-5cfa-bb47-344b5335aeef
splat_kind: rule
splat_key: dnd2024:rules/warlock_eldritch_mind
splat_name: Eldritch Mind
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_eldritch_mind
  parameters:
    advantage:
      save_ability: constitution
      purpose: maintain_concentration
  implementation: dnd2024:rule_warlock_eldritch_mind
splat_contracts:
- id: dnd2024:rule_warlock_eldritch_mind
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - feature_effects
  events: []
  capabilities:
  - invocations
  - concentration
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 72
  engine_gaps: &id001
  - dnd2024:rule_warlock_eldritch_mind
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 92ce6e9d4127d70186478899249989da4bcc3a83c786fab3966ce05063e24ebf
  parent_name: Warlock
  line_span:
    first: 6170
    count: 3
splat_clauses:
- key: source_rules
  text: '# Eldritch Mind You have Advantage on Constitution saving throws that you make to maintain Concentration.'
  sha256: b7eb2803d0fbe33b3aa3297e2b2a97b63fc3093a92166a523a09341376dae229
  status: contract_specified
splat_requires_mechanics: *id001
---

# Eldritch Mind

<!-- SRD page 72 -->

You have Advantage on Constitution saving throws\
that you make to maintain Concentration.\
