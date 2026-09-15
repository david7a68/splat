---
splat_schema: 1
splat_id: c0eb6b9d-4c22-5322-8308-8c5f30244c80
splat_kind: rule
splat_key: dnd2024:rules/multiclassing_armor_class
splat_name: Armor Class
splat_origin:
  key: dnd2024:reference/pages/025
splat_sources:
- title: SRD 5.2.1
  locator: pp. 25
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/multiclassing_armor_class
  parameters:
    ac_calculations_active_maximum: 1
    selection: player_choice
  implementation: dnd2024:rule_multiclassing_armor_class
splat_contracts:
- id: dnd2024:rule_multiclassing_armor_class
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
  - statistics
  events: []
  capabilities:
  - statistics
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 25
  engine_gaps: &id001
  - dnd2024:rule_multiclassing_armor_class
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 58ed4be06c5e50e3c63a3d2a2f3fe98abd94e3114b1efeff186650ef7d5db508
  parent_name: Multiclassing
  line_span:
    first: 1960
    count: 7
splat_clauses:
- key: source_rules
  text: '# Armor Class If you have multiple ways to calculate your Armor Class, you can benefit from only
    one at a time. For example, a Monk/Sorcerer with a Monk’s Unarmored Defense feature and a Sorcerer’s
    Draconic Resilience feature must choose only one of those features as a way to calculate Armor Class.'
  sha256: 306cb993046bc82d38133440acfc690182dcb406369a69767db1aff61f6ea13f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Armor Class

<!-- SRD page 25 -->

If you have multiple ways to calculate your Armor\
Class, you can benefit from only one at a time.\
For example, a Monk/Sorcerer with a Monk’s\
Unarmored Defense feature and a Sorcerer’s Dra-\
conic Resilience feature must choose only one of\
those features as a way to calculate Armor Class.\
