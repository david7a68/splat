---
splat_schema: 1
splat_id: 2a3dabab-9c38-55dc-9bf9-412bc5eaef1a
splat_kind: rule
splat_key: dnd2024:rules/weapons_reach
splat_name: Reach
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_reach
  parameters:
    reach_bonus_feet: 5
    scope:
    - attacks_with_weapon
    - opportunity_attack_reach_with_weapon
  implementation: dnd2024:rule_weapons_reach
splat_contracts:
- id: dnd2024:rule_weapons_reach
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - item_effects
  events: []
  capabilities:
  - equipment
  - attacks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 90
  engine_gaps: &id001
  - dnd2024:rule_weapons_reach
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 3b09d614dbb1543f6fee264b6d4ba89960cb78ff100411d9d0b598173a367d87
  parent_name: Weapons
  line_span:
    first: 7742
    count: 4
splat_clauses:
- key: source_rules
  text: '# Reach A Reach weapon adds 5 feet to your reach when you attack with it, as well as when determining
    your reach for Opportunity Attacks with it.'
  sha256: 6a2ee67cc7014cba21164dc201eda848c9b588cad240ad06922bf896b20f5e32
  status: contract_specified
splat_requires_mechanics: *id001
---

# Reach

<!-- SRD page 90 -->

A Reach weapon adds 5 feet to your reach when you\
attack with it, as well as when determining your\
reach for Opportunity Attacks with it.\
