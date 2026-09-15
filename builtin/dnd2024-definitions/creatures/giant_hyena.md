---
splat_schema: 1
splat_id: f72fd22b-59c0-5ac7-acd1-5f9ed6b376bd
splat_kind: creature
splat_key: dnd2024:creatures/giant_hyena
splat_name: Giant Hyena
splat_origin:
  key: dnd2024:reference/creature/giant_hyena
splat_sources:
- title: SRD 5.2.1
  locator: pp. 352
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 50 ft.
  skills_text: Perception +3
  senses_text: Darkvision 60 ft.; Passive Perception 13
  languages_text: None
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 45 (6d10 + 12)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 14
  save_dexterity:
    value: 2
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
    value: 12
  max_hp:
    value: 45
  initiative:
    value: 2
  speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 6d79c67fea4b11b48a7a4d7a7d2575cc74e186f222bc99f8ad48323baf3e02a0
  source_path: builtin/srd-5.2.1/creature/giant_hyena.md
  pages:
  - 352
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Giant Hyena Large Beast, Unaligned AC 12 Initiative +2 (12) HP 45 (6d10 + 12) Speed 50 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 14 +2 +2 Con 14 +2 +2 Int 2 -4 -4 Wis 12 +1 +1 Cha 7 -2
    -2 Skills Perception +3 Senses Darkvision 60 ft.; Passive Perception 13 Languages None CR 1 (XP 200;
    PB +2) ### Actions Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Piercing damage. ###
    Bonus Actions Rampage (1/Day). Immediately after dealing damage to a creature that was already Bloodied,
    the hyena can move up to half its Speed, and it makes one Bite attack.'
  sha256: bbca21909edd55ed8303d2e9fcce5134e42c4049eb285057593df552af1acaf3
  status: needs_semantic_conversion
---

# Giant Hyena

<!-- SRD page 352 -->

Large Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 45 (6d10 + 12)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 14 +2 +2 Con 14 +2 +2\
Int 2 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
Skills Perception +3\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 1 (XP 200; PB +2)\

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3)\
Piercing damage.\

### Bonus Actions

Rampage (1/Day). Immediately after dealing damage\
to a creature that was already Bloodied, the hyena can\
move up to half its Speed, and it makes one Bite attack.\

Original layout: [[../../srd-5.2.1/pages/352|page 352]].
