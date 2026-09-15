---
splat_schema: 1
splat_id: 63c848ff-ddfe-5e35-9e68-d069146287ac
splat_kind: rule
splat_key: dnd2024:rules/combat_impeded_weapons
splat_name: Impeded Weapons
splat_origin:
  key: dnd2024:reference/pages/016
splat_sources:
- title: SRD 5.2.1
  locator: pp. 16
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_impeded_weapons
  parameters:
    environment: underwater
    melee_weapon_attack:
      disadvantage_when:
        swim_speed: false
        damage_type_not: piercing
    ranged_weapon_attack:
      beyond_normal_range: automatic_miss
      within_normal_range: disadvantage
  implementation: dnd2024:rule_combat_impeded_weapons
splat_contracts:
- id: dnd2024:rule_combat_impeded_weapons
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - attack_modifiers
  events: []
  capabilities:
  - attacks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 16
  engine_gaps: &id001
  - dnd2024:rule_combat_impeded_weapons
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 129f58b218f45ecb201ea4625e3aee4d924fb48697d098a87ffdf231678a83b0
  parent_name: Combat
  line_span:
    first: 1092
    count: 9
splat_clauses:
- key: source_rules
  text: '# Impeded Weapons When making a melee attack roll with a weapon underwater, a creature that lacks
    a Swim Speed has Disadvantage on the attack roll unless the weapon deals Piercing damage. A ranged
    attack roll with a weapon underwater automatically misses a target beyond the weapon’s normal range,
    and the attack roll has Disadvantage against a target within normal range.'
  sha256: ff9599c10e607ad3b25a6cc5c0715a3406a50dce167d94d330607e5cb5aaa84f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Impeded Weapons

<!-- SRD page 16 -->

When making a melee attack roll with a weapon\
underwater, a creature that lacks a Swim Speed has\
Disadvantage on the attack roll unless the weapon\
deals Piercing damage.\
A ranged attack roll with a weapon underwater\
automatically misses a target beyond the weapon’s\
normal range, and the attack roll has Disadvantage\
against a target within normal range.\
