---
splat_schema: 1
splat_id: 2fd061de-bedc-5b54-adcd-0f6dd4ea5d7f
splat_kind: rule
splat_key: dnd2024:rules/weapons_cleave
splat_name: Cleave
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_cleave
  parameters:
    trigger:
      melee_attack_hit: true
      target_kind: creature
      weapon: this_weapon
    optional: true
    extra_attack:
      kind: melee
      weapon: same_weapon
      target: different_creature
      target_distance_from_first_max_ft: 5
      target_within_attacker_reach: true
    extra_hit_damage:
      weapon_damage: true
      ability_modifier: only_if_negative
    uses:
      max: 1
      period: turn
  implementation: dnd2024:rules_weapons_cleave
splat_contracts:
- id: dnd2024:rules_weapons_cleave
  inputs:
    subject:
      type: participant
      required: true
    first_target:
      type: participant
      required: true
    second_target:
      type: participant
      required: true
    weapon:
      type: item_instance
      required: true
  outputs:
  - effects
  events:
  - attack.hit
  capabilities:
  - weapon_mastery
  - attacks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 90
  engine_gaps: &id001
  - dnd2024:rules_weapons_cleave
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: abbfc6dfde87bbad805fae704ab464ddb0dbd0667e79412bf50665462a398d73
  parent_name: Weapons
  line_span:
    first: 7772
    count: 9
splat_clauses:
- key: source_rules
  text: '# Cleave If you hit a creature with a melee attack roll using this weapon, you can make a melee
    attack roll with the weapon against a second creature within 5 feet of the first that is also within
    your reach. On a hit, the second creature takes the weapon’s damage, but don’t add your ability modifier
    to that damage unless that modifier is negative. You can make this extra attack only once per turn.'
  sha256: 7c01767cbb8419166b0a09a3f3ac879d2e14d1caa788ace948f3bb25145df48e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cleave

<!-- SRD page 90 -->

If you hit a creature with a melee attack roll using\
this weapon, you can make a melee attack roll with\
the weapon against a second creature within 5 feet\
of the first that is also within your reach. On a hit,\
the second creature takes the weapon’s damage,\
but don’t add your ability modifier to that damage\
unless that modifier is negative. You can make this\
extra attack only once per turn.\
