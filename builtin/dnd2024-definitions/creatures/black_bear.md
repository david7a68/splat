---
splat_schema: 1
splat_id: e76f4d97-c2ca-599e-a26e-7ded8834ee22
splat_kind: creature
splat_key: dnd2024:creatures/black_bear
splat_name: Black Bear
splat_origin:
  key: dnd2024:reference/creature/black_bear
splat_sources:
- title: SRD 5.2.1
  locator: pp. 345
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Climb 30 ft., Swim 30 ft.
  skills_text: Perception +5
  senses_text: Darkvision 60 ft.; Passive Perception 15
  languages_text: None
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 19 (3d8 + 6)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 11
  max_hp:
    value: 19
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
  climb_speed:
    value: 30
    unit: ft
  swim_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 1d66d2ea0cb1546b99eb8212a9088422199dd0d0449c4860f183beb3669a6d69
  source_path: builtin/srd-5.2.1/creature/black_bear.md
  pages:
  - 345
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles:
  - dnd2024:creatures/black_bear
splat_clauses:
- key: source_rules
  text: '# Black Bear Medium Beast, Unaligned AC 11 Initiative +1 (11) HP 19 (3d8 + 6) Speed 30 ft., Climb
    30 ft., Swim 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 12 +1 +1 Con 14 +2 +2 Int 2 -4 -4
    Wis 12 +1 +1 Cha 7 -2 -2 Skills Perception +5 Senses Darkvision 60 ft.; Passive Perception 15 Languages
    None CR 1/2 (XP 100; PB +2) ### Actions Multiattack. The bear makes two Rend attacks. Rend. Melee
    Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Slashing damage.'
  sha256: a0720d2cb50d42c2f0acf5d806965526b913527deb9de472d7c4c77a124cdac3
  status: needs_semantic_conversion
---

# Black Bear

<!-- SRD page 345 -->

Medium Beast, Unaligned\
AC 11 Initiative +1 (11)\
HP 19 (3d8 + 6)\
Speed 30 ft., Climb 30 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 12 +1 +1 Con 14 +2 +2\
Int 2 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
Skills Perception +5\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Actions

Multiattack. The bear makes two Rend attacks.\
Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2)\
Slashing damage.\

Original layout: [[../../srd-5.2.1/pages/345|page 345]].
