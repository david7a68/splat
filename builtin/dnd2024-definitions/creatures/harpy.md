---
splat_schema: 1
splat_id: 3ddc1ac6-e9a6-55e1-86b8-acebc64ab05e
splat_kind: creature
splat_key: dnd2024:creatures/harpy
splat_name: Harpy
splat_origin:
  key: dnd2024:reference/creature/harpy
splat_sources:
- title: SRD 5.2.1
  locator: pp. 297
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: monstrosity
  alignment: Chaotic Evil
  movement_text: 20 ft., Fly 40 ft.
  senses_text: Passive Perception 10
  languages_text: Common
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 38 (7d8 + 7)
splat_stats:
  strength:
    value: 12
  save_strength:
    value: 1
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 7
  save_intelligence:
    value: -2
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 13
  save_charisma:
    value: 1
  armor_class:
    value: 11
  max_hp:
    value: 38
  initiative:
    value: 1
  speed:
    value: 20
    unit: ft
  fly_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: dbee13ac9f0e5de071a14741412bb5b46a0057d1104287ec7b9ea8b9e38aec90
  source_path: builtin/srd-5.2.1/creature/harpy.md
  pages:
  - 297
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Harpy Medium Monstrosity, Chaotic Evil AC 11 Initiative +1 (11) HP 38 (7d8 + 7) Speed 20 ft.,
    Fly 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 12 +1 +1 Dex 13 +1 +1 Con 12 +1 +1 Int 7 -2 -2 Wis 10 +0
    +0 Cha 13 +1 +1 Senses Passive Perception 10 Languages Common CR 1 (XP 200; PB +2) ### Actions Claw.
    Melee Attack Roll: +3, reach 5 ft. Hit: 6 (2d4 + 1) Slashing damage. Luring Song. The harpy sings
    a magical melody, which lasts until the harpy’s Concentration ends on it. Wisdom Saving Throw: DC
    11, each Humanoid and Giant in a 300-foot Emanation originating from the harpy when the song starts.
    Failure: The target has the Charmed condition until the song ends and repeats the save at the end
    of each of its turns. While Charmed, the target has the Incapacitated condition and ignores the Luring
    Song of other harpies. If the target is more than 5 feet from the harpy, the target moves on its turn
    toward the harpy by the most direct route, trying to get within 5 feet of the harpy. It doesn’t avoid
    Opportunity Attacks; however, before moving into damaging terrain (such as lava or a pit) and whenever
    it takes damage from a source other than the harpy, the target repeats the save. Success: The target
    is immune to this harpy’s Luring Song for 24 hours.'
  sha256: 99b75f5e99b7df791f134cf3f7c97b0f0d038a4d0da72701db982b749b0e8096
  status: needs_semantic_conversion
---

# Harpy

<!-- SRD page 297 -->

Medium Monstrosity, Chaotic Evil\
AC 11 Initiative +1 (11)\
HP 38 (7d8 + 7)\
Speed 20 ft., Fly 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 12 +1 +1 Dex 13 +1 +1 Con 12 +1 +1\
Int 7 −2 −2 Wis 10 +0 +0 Cha 13 +1 +1\
Senses Passive Perception 10\
Languages Common\
CR 1 (XP 200; PB +2)\

### Actions

Claw. Melee Attack Roll: +3, reach 5 ft. Hit: 6 (2d4 + 1)\
Slashing damage.\
Luring Song. The harpy sings a magical melody, which\
lasts until the harpy’s Concentration ends on it. Wisdom\
Saving Throw: DC 11, each Humanoid and Giant in a\
300-foot Emanation originating from the harpy when\
the song starts. Failure: The target has the Charmed\
condition until the song ends and repeats the save at\
the end of each of its turns. While Charmed, the target\
has the Incapacitated condition and ignores the Luring\
Song of other harpies. If the target is more than 5 feet\
from the harpy, the target moves on its turn toward the\
harpy by the most direct route, trying to get within 5\
feet of the harpy. It doesn’t avoid Opportunity Attacks;\
however, before moving into damaging terrain (such\
as lava or a pit) and whenever it takes damage from a\
source other than the harpy, the target repeats the save.\
Success: The target is immune to this harpy’s Luring\
Song for 24 hours.\

Original layout: [[../../srd-5.2.1/pages/297|page 297]].
