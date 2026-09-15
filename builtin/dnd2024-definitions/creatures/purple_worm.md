---
splat_schema: 1
splat_id: b0392038-ffb8-5c6d-9fe9-99bc4eda61ee
splat_kind: creature
splat_key: dnd2024:creatures/purple_worm
splat_name: Purple Worm
splat_origin:
  key: dnd2024:reference/creature/purple_worm
splat_sources:
- title: SRD 5.2.1
  locator: pp. 316, 317
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: gargantuan
  type: monstrosity
  alignment: Unaligned
  movement_text: 50 ft., Burrow 50 ft.
  senses_text: Blindsight 30 ft., Tremorsense 60 ft.; Passive Perception 9
  languages_text: None
  cr_text: 15 (XP 13,000; PB +5)
  hit_points_text: 247 (15d20 + 90)
splat_stats:
  strength:
    value: 28
  save_strength:
    value: 9
  dexterity:
    value: 7
  save_dexterity:
    value: -2
  constitution:
    value: 22
  save_constitution:
    value: 11
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 8
  save_wisdom:
    value: 4
  charisma:
    value: 4
  save_charisma:
    value: -3
  armor_class:
    value: 18
  max_hp:
    value: 247
  initiative:
    value: 3
  speed:
    value: 50
    unit: ft
  burrow_speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: cb7b19b783001d1e37f5c6effa6f2f9af306c843905bbeacf614c56e9d366cc2
  source_path: builtin/srd-5.2.1/creature/purple_worm.md
  pages:
  - 316
  - 317
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Purple Worm Gargantuan Monstrosity, Unaligned AC 18 Initiative +3 (13) HP 247 (15d20 + 90)
    Speed 50 ft., Burrow 50 ft. MOD SAVE MOD SAVE MOD SAVE Str 28 +9 +9 Dex 7 -2 -2 Con 22 +6 +11 Int
    1 -5 -5 Wis 8 -1 +4 Cha 4 -3 -3 Senses Blindsight 30 ft., Tremorsense 60 ft.; Passive Perception 9
    Languages None CR 15 (XP 13,000; PB +5) ### Traits Tunneler. The worm can burrow through solid rock
    at half its Burrow Speed and leaves a 10-foot-diameter tunnel in its wake. ### Actions Multiattack.
    The worm makes one Bite attack and one Tail Stinger attack. Bite. Melee Attack Roll: +14, reach 10
    ft. Hit: 22 (3d8 + 9) Piercing damage. If the target is a Large or smaller creature, it has the Grappled
    condition (escape DC 19), and it has the Restrained condition until the grapple ends. Tail Stinger.
    Melee Attack Roll: +14, reach 10 ft. Hit: 16 (2d6 + 9) Piercing damage plus 35 (10d6) Poison damage.
    ### Bonus Actions Swallow. Strength Saving Throw: DC 19, one Large or smaller creature Grappled by
    the worm (it can have up to three creatures swallowed at a time). Failure: The target is swallowed
    by the worm, and the Grappled condition ends. A swallowed creature has the Blinded and Restrained
    conditions, has Total Cover against attacks and other effects outside the worm, and takes 17 (5d6)
    Acid damage at the start of each of the worm’s turns. If the worm takes 30 damage or more on a single
    turn from a creature inside it, the worm must succeed on a DC 21 Constitution saving throw at the
    end of that turn or regurgitate all swallowed creatures, each of which falls in a space within 5 feet
    of the worm and has the Prone condition. If the worm dies, any swallowed creature no longer has the
    Restrained condition and can escape from the corpse using 20 feet of movement, exiting Prone.'
  sha256: f20fc4d688d675825590278168f4e720cbd60fc45424b54605d49d7cb4e4e414
  status: needs_semantic_conversion
---

# Purple Worm

<!-- SRD page 316 -->

Gargantuan Monstrosity, Unaligned\
AC 18 Initiative +3 (13)\
HP 247 (15d20 + 90)\
Speed 50 ft., Burrow 50 ft.\

<!-- SRD page 317 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 28 +9 +9 Dex 7 −2 −2 Con 22 +6 +11\
Int 1 −5 −5 Wis 8 −1 +4 Cha 4 −3 −3\
Senses Blindsight 30 ft., Tremorsense 60 ft.;\
Passive Perception 9\
Languages None\
CR 15 (XP 13,000; PB +5)\

### Traits

Tunneler. The worm can burrow through solid rock at\
half its Burrow Speed and leaves a 10-foot-diameter\
tunnel in its wake.\

### Actions

Multiattack. The worm makes one Bite attack and one\
Tail Stinger attack.\
Bite. Melee Attack Roll: +14, reach 10 ft. Hit: 22\
(3d8 + 9) Piercing damage. If the target is a Large or\
smaller creature, it has the Grappled condition (escape\
DC 19), and it has the Restrained condition until the\
grapple ends.\
Tail Stinger. Melee Attack Roll: +14, reach 10 ft. Hit:\
16 (2d6 + 9) Piercing damage plus 35 (10d6) Poi-\
son damage.\

### Bonus Actions

Swallow. Strength Saving Throw: DC 19, one Large or\
smaller creature Grappled by the worm (it can have up\
to three creatures swallowed at a time). Failure: The tar-\
get is swallowed by the worm, and the Grappled con-\
dition ends. A swallowed creature has the Blinded and\
Restrained conditions, has Total Cover against attacks\
and other effects outside the worm, and takes 17 (5d6)\
Acid damage at the start of each of the worm’s turns.\
If the worm takes 30 damage or more on a single turn\
from a creature inside it, the worm must succeed on a\
DC 21 Constitution saving throw at the end of that turn\
or regurgitate all swallowed creatures, each of which\
falls in a space within 5 feet of the worm and has the\
Prone condition. If the worm dies, any swallowed crea-\
ture no longer has the Restrained condition and can\
escape from the corpse using 20 feet of movement,\
exiting Prone.\

Original layout: [[../../srd-5.2.1/pages/316|page 316]], [[../../srd-5.2.1/pages/317|page 317]].
