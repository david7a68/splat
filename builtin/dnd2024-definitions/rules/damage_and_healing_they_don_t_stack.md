---
splat_schema: 1
splat_id: 5f2dfc80-4a5b-5ea3-b79d-0984f19dffc8
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_they_don_t_stack
splat_name: They Don’t Stack
splat_origin:
  key: dnd2024:reference/pages/018
splat_sources:
- title: SRD 5.2.1
  locator: pp. 18
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_they_don_t_stack
  parameters:
    temporary_hit_points_stacking: false
    new_grant_choice:
    - retain_existing
    - replace_with_new
  implementation: dnd2024:rule_damage_and_healing_they_don_t_stack
splat_contracts:
- id: dnd2024:rule_damage_and_healing_they_don_t_stack
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - hit_points
  events: []
  capabilities:
  - hit_points
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 18
  engine_gaps: &id001
  - dnd2024:rule_damage_and_healing_they_don_t_stack
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 4d6be2e16dfed98a560be25114e990fffde3f2860cbcc98064390128710d9acf
  parent_name: Damage and Healing
  line_span:
    first: 1301
    count: 7
splat_clauses:
- key: source_rules
  text: '# They Don’t Stack Temporary Hit Points can’t be added together. If you have Temporary Hit Points
    and receive more of them, you decide whether to keep the ones you have or to gain the new ones. For
    example, if a spell grants you 12 Temporary Hit Points when you already have 10, you can have 12 or
    10, not 22.'
  sha256: 6b2fdb25fc7561808cf5c96f60eda912f8f662b67b9109e8bef98e89793faaf6
  status: contract_specified
splat_requires_mechanics: *id001
---

# They Don’t Stack

<!-- SRD page 18 -->

Temporary Hit Points can’t be added together. If\
you have Temporary Hit Points and receive more\
of them, you decide whether to keep the ones you\
have or to gain the new ones. For example, if a spell\
grants you 12 Temporary Hit Points when you al-\
ready have 10, you can have 12 or 10, not 22.\
