---
splat_schema: 1
splat_id: 4544c113-2676-582c-898a-9b6ad5031d8b
splat_kind: creature
splat_key: dnd2024:creatures/young_white_dragon
splat_name: Young White Dragon
splat_origin:
  key: dnd2024:reference/creature/young_white_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 340
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: dragon
  alignment: Chaotic Evil
  tags_text: (Chromatic)
  movement_text: 40 ft., Burrow 20 ft., Fly 80 ft., Swim 40 ft.
  skills_text: Perception +6, Stealth +3
  immunities_text: Cold
  senses_text: Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 16
  languages_text: Common, Draconic
  cr_text: 6 (2,300 XP; PB +3)
  hit_points_text: 123 (13d10 + 52)
  ambiguous_saves:
  - intelligence
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 10
  save_dexterity:
    value: 3
  constitution:
    value: 18
  save_constitution:
    value: 4
  intelligence:
    value: 6
  wisdom:
    value: 11
  save_wisdom:
    value: 3
  charisma:
    value: 12
  save_charisma:
    value: 1
  armor_class:
    value: 17
  max_hp:
    value: 123
  initiative:
    value: 3
  speed:
    value: 40
    unit: ft
  burrow_speed:
    value: 20
    unit: ft
  fly_speed:
    value: 80
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: de45456b6855dad80c7f504a26c2052291d1c034cfe78f90441dd86b2d1288ac
  source_path: builtin/srd-5.2.1/creature/young_white_dragon.md
  pages:
  - 340
  engine_gaps: []
  conversion_gaps:
  - Unsigned save_intelligence in official PDF; requires explicit correction policy
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Young White Dragon Large Dragon (Chromatic), Chaotic Evil AC 17 Initiative +3 (13) HP 123 (13d10
    + 52) Speed 40 ft., Burrow 20 ft., Fly 80 ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4
    Dex 10 +0 +3 Con 18 +4 +4 Int 6 -2 2 Wis 11 +0 +3 Cha 12 +1 +1 Skills Perception +6, Stealth +3 Immunities
    Cold Senses Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 16 Languages Common, Draconic
    CR 6 (2,300 XP; PB +3) ### Traits Ice Walk. The dragon can move across and climb icy surfaces without
    needing to make an ability check. Additionally, Difficult Terrain composed of ice or snow doesn’t
    cost it extra movement. ### Actions Multiattack. The dragon makes three Rend attacks. Rend. Melee
    Attack Roll: +7, reach 10 ft. Hit: 9 (2d4 + 4) Slashing damage plus 2 (1d4) Cold damage. Cold Breath
    (Recharge 5–6). Constitution Saving Throw: DC 15, each creature in a 30-foot Cone. Failure: 40 (9d8)
    Cold damage. Success: Half damage.'
  sha256: 949c1e21ce10adc5c7ddf91fcdf577587865d8f13ed275c4c16f6506ba1b9864
  status: needs_semantic_conversion
---

# Young White Dragon

<!-- SRD page 340 -->

Large Dragon (Chromatic), Chaotic Evil\
AC 17 Initiative +3 (13)\
HP 123 (13d10 + 52)\
Speed 40 ft., Burrow 20 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 10 +0 +3 Con 18 +4 +4\
Int 6 −2 2 Wis 11 +0 +3 Cha 12 +1 +1\
Skills Perception +6, Stealth +3\
Immunities Cold\
Senses Blindsight 30 ft., Darkvision 120 ft.;\
Passive Perception 16\
Languages Common, Draconic\
CR 6 (2,300 XP; PB +3)\

### Traits

Ice Walk. The dragon can move across and climb icy\
surfaces without needing to make an ability check.\
Additionally, Difficult Terrain composed of ice or snow\
doesn’t cost it extra movement.\

### Actions

Multiattack. The dragon makes three Rend attacks.\
Rend. Melee Attack Roll: +7, reach 10 ft. Hit: 9 (2d4\
+ 4) Slashing damage plus 2 (1d4) Cold damage.\
Cold Breath (Recharge 5–6). Constitution Saving\
Throw: DC 15, each creature in a 30-foot Cone. Failure:\
40 (9d8) Cold damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/340|page 340]].
