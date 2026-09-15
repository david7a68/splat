---
splat_schema: 1
splat_id: 273b9cd7-f749-5b98-a216-65592f4c1360
splat_kind: creature
splat_key: dnd2024:creatures/ghast
splat_name: Ghast
splat_origin:
  key: dnd2024:reference/creature/ghast
splat_sources:
- title: SRD 5.2.1
  locator: pp. 287
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: undead
  alignment: Chaotic Evil
  movement_text: 30 ft.
  resistances_text: Necrotic
  immunities_text: Poison; Charmed, Exhaustion, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Common
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 36 (8d8)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 17
  save_dexterity:
    value: 3
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 11
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 2
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 13
  max_hp:
    value: 36
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 6aa15f013f66af5305a209ed6ad3ca25cb4de569451b5ae746e16a9157b04fa7
  source_path: builtin/srd-5.2.1/creature/ghast.md
  pages:
  - 287
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Ghast Medium Undead, Chaotic Evil AC 13 Initiative +3 (13) HP 36 (8d8) Speed 30 ft. MOD SAVE
    MOD SAVE MOD SAVE Str 16 +3 +3 Dex 17 +3 +3 Con 10 +0 +0 Int 11 +0 +0 Wis 10 +0 +2 Cha 8 -1 -1 Resistances
    Necrotic Immunities Poison; Charmed, Exhaustion, Poisoned Senses Darkvision 60 ft.; Passive Perception
    10 Languages Common CR 2 (XP 450; PB +2) ### Traits Stench. Constitution Saving Throw: DC 10, any
    creature that starts its turn in a 5-foot Emanation originating from the ghast. Failure: The target
    has the Poisoned condition until the start of its next turn. Success: The target is immune to this
    ghast’s Stench for 24 hours. ### Actions Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3)
    Piercing damage plus 9 (2d8) Necrotic damage. Claw. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6
    + 3) Slashing damage. If the target is a non-Undead creature, it is subjected to the following effect.
    Constitution Saving Throw: DC 10. Failure: The target has the Paralyzed condition until the end of
    its next turn.'
  sha256: 8b77bbe80fe209aea45dda17c1eb4d6fb98f05fcf43e670f2a1ae3ee136ff5a7
  status: needs_semantic_conversion
---

# Ghast

<!-- SRD page 287 -->

Medium Undead, Chaotic Evil\
AC 13 Initiative +3 (13)\
HP 36 (8d8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 17 +3 +3 Con 10 +0 +0\
Int 11 +0 +0 Wis 10 +0 +2 Cha 8 −1 −1\
Resistances Necrotic\
Immunities Poison; Charmed, Exhaustion, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Common\
CR 2 (XP 450; PB +2)\

### Traits

Stench. Constitution Saving Throw: DC 10, any creature\
that starts its turn in a 5-foot Emanation originating from\
the ghast. Failure: The target has the Poisoned condi-\
tion until the start of its next turn. Success: The target is\
immune to this ghast’s Stench for 24 hours.\

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3)\
Piercing damage plus 9 (2d8) Necrotic damage.\
Claw. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6\
+ 3) Slashing damage. If the target is a non-Undead\
creature, it is subjected to the following effect. Consti-\
tution Saving Throw: DC 10. Failure: The target has the\
Paralyzed condition until the end of its next turn.\

Original layout: [[../../srd-5.2.1/pages/287|page 287]].
