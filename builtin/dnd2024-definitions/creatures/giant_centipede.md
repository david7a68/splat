---
splat_schema: 1
splat_id: 15457a07-7534-57dd-8d2d-e338cc32d8bc
splat_kind: creature
splat_key: dnd2024:creatures/giant_centipede
splat_name: Giant Centipede
splat_origin:
  key: dnd2024:reference/creature/giant_centipede
splat_sources:
- title: SRD 5.2.1
  locator: pp. 349, 350
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Climb 30 ft.
  senses_text: Blindsight 30 ft.; Passive Perception 8
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 9 (2d6 + 2)
splat_stats:
  strength:
    value: 5
  save_strength:
    value: -3
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 7
  save_wisdom:
    value: -2
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 14
  max_hp:
    value: 9
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 3b822dd4f34e0f993ca895063f9bd27bfe5f1cfa0073cf7c3db416875833f9d1
  source_path: builtin/srd-5.2.1/creature/giant_centipede.md
  pages:
  - 349
  - 350
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Giant Centipede Small Beast, Unaligned AC 14 Initiative +2 (12) HP 9 (2d6 + 2) Speed 30 ft.,
    Climb 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 5 -3 -3 Dex14 +2 +2 Con 12 +1 +1 Int 1 -5 -5 Wis7 -2 -2
    Cha 3 -4 -4 Senses Blindsight 30 ft.; Passive Perception 8 Languages None CR 1/4 (XP 50; PB +2) ###
    Actions Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Piercing damage, and the target
    has the Poisoned condition until the start of the centipede’s next turn.'
  sha256: f6690062cccd1173d185daf655a25e67980f4e4f9e999ce39b73fcf896e89950
  status: needs_semantic_conversion
---

# Giant Centipede

<!-- SRD page 349 -->

Small Beast, Unaligned\
AC 14  Initiative +2 (12)\
HP 9 (2d6 + 2)\
Speed 30 ft., Climb 30 ft.\

<!-- SRD page 350 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 5 −3 −3 Dex14 +2 +2 Con 12 +1 +1\
Int 1 −5 −5 Wis7 −2 −2 Cha 3 −4 −4\
Senses Blindsight 30 ft.; Passive Perception 8\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2)\
Piercing damage, and the target has the Poisoned con-\
dition until the start of the centipede’s next turn.\

Original layout: [[../../srd-5.2.1/pages/349|page 349]], [[../../srd-5.2.1/pages/350|page 350]].
