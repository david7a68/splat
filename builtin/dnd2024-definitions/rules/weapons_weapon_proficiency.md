---
splat_schema: 1
splat_id: 186e1d16-7b5f-5615-9830-e45b15865420
splat_kind: rule
splat_key: dnd2024:rules/weapons_weapon_proficiency
splat_name: Weapon Proficiency
splat_origin:
  key: dnd2024:reference/pages/089
splat_sources:
- title: SRD 5.2.1
  locator: pp. 89
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_weapon_proficiency
  parameters:
    wield_requires_proficiency: false
    attack_proficiency_bonus:
      requires_weapon_proficiency: true
    monster_proficiency:
      all_stat_block_weapons: true
  implementation: dnd2024:rules_weapons_weapon_proficiency
splat_contracts:
- id: dnd2024:rules_weapons_weapon_proficiency
  inputs:
    actor:
      type: participant
      required: true
    test:
      type: resolution
      required: true
  outputs:
  - test_modifiers
  - test_result
  events: []
  capabilities:
  - d20_tests
  - proficiency
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 89
  engine_gaps: &id001
  - dnd2024:rules_weapons_weapon_proficiency
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 5ef840df4af38056555f1af3f09996639f7ababba1257fe3f0faa20c8ed2b14b
  parent_name: Weapons
  line_span:
    first: 7686
    count: 6
splat_clauses:
- key: source_rules
  text: '# Weapon Proficiency Anyone can wield a weapon, but you must have proficiency with it to add
    your Proficiency Bonus to an attack roll you make with it. A player character’s features can provide
    weapon proficiencies. A monster is proficient with any weapon in its stat block.'
  sha256: 4efa8b0de906cd340b0d8f00f4c4618684dc3b841f1025a98a0abe459b57bf57
  status: contract_specified
splat_requires_mechanics: *id001
---

# Weapon Proficiency

<!-- SRD page 89 -->

Anyone can wield a weapon, but you must have\
proficiency with it to add your Proficiency Bonus to\
an attack roll you make with it. A player character’s\
features can provide weapon proficiencies. A mon-\
ster is proficient with any weapon in its stat block.\
