---
splat_schema: 1
splat_id: 45736a8e-2b24-57d7-ba5a-3551a26cfee1
splat_kind: creature
splat_key: dnd2024:creatures/remorhaz
splat_name: Remorhaz
splat_origin:
  key: dnd2024:reference/creature/remorhaz
splat_sources:
- title: SRD 5.2.1
  locator: pp. 319, 320
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: monstrosity
  alignment: Unaligned
  movement_text: 40 ft., Burrow 30 ft.
  immunities_text: Cold, Fire
  senses_text: Darkvision 60 ft., Tremorsense 60 ft.; Passive Perception 10
  languages_text: None
  cr_text: 11 (XP 7,200; PB +4)
  hit_points_text: 195 (17d12 + 85)
splat_stats:
  strength:
    value: 24
  save_strength:
    value: 7
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 21
  save_constitution:
    value: 5
  intelligence:
    value: 4
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
    value: 17
  max_hp:
    value: 195
  initiative:
    value: 5
  speed:
    value: 40
    unit: ft
  burrow_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 7e7b1c49b60332c99da4125af12e4010fb7429152fab541cfd1812492d1554f7
  source_path: builtin/srd-5.2.1/creature/remorhaz.md
  pages:
  - 319
  - 320
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Remorhaz Huge Monstrosity, Unaligned AC 17 Initiative +5 (15) HP 195 (17d12 + 85) Speed 40
    ft., Burrow 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 24 +7 +7 Dex 13 +1 +1 Con 21 +5 +5 Int 4 -3 -3 Wis
    10 +0 +0 Cha 5 -3 -3 Immunities Cold, Fire Senses Darkvision 60 ft., Tremorsense 60 ft.; Passive Perception
    10 Languages None CR 11 (XP 7,200; PB +4) ### Traits Heat Aura. At the end of each of the remorhaz’s
    turns, each creature in a 5-foot Emanation originating from the remorhaz takes 16 (3d10) Fire damage.
    ### Actions Bite. Melee Attack Roll: +11, reach 10 ft. Hit: 18 (2d10 + 7) Piercing damage plus 14
    (4d6) Fire damage. If the target is a Large or smaller creature, it has the Grappled condition (escape
    DC 17), and it has the Restrained condition until the grapple ends. ### Bonus Actions Swallow. Strength
    Saving Throw: DC 19, one Large or smaller creature Grappled by the remorhaz (it can have up to two
    creatures swallowed at a time). Failure: The target is swallowed by the remorhaz, and the Grappled
    condition ends. A swallowed creature has the Blinded and Restrained conditions, it has Total Cover
    against attacks and other effects outside the remorhaz, and it takes 10 (3d6) Acid damage plus 10
    (3d6) Fire damage at the start of each of the remorhaz’s turns. If the remorhaz takes 30 damage or
    more on a single turn from a creature inside it, the remorhaz must succeed on a DC 15 Constitution
    saving throw at the end of that turn or regurgitate all swallowed creatures, each of which falls in
    a space within 5 feet of the remorhaz and has the Prone condition. If the remorhaz dies, any swallowed
    creature no longer has the Restrained condition and can escape from the corpse by using 15 feet of
    movement, exiting Prone.'
  sha256: 558747f34caae5043d41bbc8a19732a42c73e675346f32e0c4d6a46a1aec1d81
  status: needs_semantic_conversion
---

# Remorhaz

<!-- SRD page 319 -->

Huge Monstrosity, Unaligned\
AC 17 Initiative +5 (15)\
HP 195 (17d12 + 85)\
Speed 40 ft., Burrow 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 24 +7 +7 Dex 13 +1 +1 Con 21 +5 +5\
Int 4 −3 −3 Wis 10 +0 +0 Cha 5 −3 −3\
Immunities Cold, Fire\
Senses Darkvision 60 ft., Tremorsense 60 ft.;\
Passive Perception 10\
Languages None\
CR 11 (XP 7,200; PB +4)\

### Traits

Heat Aura. At the end of each of the remorhaz’s turns,\
each creature in a 5-foot Emanation originating from\
the remorhaz takes 16 (3d10) Fire damage.\

### Actions

Bite. Melee Attack Roll: +11, reach 10 ft. Hit: 18 (2d10\
+ 7) Piercing damage plus 14 (4d6) Fire damage. If the\
target is a Large or smaller creature, it has the Grappled\
condition (escape DC 17), and it has the Restrained\
condition until the grapple ends.\

### Bonus Actions

Swallow. Strength Saving Throw: DC 19, one Large or\
smaller creature Grappled by the remorhaz (it can have\
up to two creatures swallowed at a time). Failure: The\
target is swallowed by the remorhaz, and the Grappled\
condition ends. A swallowed creature has the Blinded\

<!-- SRD page 320 -->

and Restrained conditions, it has Total Cover against\
attacks and other effects outside the remorhaz, and it\
takes 10 (3d6) Acid damage plus 10 (3d6) Fire damage\
at the start of each of the remorhaz’s turns.\
If the remorhaz takes 30 damage or more on a single\
turn from a creature inside it, the remorhaz must suc-\
ceed on a DC 15 Constitution saving throw at the end\
of that turn or regurgitate all swallowed creatures, each\
of which falls in a space within 5 feet of the remorhaz\
and has the Prone condition. If the remorhaz dies, any\
swallowed creature no longer has the Restrained condi-\
tion and can escape from the corpse by using 15 feet of\
movement, exiting Prone.\

Original layout: [[../../srd-5.2.1/pages/319|page 319]], [[../../srd-5.2.1/pages/320|page 320]].
