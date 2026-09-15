---
splat_schema: 1
splat_id: 57ac56de-6e36-5dd6-be4a-7c8729db2ac1
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_critical_hits
splat_name: Critical Hits
splat_origin:
  key: dnd2024:reference/pages/016
splat_sources:
- title: SRD 5.2.1
  locator: pp. 16
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_critical_hits
  parameters:
    critical_hit:
      damage_dice_multiplier: 2
      include_additional_attack_dice: true
      flat_modifiers_multiplier: 1
  implementation: dnd2024:rules_damage_and_healing_critical_hits
splat_contracts:
- id: dnd2024:rules_damage_and_healing_critical_hits
  inputs:
    attack:
      type: resolution
      required: true
  outputs:
  - damage_roll
  events:
  - attack.critical_hit
  capabilities:
  - damage
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 16
  engine_gaps: &id001
  - dnd2024:rules_damage_and_healing_critical_hits
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 5161c6822fdb03cf74867e4bfd8b136ac6363c342dcdb33f486be43087a9efd5
  parent_name: Damage and Healing
  line_span:
    first: 1142
    count: 9
splat_clauses:
- key: source_rules
  text: '# Critical Hits When you score a Critical Hit, you deal extra damage. Roll the attack’s damage
    dice twice, add them together, and add any relevant modifiers as normal. For example, if you score
    a Critical Hit with a Dagger, roll 2d4 for the damage rather than 1d4, and add your relevant ability
    modifier. If the attack involves other damage dice, such as from the Rogue’s Sneak Attack feature,
    you also roll those dice twice.'
  sha256: 7c2593858dcba2c7a863ce9c414b028aca243b6d86e7ca184d43b58ff4aaa78b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Critical Hits

<!-- SRD page 16 -->

When you score a Critical Hit, you deal extra dam-\
age. Roll the attack’s damage dice twice, add them\
together, and add any relevant modifiers as normal.\
For example, if you score a Critical Hit with a Dag-\
ger, roll 2d4 for the damage rather than 1d4, and\
add your relevant ability modifier. If the attack in-\
volves other damage dice, such as from the Rogue’s\
Sneak Attack feature, you also roll those dice twice.\
