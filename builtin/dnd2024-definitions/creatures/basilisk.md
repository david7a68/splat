---
splat_schema: 1
splat_id: 1b37be75-c5c9-5acc-9039-e032862c647f
splat_kind: creature
splat_key: dnd2024:creatures/basilisk
splat_name: Basilisk
splat_origin:
  key: dnd2024:reference/creature/basilisk
splat_sources:
- title: SRD 5.2.1
  locator: pp. 262
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: monstrosity
  alignment: Unaligned
  movement_text: 20 ft.
  senses_text: Darkvision 60 ft.; Passive Perception 9
  languages_text: None
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 52 (8d8 + 16)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 8
  save_wisdom:
    value: -1
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 15
  max_hp:
    value: 52
  initiative:
    value: -1
  speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 8a5782180bc9c45eceedab36e357a3762ebedae0b0585af2b8c81e8f36dc6ae4
  source_path: builtin/srd-5.2.1/creature/basilisk.md
  pages:
  - 262
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Basilisk Medium Monstrosity, Unaligned AC 15 Initiative -1 (9) HP 52 (8d8 + 16) Speed 20 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 8 -1 -1 Con 15 +2 +2 Int 2 -4 -4 Wis 8 -1 -1 Cha 7 -2
    -2 Senses Darkvision 60 ft.; Passive Perception 9 Languages None CR 3 (XP 700; PB +2) ### Actions
    Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Piercing damage plus 7 (2d6) Poison damage.
    ### Bonus Actions Petrifying Gaze (Recharge 4–6). Constitution Saving Throw: DC 12, each creature
    in a 30-foot Cone. If the basilisk sees its reflection in the Cone, the basilisk must make this save.
    First Failure: The target has the Restrained condition and repeats the save at the end of its next
    turn if it is still Restrained, ending the effect on itself on a success. Second Failure: The target
    has the Petrified condition instead of the Restrained condition.'
  sha256: a50df939a8fcec315b461f5d6a5b8f04cb95c122a10d99a56c2016566e2ae5f0
  status: needs_semantic_conversion
---

# Basilisk

<!-- SRD page 262 -->

Medium Monstrosity, Unaligned\
AC 15 Initiative −1 (9)\
HP 52 (8d8 + 16)\
Speed 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 8 −1 −1 Con 15 +2 +2\
Int 2 −4 −4 Wis 8 −1 −1 Cha 7 −2 −2\
Senses Darkvision 60 ft.; Passive Perception 9\
Languages None\
CR 3 (XP 700; PB +2)\

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3)\
Piercing damage plus 7 (2d6) Poison damage.\

### Bonus Actions

Petrifying Gaze (Recharge 4–6). Constitution Saving\
Throw: DC 12, each creature in a 30-foot Cone. If\
the basilisk sees its reflection in the Cone, the basilisk\
must make this save. First Failure: The target has the\
Restrained condition and repeats the save at the end of\
its next turn if it is still Restrained, ending the effect on\
itself on a success. Second Failure: The target has the\
Petrified condition instead of the Restrained condition.\

Original layout: [[../../srd-5.2.1/pages/262|page 262]].
