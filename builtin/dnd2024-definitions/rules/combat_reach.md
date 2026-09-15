---
splat_schema: 1
splat_id: c15c1225-c9c7-51e4-86a3-1e9e80d01e46
splat_kind: rule
splat_key: dnd2024:rules/combat_reach
splat_name: Reach
splat_origin:
  key: dnd2024:reference/pages/015
splat_sources:
- title: SRD 5.2.1
  locator: pp. 15
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_reach
  parameters:
    default_reach_ft: 5
    attack_requires_within_reach: true
    specific_attack_reach_overrides: true
  implementation: dnd2024:rules_combat_reach
splat_contracts:
- id: dnd2024:rules_combat_reach
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - melee_reach
  events: []
  capabilities:
  - attacks
  - geometry
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 15
  engine_gaps: &id001
  - dnd2024:rules_combat_reach
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: f74b690b10049b4aeefea0c7a8ae8aa54ed0b33bc1a0078a6d5b462b64ca7a5a
  parent_name: Combat
  line_span:
    first: 1036
    count: 5
splat_clauses:
- key: source_rules
  text: '# Reach A creature has a 5-foot reach and can thus attack targets within 5 feet when making a
    melee attack. Certain creatures have melee attacks with a reach greater than 5 feet, as noted in their
    descriptions.'
  sha256: 3ed1c3d8e7706503a3863908fd0c2669521ff51842d19d406aa5ed290bf7ce58
  status: contract_specified
splat_requires_mechanics: *id001
---

# Reach

<!-- SRD page 15 -->

A creature has a 5-foot reach and can thus attack\
targets within 5 feet when making a melee attack.\
Certain creatures have melee attacks with a reach\
greater than 5 feet, as noted in their descriptions.\
