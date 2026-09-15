---
splat_schema: 1
splat_id: 901a4cef-4135-5abc-adf5-810c1e85a6a8
splat_kind: creature
splat_key: dnd2024:creatures/ice_mephit
splat_name: Ice Mephit
splat_origin:
  key: dnd2024:reference/creature/ice_mephit
splat_sources:
- title: SRD 5.2.1
  locator: pp. 307
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: elemental
  alignment: Neutral Evil
  movement_text: 30 ft., Fly 30 ft.
  skills_text: Perception +2, Stealth +3
  vulnerabilities_text: Fire
  immunities_text: Cold, Poison; Exhaustion, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 12
  languages_text: Primordial (Aquan, Auran)
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 21 (6d6)
splat_stats:
  strength:
    value: 7
  save_strength:
    value: -2
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 9
  save_intelligence:
    value: -1
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 12
  save_charisma:
    value: 1
  armor_class:
    value: 11
  max_hp:
    value: 21
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 714af5ea3c19f777ed34bcff05f018c93e067f42ed70855677f94839fc8eaf8b
  source_path: builtin/srd-5.2.1/creature/ice_mephit.md
  pages:
  - 307
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Ice Mephit Small Elemental, Neutral Evil AC 11 Initiative +1 (11) HP 21 (6d6) Speed 30 ft.,
    Fly 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 7 -2 -2 Dex 13 +1 +1 Con 10 +0 +0 Int 9 -1 -1 Wis 11 +0
    +0 Cha 12 +1 +1 Skills Perception +2, Stealth +3 Vulnerabilities Fire Immunities Cold, Poison; Exhaustion,
    Poisoned Senses Darkvision 60 ft.; Passive Perception 12 Languages Primordial (Aquan, Auran) CR 1/2
    (XP 100; PB +2) ### Traits Death Burst. The mephit explodes when it dies. Constitution Saving Throw:
    DC 10, each creature in a 5-foot Emanation originating from the mephit. Failure: 5 (2d4) Cold damage.
    Success: Half damage. ### Actions Claw. Melee Attack Roll: +3, reach 5 ft. Hit: 3 (1d4 + 1) Slashing
    damage plus 2 (1d4) Cold damage. Fog Cloud (1/Day). The mephit casts Fog Cloud, requiring no spell
    components and using Charisma as the spellcasting ability. Frost Breath (Recharge 6). Constitution
    Saving Throw: DC 10, each creature in a 15-foot Cone. Failure: 7 (3d4) Cold damage. Success: Half
    damage.'
  sha256: 2eb2103d6faa3548ad55e19e3912660378dcf74dcaa5cde9f0920cfd262bdfeb
  status: needs_semantic_conversion
---

# Ice Mephit

<!-- SRD page 307 -->

Small Elemental, Neutral Evil\
AC 11 Initiative +1 (11)\
HP 21 (6d6)\
Speed 30 ft., Fly 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 7 −2 −2 Dex 13 +1 +1 Con 10 +0 +0\
Int 9 −1 −1 Wis 11 +0 +0 Cha 12 +1 +1\
Skills Perception +2, Stealth +3\
Vulnerabilities Fire\
Immunities Cold, Poison; Exhaustion, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages Primordial (Aquan, Auran)\
CR 1/2 (XP 100; PB +2)\

### Traits

Death Burst. The mephit explodes when it dies. Consti-\
tution Saving Throw: DC 10, each creature in a 5-foot\
Emanation originating from the mephit. Failure: 5 (2d4)\
Cold damage. Success: Half damage.\

### Actions

Claw. Melee Attack Roll: +3, reach 5 ft. Hit: 3 (1d4 + 1)\
Slashing damage plus 2 (1d4) Cold damage.\
Fog Cloud (1/Day). The mephit casts Fog Cloud, re-\
quiring no spell components and using Charisma as the\
spellcasting ability.\
Frost Breath (Recharge 6). Constitution Saving Throw:\
DC 10, each creature in a 15-foot Cone. Failure: 7 (3d4)\
Cold damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/307|page 307]].
