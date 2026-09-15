---
splat_schema: 1
splat_id: 7c7d3bae-a1a3-5189-a524-3966385eced8
splat_kind: creature
splat_key: dnd2024:creatures/shambling_mound
splat_name: Shambling Mound
splat_origin:
  key: dnd2024:reference/creature/shambling_mound
splat_sources:
- title: SRD 5.2.1
  locator: pp. 323
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: plant
  alignment: Unaligned
  movement_text: 30 ft., Swim 20 ft.
  skills_text: Stealth +3
  resistances_text: Cold, Fire
  immunities_text: Lightning; Deafened, Exhaustion
  senses_text: Blindsight 60 ft.; Passive Perception 10
  languages_text: None
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 110 (13d10 + 39)
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
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 5
  save_intelligence:
    value: -3
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 15
  max_hp:
    value: 110
  initiative:
    value: -1
  speed:
    value: 30
    unit: ft
  swim_speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 74a6f58a989cb367c71316b812a517d4a5adcacf392abef7061ecb4c93d08b7b
  source_path: builtin/srd-5.2.1/creature/shambling_mound.md
  pages:
  - 323
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Shambling Mound Large Plant, Unaligned AC 15 Initiative -1 (9) HP 110 (13d10 + 39) Speed 30
    ft., Swim 20 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 8 -1 -1 Con 16 +3 +3 Int 5 -3 -3 Wis
    10 +0 +0 Cha 5 -3 -3 Skills Stealth +3 Resistances Cold, Fire Immunities Lightning; Deafened, Exhaustion
    Senses Blindsight 60 ft.; Passive Perception 10 Languages None CR 5 (XP 1,800; PB +3) ### Traits Lightning
    Absorption. Whenever the shambling mound is subjected to Lightning damage, it regains a number of
    Hit Points equal to the Lightning damage dealt. ### Actions Multiattack. The shambling mound makes
    three Charged Tendril attacks. It can replace one attack with a use of Engulf. Charged Tendril. Melee
    Attack Roll: +7, reach 10 ft. Hit: 7 (1d6 + 4) Bludgeoning damage plus 5 (2d4) Lightning damage. If
    the target is a Medium or smaller creature, the shambling mound pulls the target 5 feet straight toward
    itself. Engulf. Strength Saving Throw: DC 15, one Medium or smaller creature within 5 feet. Failure:
    The target is pulled into the shambling mound’s space and has the Grappled condition (escape DC 14).
    Until the grapple ends, the target has the Blinded and Restrained conditions, and it takes 10 (3d6)
    Lightning damage at the start of each of its turns. When the shambling mound moves, the Grappled target
    moves with it, costing it no extra movement. The shambling mound can have only one creature Grappled
    by this action at a time.'
  sha256: a871cb87837967b5a34c9a7bc9395b0ff09a80082aff507cc6e5b64b7110e500
  status: needs_semantic_conversion
---

# Shambling Mound

<!-- SRD page 323 -->

Large Plant, Unaligned\
AC 15  Initiative −1 (9)\
HP 110 (13d10 + 39)\
Speed 30 ft., Swim 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 8 −1 −1 Con 16 +3 +3\
Int 5 −3 −3 Wis 10 +0 +0 Cha 5 −3 −3\
Skills Stealth +3\
Resistances Cold, Fire\
Immunities Lightning; Deafened, Exhaustion\
Senses Blindsight 60 ft.; Passive Perception 10\
Languages None\
CR 5 (XP 1,800; PB +3)\

### Traits

Lightning Absorption. Whenever the shambling mound\
is subjected to Lightning damage, it regains a number of\
Hit Points equal to the Lightning damage dealt.\

### Actions

Multiattack. The shambling mound makes three\
Charged Tendril attacks. It can replace one attack with\
a use of Engulf.\
Charged Tendril. Melee Attack Roll: +7, reach 10 ft.\
Hit: 7 (1d6 + 4) Bludgeoning damage plus 5 (2d4)\
Lightning damage. If the target is a Medium or smaller\
creature, the shambling mound pulls the target 5 feet\
straight toward itself.\
Engulf. Strength Saving Throw: DC 15, one Medium\
or smaller creature within 5 feet. Failure: The target is\
pulled into the shambling mound’s space and has the\
Grappled condition (escape DC 14). Until the grapple\
ends, the target has the Blinded and Restrained con-\
ditions, and it takes 10 (3d6) Lightning damage at the\
start of each of its turns. When the shambling mound\
moves, the Grappled target moves with it, costing it no\
extra movement. The shambling mound can have only\
one creature Grappled by this action at a time.\

Original layout: [[../../srd-5.2.1/pages/323|page 323]].
