---
splat_schema: 1
splat_id: a022e1fe-c390-5c20-90fe-93dfddfb57be
splat_kind: creature
splat_key: dnd2024:creatures/gorgon
splat_name: Gorgon
splat_origin:
  key: dnd2024:reference/creature/gorgon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 292, 293
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: construct
  alignment: Unaligned
  movement_text: 40 ft.
  skills_text: Perception +7
  immunities_text: Exhaustion, Petrified
  senses_text: Darkvision 60 ft.; Passive Perception 17
  languages_text: None
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 114 (12d10 + 48)
splat_stats:
  strength:
    value: 20
  save_strength:
    value: 5
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 18
  save_constitution:
    value: 4
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
    value: 19
  max_hp:
    value: 114
  initiative:
    value: 0
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: dadb6e45b4ecf8a24851a9fff8e04da72f9114c9fda5240a79669efcf795fa4b
  source_path: builtin/srd-5.2.1/creature/gorgon.md
  pages:
  - 292
  - 293
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Gorgon Large Construct, Unaligned AC 19 Initiative +0 (10) HP 114 (12d10 + 48) Speed 40 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 20 +5 +5 Dex 11 +0 +0 Con 18 +4 +4 Int 2 -4 -4 Wis 12 +1 +1 Cha 7 -2
    -2 Skills Perception +7 Immunities Exhaustion, Petrified Senses Darkvision 60 ft.; Passive Perception
    17 Languages None CR 5 (XP 1,800; PB +3) ### Actions Gore. Melee Attack Roll: +8, reach 5 ft. Hit:
    18 (2d12 + 5) Piercing damage. If the target is a Large or smaller creature and the gorgon moved 20+
    feet straight toward it immediately before the hit, the target has the Prone condition. Petrifying
    Breath (Recharge 5–6). Constitution Saving Throw: DC 15, each creature in a 30-foot Cone. First Failure:
    The target has the Restrained condition and repeats the save at the end of its next turn if it is
    still Restrained, ending the effect on itself on a success. Second Failure: The target has the Petrified
    condition instead of the Restrained condition. ### Bonus Actions Trample. Dexterity Saving Throw:
    DC 16, one creature within 5 feet that has the Prone condition. Failure: 16 (2d10 + 5) Bludgeoning
    damage. Success: Half damage.'
  sha256: e9cc0cf72705d231c61b29ed763bb96e2e51475e8ff75bebbf31005374e7fee7
  status: needs_semantic_conversion
---

# Gorgon

<!-- SRD page 292 -->

Large Construct, Unaligned\
AC 19 Initiative +0 (10)\
HP 114 (12d10 + 48)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 20 +5 +5 Dex 11 +0 +0 Con 18 +4 +4\
Int 2 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
Skills Perception +7\
Immunities Exhaustion, Petrified\
Senses Darkvision 60 ft.; Passive Perception 17\
Languages None\
CR 5 (XP 1,800; PB +3)\

<!-- SRD page 293 -->


### Actions

Gore. Melee Attack Roll: +8, reach 5 ft. Hit: 18 (2d12\
+ 5) Piercing damage. If the target is a Large or smaller\
creature and the gorgon moved 20+ feet straight toward\
it immediately before the hit, the target has the Prone\
condition.\
Petrifying Breath (Recharge 5–6). Constitution Saving\
Throw: DC 15, each creature in a 30-foot Cone. First\
Failure: The target has the Restrained condition and\
repeats the save at the end of its next turn if it is still\
Restrained, ending the effect on itself on a success.\
Second Failure: The target has the Petrified condition\
instead of the Restrained condition.\

### Bonus Actions

Trample. Dexterity Saving Throw: DC 16, one creature\
within 5 feet that has the Prone condition. Failure: 16\
(2d10 + 5) Bludgeoning damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/292|page 292]], [[../../srd-5.2.1/pages/293|page 293]].
