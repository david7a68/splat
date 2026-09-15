---
splat_schema: 1
splat_id: 67b895f2-bf1b-5105-b40c-2ed954a893da
splat_kind: creature
splat_key: dnd2024:creatures/gelatinous_cube
splat_name: Gelatinous Cube
splat_origin:
  key: dnd2024:reference/creature/gelatinous_cube
splat_sources:
- title: SRD 5.2.1
  locator: pp. 286, 287
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: ooze
  alignment: Unaligned
  movement_text: 15 ft.
  immunities_text: Acid; Blinded, Charmed, Deafened, Exhaustion, Frightened, Prone
  senses_text: Blindsight 60 ft.; Passive Perception 8
  languages_text: None
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 63 (6d10 + 30)
splat_stats:
  strength:
    value: 14
  save_strength:
    value: 2
  dexterity:
    value: 3
  save_dexterity:
    value: -4
  constitution:
    value: 20
  save_constitution:
    value: 5
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 6
  save_wisdom:
    value: -2
  charisma:
    value: 1
  save_charisma:
    value: -5
  armor_class:
    value: 6
  max_hp:
    value: 63
  initiative:
    value: -4
  speed:
    value: 15
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 6bc71f608e84bce08024eab82f6dca66ad4a676ebc11b5e7d940ce2ebd82d14d
  source_path: builtin/srd-5.2.1/creature/gelatinous_cube.md
  pages:
  - 286
  - 287
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Gelatinous Cube Large Ooze, Unaligned AC 6 Initiative -4 (6) HP 63 (6d10 + 30) Speed 15 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 14 +2 +2 Dex 3 -4 -4 Con 20 +5 +5 Int 1 -5 -5 Wis 6 -2 -2 Cha 1 -5
    -5 Immunities Acid; Blinded, Charmed, Deafened, Exhaustion, Frightened, Prone Senses Blindsight 60
    ft.; Passive Perception 8 Languages None CR 2 (XP 450; PB +2) ### Traits Ooze Cube. The cube fills
    its entire space and is transparent. Other creatures can enter that space, but a creature that does
    so is subjected to the cube’s Engulf and has Disadvantage on the saving throw. Creatures inside the
    cube have Total Cover, and the cube can hold one Large creature or up to four Medium or Small creatures
    inside itself at a time. As an action, a creature within 5 feet of the cube can pull a creature or
    an object out of the cube by succeeding on a DC 12 Strength (Athletics) check, and the puller takes
    10 (3d6) Acid damage. Transparent. Even when the cube is in plain sight, a creature must succeed on
    a DC 15 Wisdom (Perception) check to notice the cube if the creature hasn’t witnessed the cube move
    or otherwise act. ### Actions Pseudopod. Melee Attack Roll: +4, reach 5 ft. Hit: 12 (3d6 + 2) Acid
    damage. Engulf. The cube moves up to its Speed without provoking Opportunity Attacks. The cube can
    move through the spaces of Large or smaller creatures if it has room inside itself to contain them
    (see the Ooze Cube trait). Dexterity Saving Throw: DC 12, each creature whose space the cube enters
    for the first time during this move. Failure: 10 (3d6) Acid damage, and the target is engulfed. An
    engulfed target is suffocating, can’t cast spells with a Verbal component, has the Restrained condition,
    and takes 10 (3d6) Acid damage at the start of each of the cube’s turns. When the cube moves, the
    engulfed target moves with it. An engulfed target can try to escape by taking an action to make a
    DC 12 Strength (Athletics) check. On a successful check, the target escapes and enters the nearest
    unoccupied space. Success: Half damage, and the target moves to an unoccupied space within 5 feet
    of the cube. If there is no unoccupied space, the target fails the save instead.'
  sha256: fbddad9661dd577bc58a2aa705c46eee9a716de73e59eb1dbde9be672e3d11e9
  status: needs_semantic_conversion
---

# Gelatinous Cube

<!-- SRD page 286 -->

Large Ooze, Unaligned\
AC 6 Initiative −4 (6)\
HP 63 (6d10 + 30)\
Speed 15 ft.\

<!-- SRD page 287 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 3 −4 −4 Con 20 +5 +5\
Int 1 −5 −5 Wis 6 −2 −2 Cha 1 −5 −5\
Immunities Acid; Blinded, Charmed, Deafened,\
Exhaustion, Frightened, Prone\
Senses Blindsight 60 ft.; Passive Perception 8\
Languages None\
CR 2 (XP 450; PB +2)\

### Traits

Ooze Cube. The cube fills its entire space and is trans-\
parent. Other creatures can enter that space, but a\
creature that does so is subjected to the cube’s Engulf\
and has Disadvantage on the saving throw.\
Creatures inside the cube have Total Cover, and the\
cube can hold one Large creature or up to four Medium\
or Small creatures inside itself at a time.\
As an action, a creature within 5 feet of the cube can\
pull a creature or an object out of the cube by suc-\
ceeding on a DC 12 Strength (Athletics) check, and the\
puller takes 10 (3d6) Acid damage.\
Transparent. Even when the cube is in plain sight, a\
creature must succeed on a DC 15 Wisdom (Percep-\
tion) check to notice the cube if the creature hasn’t wit-\
nessed the cube move or otherwise act.\

### Actions

Pseudopod. Melee Attack Roll: +4, reach 5 ft. Hit: 12\
(3d6 + 2) Acid damage.\
Engulf. The cube moves up to its Speed without\
provoking Opportunity Attacks. The cube can move\
through the spaces of Large or smaller creatures if it has\
room inside itself to contain them (see the Ooze Cube\
trait). Dexterity Saving Throw: DC 12, each creature\
whose space the cube enters for the first time during\
this move. Failure: 10 (3d6) Acid damage, and the target\
is engulfed. An engulfed target is suffocating, can’t cast\
spells with a Verbal component, has the Restrained\
condition, and takes 10 (3d6) Acid damage at the start\
of each of the cube’s turns. When the cube moves,\
the engulfed target moves with it. An engulfed target\
can try to escape by taking an action to make a DC\
12 Strength (Athletics) check. On a successful check,\
the target escapes and enters the nearest unoccupied\
space. Success: Half damage, and the target moves to\
an unoccupied space within 5 feet of the cube. If there\
is no unoccupied space, the target fails the save instead.\

Original layout: [[../../srd-5.2.1/pages/286|page 286]], [[../../srd-5.2.1/pages/287|page 287]].
