---
splat_schema: 1
splat_id: d00bd6b3-23c5-5121-81d2-def8a283e438
splat_kind: creature
splat_key: dnd2024:creatures/giant_spider
splat_name: Giant Spider
splat_origin:
  key: dnd2024:reference/creature/giant_spider
splat_sources:
- title: SRD 5.2.1
  locator: pp. 353, 354
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Climb 30 ft.
  skills_text: Perception +4, Stealth +7
  senses_text: Darkvision 60 ft.; Passive Perception 14
  languages_text: None
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 26 (4d10 + 4)
splat_stats:
  strength:
    value: 14
  save_strength:
    value: 2
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 4
  save_charisma:
    value: -3
  armor_class:
    value: 14
  max_hp:
    value: 26
  initiative:
    value: 3
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
  source_sha256: b306725f61da9def8dca3cf4f3ef480bfbd27bc3677991b0f1e02b668406aacb
  source_path: builtin/srd-5.2.1/creature/giant_spider.md
  pages:
  - 353
  - 354
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Giant Spider Large Beast, Unaligned AC 14 Initiative +3 (13) HP 26 (4d10 + 4) Speed 30 ft.,
    Climb 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 14 +2 +2 Dex 16 +3 +3 Con 12 +1 +1 Int 2 -4 -4 Wis 11
    +0 +0 Cha 4 -3 -3 Skills Perception +4, Stealth +7 Senses Darkvision 60 ft.; Passive Perception 14
    Languages None CR 1 (XP 200; PB +2) ### Traits Spider Climb. The spider can climb difficult surfaces,
    including along ceilings, without needing to make an ability check. Web Walker. The spider ignores
    movement restrictions caused by webs, and it knows the location of any other creature in contact with
    the same web. ### Actions Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Piercing damage
    plus 7 (2d6) Poison damage. Web (Recharge 5–6). Dexterity Saving Throw: DC 13, one creature the spider
    can see within 60 feet. Failure: The target has the Restrained condition until the web is destroyed
    (AC 10; HP 5; Vulnerability to Fire damage; Immunity to Poison and Psychic damage).'
  sha256: a6c981f1e67aa74b0fa00997f568f000b3205ad5cb941e90e04e7df88c040556
  status: needs_semantic_conversion
---

# Giant Spider

<!-- SRD page 353 -->

Large Beast, Unaligned\
AC 14  Initiative +3 (13)\
HP 26 (4d10 + 4)\
Speed 30 ft., Climb 30 ft.\

<!-- SRD page 354 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 16 +3 +3 Con 12 +1 +1\
Int 2 −4 −4 Wis 11 +0 +0 Cha 4 −3 −3\
Skills Perception +4, Stealth +7\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages None\
CR 1 (XP 200; PB +2)\

### Traits

Spider Climb. The spider can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\
Web Walker. The spider ignores movement restrictions\
caused by webs, and it knows the location of any other\
creature in contact with the same web.\

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3)\
Piercing damage plus 7 (2d6) Poison damage.\
Web (Recharge 5–6). Dexterity Saving Throw: DC 13,\
one creature the spider can see within 60 feet. Failure:\
The target has the Restrained condition until the web is\
destroyed (AC 10; HP 5; Vulnerability to Fire damage;\
Immunity to Poison and Psychic damage).\

Original layout: [[../../srd-5.2.1/pages/353|page 353]], [[../../srd-5.2.1/pages/354|page 354]].
