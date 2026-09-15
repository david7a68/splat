---
splat_schema: 1
splat_id: aad2d730-166b-50f4-acd0-c355c6b3b9e5
splat_kind: creature
splat_key: dnd2024:creatures/roper
splat_name: Roper
splat_origin:
  key: dnd2024:reference/creature/roper
splat_sources:
- title: SRD 5.2.1
  locator: pp. 320
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: aberration
  alignment: Neutral Evil
  movement_text: 10 ft., Climb 20 ft.
  skills_text: Perception +6, Stealth +5
  senses_text: Darkvision 60 ft.; Passive Perception 16
  languages_text: None
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 93 (11d10 + 33)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 7
  save_intelligence:
    value: -2
  wisdom:
    value: 16
  save_wisdom:
    value: 3
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 20
  max_hp:
    value: 93
  initiative:
    value: 5
  speed:
    value: 10
    unit: ft
  climb_speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 88c8b85a1319a8f6b0e75fc6b92c5667ed51145639ee988bc8ed413c5b4fc616
  source_path: builtin/srd-5.2.1/creature/roper.md
  pages:
  - 320
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Roper Large Aberration, Neutral Evil AC 20 Initiative +5 (15) HP 93 (11d10 + 33) Speed 10 ft.,
    Climb 20 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 8 -1 -1 Con 17 +3 +3 Int 7 -2 -2 Wis 16 +3
    +3 Cha 6 -2 -2 Skills Perception +6, Stealth +5 Senses Darkvision 60 ft.; Passive Perception 16 Languages
    None CR 5 (XP 1,800; PB +3) ### Traits Spider Climb. The roper can climb difficult surfaces, including
    along ceilings, without needing to make an ability check. ### Actions Multiattack. The roper makes
    two Tentacle attacks, uses Reel, and makes two Bite attacks. Bite. Melee Attack Roll: +7, reach 5
    ft. Hit: 17 (3d8 + 4) Piercing damage. Tentacle. Melee Attack Roll: +7, reach 60 ft. Hit: The target
    has the Grappled condition (escape DC 14) from one of six tentacles, and the target has the Poisoned
    condition until the grapple ends. The tentacle can be damaged, freeing a creature it has Grappled
    when destroyed (AC 20, HP 10, Immunity to Poison and Psychic damage). Damaging the tentacle deals
    no damage to the roper, and a destroyed tentacle regrows at the start of the roper’s next turn. Reel.
    The roper pulls each creature Grappled by it up to 30 feet straight toward it.'
  sha256: 1c9014364052f94c08279d1d5794cb40b1b57f95d1bc026c9bf354f26e5561c2
  status: needs_semantic_conversion
---

# Roper

<!-- SRD page 320 -->

Large Aberration, Neutral Evil\
AC 20  Initiative +5 (15)\
HP 93 (11d10 + 33)\
Speed 10 ft., Climb 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 8 −1 −1 Con 17 +3 +3\
Int 7 −2 −2 Wis 16 +3 +3 Cha 6 −2 −2\
Skills Perception +6, Stealth +5\
Senses Darkvision 60 ft.; Passive Perception 16\
Languages None\
CR 5 (XP 1,800; PB +3)\

### Traits

Spider Climb. The roper can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\

### Actions

Multiattack. The roper makes two Tentacle attacks,\
uses Reel, and makes two Bite attacks.\
Bite. Melee Attack Roll: +7, reach 5 ft. Hit: 17 (3d8 + 4)\
Piercing damage.\
Tentacle. Melee Attack Roll: +7, reach 60 ft. Hit: The\
target has the Grappled condition (escape DC 14) from\
one of six tentacles, and the target has the Poisoned\
condition until the grapple ends.\
The tentacle can be damaged, freeing a creature it has\
Grappled when destroyed (AC 20, HP 10, Immunity to\
Poison and Psychic damage). Damaging the tentacle\
deals no damage to the roper, and a destroyed tentacle\
regrows at the start of the roper’s next turn.\
Reel. The roper pulls each creature Grappled by it up\
to 30 feet straight toward it.\

Original layout: [[../../srd-5.2.1/pages/320|page 320]].
