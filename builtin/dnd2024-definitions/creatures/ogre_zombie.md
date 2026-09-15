---
splat_schema: 1
splat_id: 7a6ad303-6841-5a1d-ae28-fe80b260ced7
splat_kind: creature
splat_key: dnd2024:creatures/ogre_zombie
splat_name: Ogre Zombie
splat_origin:
  key: dnd2024:reference/creature/ogre_zombie
splat_sources:
- title: SRD 5.2.1
  locator: pp. 344
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: undead
  alignment: Neutral Evil
  movement_text: 30 ft.
  immunities_text: Poison; Exhaustion, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 8
  languages_text: Understands Common and Giant but can’t speak
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 85 (9d10 + 36)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 6
  save_dexterity:
    value: -2
  constitution:
    value: 18
  save_constitution:
    value: 4
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 6
  save_wisdom:
    value: 0
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 8
  max_hp:
    value: 85
  initiative:
    value: -2
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: ec9b43610997b5a79b2a6c977b1b5df4e59627ccea8780e8d5e9b9772f8e7ac8
  source_path: builtin/srd-5.2.1/creature/ogre_zombie.md
  pages:
  - 344
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Ogre Zombie Large Undead, Neutral Evil AC 8 Initiative -2 (8) HP 85 (9d10 + 36) Speed 30 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 6 -2 -2 Con 18 +4 +4 Int 3 -4 -4 Wis 6 -2 +0 Cha 5 -3
    -3 Immunities Poison; Exhaustion, Poisoned Senses Darkvision 60 ft.; Passive Perception 8 Languages
    Understands Common and Giant but can’t speak CR 2 (XP 450; PB +2) ### Traits Undead Fortitude. If
    damage reduces the zombie to 0 Hit Points, it makes a Constitution saving throw (DC 5 plus the damage
    taken) unless the damage is Radiant or from a Critical Hit. On a successful save, the zombie drops
    to 1 Hit Point instead. ### Actions Slam. Melee Attack Roll: +6, reach 5 ft. Hit: 13 (2d8 + 4) Bludgeoning
    damage.'
  sha256: 9f93426545c6e72deb4ff2491681d5f0e954a533b2c69bf62e78bec2a9cc5d1a
  status: needs_semantic_conversion
---

# Ogre Zombie

<!-- SRD page 344 -->

Large Undead, Neutral Evil\
AC 8 Initiative −2 (8)\
HP 85 (9d10 + 36)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 6 −2 −2 Con 18 +4 +4\
Int 3 −4 −4 Wis 6 −2 +0 Cha 5 −3 −3\
Immunities Poison; Exhaustion, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 8\
Languages Understands Common and Giant but can’t\
speak\
CR 2 (XP 450; PB +2)\

### Traits

Undead Fortitude. If damage reduces the zombie to 0\
Hit Points, it makes a Constitution saving throw (DC 5\
plus the damage taken) unless the damage is Radiant or\
from a Critical Hit. On a successful save, the zombie\
drops to 1 Hit Point instead.\

### Actions

Slam. Melee Attack Roll: +6, reach 5 ft. Hit: 13 (2d8\
+ 4) Bludgeoning damage.\

Original layout: [[../../srd-5.2.1/pages/344|page 344]].
