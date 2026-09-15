---
splat_schema: 1
splat_id: 56739d5d-c675-5561-8eb4-4d331e83231c
splat_kind: creature
splat_key: dnd2024:creatures/behir
splat_name: Behir
splat_origin:
  key: dnd2024:reference/creature/behir
splat_sources:
- title: SRD 5.2.1
  locator: pp. 263
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: monstrosity
  alignment: Neutral Evil
  movement_text: 50 ft., Climb 50 ft.
  skills_text: Perception +6, Stealth +7
  immunities_text: Lightning
  senses_text: Darkvision 90 ft.; Passive Perception 16
  languages_text: Draconic
  cr_text: 11 (XP 7,200; PB +4)
  hit_points_text: 168 (16d12 + 64)
splat_stats:
  strength:
    value: 23
  save_strength:
    value: 6
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 18
  save_constitution:
    value: 4
  intelligence:
    value: 7
  save_intelligence:
    value: -2
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 12
  save_charisma:
    value: 1
  armor_class:
    value: 17
  max_hp:
    value: 168
  initiative:
    value: 3
  speed:
    value: 50
    unit: ft
  climb_speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 4f91eff54d4306101de2cdd818ab797c7fd3c3c365613f67e21578eed3d756fb
  source_path: builtin/srd-5.2.1/creature/behir.md
  pages:
  - 263
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Behir Huge Monstrosity, Neutral Evil AC 17 Initiative +3 (13) HP 168 (16d12 + 64) Speed 50
    ft., Climb 50 ft. MOD SAVE MOD SAVE MOD SAVE Str 23 +6 +6 Dex 16 +3 +3 Con 18 +4 +4 Int 7 -2 -2 Wis
    14 +2 +2 Cha 12 +1 +1 Skills Perception +6, Stealth +7 Immunities Lightning Senses Darkvision 90 ft.;
    Passive Perception 16 Languages Draconic CR 11 (XP 7,200; PB +4) ### Actions Multiattack. The behir
    makes one Bite attack and uses Constrict. Bite. Melee Attack Roll: +10, reach 10 ft. Hit: 19 (2d12
    + 6) Piercing damage plus 11 (2d10) Lightning damage. Constrict. Strength Saving Throw: DC 18, one
    Large or smaller creature the behir can see within 5 feet. Failure: 28 (5d8 + 6) Bludgeoning damage.
    The target has the Grappled condition (escape DC 16), and it has the Restrained condition until the
    grapple ends. Lightning Breath (Recharge 5–6). Dexterity Saving Throw: DC 16, each creature in a 90-foot-long,
    5-footwide Line. Failure: 66 (12d10) Lightning damage. Success: Half damage. ### Bonus Actions Swallow.
    Dexterity Saving Throw: DC 18, one Large or smaller creature Grappled by the behir (the behir can
    have only one creature swallowed at a time). Failure: The behir swallows the target, which is no longer
    Grappled. While swallowed, a creature has the Blinded and Restrained conditions, has Total Cover against
    attacks and other effects outside the behir, and takes 21 (6d6) Acid damage at the start of each of
    the behir’s turns. If the behir takes 30 damage or more on a single turn from the swallowed creature,
    the behir must succeed on a DC 14 Constitution saving throw at the end of that turn or regurgitate
    the creature, which falls in a space within 10 feet of the behir and has the Prone condition. If the
    behir dies, a swallowed creature is no longer Restrained and can escape from the corpse by using 15
    feet of movement, exiting Prone.'
  sha256: 6de71422cb4affdf0a15bee85600027ee00ff41bcbece15b138bd19f0420a894
  status: needs_semantic_conversion
---

# Behir

<!-- SRD page 263 -->

Huge Monstrosity, Neutral Evil\
AC 17 Initiative +3 (13)\
HP 168 (16d12 + 64)\
Speed 50 ft., Climb 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 16 +3 +3 Con 18 +4 +4\
Int 7 −2 −2 Wis 14 +2 +2 Cha 12 +1 +1\
Skills Perception +6, Stealth +7\
Immunities Lightning\
Senses Darkvision 90 ft.; Passive Perception 16\
Languages Draconic\
CR 11 (XP 7,200; PB +4)\

### Actions

Multiattack. The behir makes one Bite attack and uses\
Constrict.\
Bite. Melee Attack Roll: +10, reach 10 ft. Hit: 19 (2d12\
+ 6) Piercing damage plus 11 (2d10) Lightning damage.\
Constrict. Strength Saving Throw: DC 18, one Large or\
smaller creature the behir can see within 5 feet. Failure:\
28 (5d8 + 6) Bludgeoning damage. The target has the\
Grappled condition (escape DC 16), and it has the Re-\
strained condition until the grapple ends.\
Lightning Breath (Recharge 5–6). Dexterity Saving\
Throw: DC 16, each creature in a 90-foot-long, 5-foot-\
wide Line. Failure: 66 (12d10) Lightning damage. Suc-\
cess: Half damage.\

### Bonus Actions

Swallow. Dexterity Saving Throw: DC 18, one Large or\
smaller creature Grappled by the behir (the behir can\
have only one creature swallowed at a time). Failure:\
The behir swallows the target, which is no longer Grap-\
pled. While swallowed, a creature has the Blinded and\
Restrained conditions, has Total Cover against attacks\
and other effects outside the behir, and takes 21 (6d6)\
Acid damage at the start of each of the behir’s turns.\
If the behir takes 30 damage or more on a single turn\
from the swallowed creature, the behir must succeed\
on a DC 14 Constitution saving throw at the end of that\
turn or regurgitate the creature, which falls in a space\
within 10 feet of the behir and has the Prone condition.\
If the behir dies, a swallowed creature is no longer Re-\
strained and can escape from the corpse by using 15\
feet of movement, exiting Prone.\

Original layout: [[../../srd-5.2.1/pages/263|page 263]].
