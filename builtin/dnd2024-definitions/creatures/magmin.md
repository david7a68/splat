---
splat_schema: 1
splat_id: 70753f51-9152-503e-972c-dff51c4d5701
splat_kind: creature
splat_key: dnd2024:creatures/magmin
splat_name: Magmin
splat_origin:
  key: dnd2024:reference/creature/magmin
splat_sources:
- title: SRD 5.2.1
  locator: pp. 305, 306
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: elemental
  alignment: Chaotic Neutral
  movement_text: 30 ft.
  immunities_text: Fire
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Primordial (Ignan)
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 13 (3d6 + 3)
splat_stats:
  strength:
    value: 7
  save_strength:
    value: -2
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 8
  save_intelligence:
    value: -1
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 14
  max_hp:
    value: 13
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: e76e9d499f5ea2b66339c5ec48847f48093934f5766e769a3d87dcd813aa92e7
  source_path: builtin/srd-5.2.1/creature/magmin.md
  pages:
  - 305
  - 306
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Magmin Small Elemental, Chaotic Neutral AC 14 Initiative +2 (12) HP 13 (3d6 + 3) Speed 30 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 7 -2 -2 Dex 15 +2 +2 Con 12 +1 +1 Int 8 -1 -1 Wis 11 +0 +0 Cha 10 +0
    +0 Immunities Fire Senses Darkvision 60 ft.; Passive Perception 10 Languages Primordial (Ignan) CR
    1/2 (XP 100; PB +2) ### Traits Death Burst. The magmin explodes when it dies. Dexterity Saving Throw:
    DC 11, each creature in a 10-foot Emanation originating from the magmin. Failure: 7 (2d6) Fire damage.
    Success: Half damage. ### Actions Touch. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (2d4 + 2) Fire
    damage. If the target is a creature or a flammable object that isn’t being worn or carried, it starts
    burning. ### Bonus Actions Ignited Illumination. The magmin sets itself ablaze or extinguishes its
    flames. While ablaze, the magmin sheds Bright Light in a 10-foot radius and Dim Light for an additional
    10 feet.'
  sha256: d7dbfa0f02791894744183acb706b88eaec8677d3150b860abde839db5824a69
  status: needs_semantic_conversion
---

# Magmin

<!-- SRD page 305 -->

Small Elemental, Chaotic Neutral\
AC 14 Initiative +2 (12)\
HP 13 (3d6 + 3)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 7 −2 −2 Dex 15 +2 +2 Con 12 +1 +1\
Int 8 −1 −1 Wis 11 +0 +0 Cha 10 +0 +0\
Immunities Fire\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Primordial (Ignan)\
CR 1/2 (XP 100; PB +2)\

### Traits

Death Burst. The magmin explodes when it dies. Dex-\
terity Saving Throw: DC 11, each creature in a 10-foot\
Emanation originating from the magmin. Failure: 7 (2d6)\
Fire damage. Success: Half damage.\

### Actions

Touch. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (2d4 +\
2) Fire damage. If the target is a creature or a flammable\
object that isn’t being worn or carried, it starts burning.\

<!-- SRD page 306 -->


### Bonus Actions

Ignited Illumination. The magmin sets itself ablaze\
or extinguishes its flames. While ablaze, the magmin\
sheds Bright Light in a 10-foot radius and Dim Light for\
an additional 10 feet.\

Original layout: [[../../srd-5.2.1/pages/305|page 305]], [[../../srd-5.2.1/pages/306|page 306]].
