---
splat_schema: 1
splat_id: 8d9274a1-66f0-5917-9aab-95806a159fe3
splat_kind: rule
splat_key: dnd2024:rules/weapons_versatile
splat_name: Versatile
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_versatile
  parameters:
    hands_options:
    - 1
    - 2
    two_handed_melee_damage: weapon_versatile_damage
    other_damage: weapon_base_damage
  implementation: dnd2024:rule_weapons_versatile
splat_contracts:
- id: dnd2024:rule_weapons_versatile
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
  - dnd2024:rule_weapons_versatile
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 42d3c83259ea9eccfbbdfb8cb26479e2f880546e51b717cc4bae6ffa04c3fd0d
  parent_name: Weapons
  line_span:
    first: 7756
    count: 5
splat_clauses:
- key: source_rules
  text: '# Versatile A Versatile weapon can be used with one or two hands. A damage value in parentheses
    appears with the property. The weapon deals that damage when used with two hands to make a melee attack.'
  sha256: f21d6694f3fa0cb99fac5e752613efd5b6086034c47f1a254a0d4b1e4139326c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Versatile

<!-- SRD page 90 -->

A Versatile weapon can be used with one or two\
hands. A damage value in parentheses appears with\
the property. The weapon deals that damage when\
used with two hands to make a melee attack.\
