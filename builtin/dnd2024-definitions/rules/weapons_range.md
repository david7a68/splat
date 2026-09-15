---
splat_schema: 1
splat_id: ae8358e1-ebac-5cbf-8b1a-65380c39bfe5
splat_kind: rule
splat_key: dnd2024:rules/weapons_range
splat_name: Range
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_range
  parameters:
    range_unit: feet
    first_number: normal_range
    second_number: long_range
    range_follows_property:
    - ammunition
    - thrown
    beyond_normal: disadvantage
    beyond_long: prohibited
  implementation: dnd2024:rules_weapons_range
splat_contracts:
- id: dnd2024:rules_weapons_range
  inputs:
    attack:
      type: resolution
      required: true
  outputs:
  - eligibility
  - roll_modifiers
  events: []
  capabilities:
  - weapon_properties
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 90
  engine_gaps: &id001
  - dnd2024:rules_weapons_range
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 648c63480587f7c40aae9c5200fdf76661567d2898d7f29ba343120423a7dcac
  parent_name: Weapons
  line_span:
    first: 7734
    count: 8
splat_clauses:
- key: source_rules
  text: '# Range A Range weapon has a range in parentheses after the Ammunition or Thrown property. The
    range lists two numbers. The first is the weapon’s normal range in feet, and the second is the weapon’s
    long range. When attacking a target beyond normal range, you have Disadvantage on the attack roll.
    You can’t attack a target beyond the long range.'
  sha256: 3719a6b4988e2ffcd034c2d876d0e571d65f5dd22a1e2af2722aa67c13020962
  status: contract_specified
splat_requires_mechanics: *id001
---

# Range

<!-- SRD page 90 -->

A Range weapon has a range in parentheses after\
the Ammunition or Thrown property. The range\
lists two numbers. The first is the weapon’s normal\
range in feet, and the second is the weapon’s long\
range. When attacking a target beyond normal\
range, you have Disadvantage on the attack roll. You\
can’t attack a target beyond the long range.\
