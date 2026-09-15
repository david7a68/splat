---
splat_schema: 1
splat_id: 33a71319-c74a-5cb6-9de9-2cd913a6f3f4
splat_kind: creature
splat_key: dnd2024:creatures/kraken
splat_name: Kraken
splat_origin:
  key: dnd2024:reference/creature/kraken
splat_sources:
- title: SRD 5.2.1
  locator: pp. 303
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: gargantuan
  type: monstrosity
  alignment: Chaotic Evil
  tags_text: (Titan)
  movement_text: 30 ft., Swim 120 ft.
  skills_text: History +13, Perception +11
  immunities_text: Cold, Lightning; Frightened, Grappled, Paralyzed, Restrained
  senses_text: Truesight 120 ft.; Passive Perception 21
  languages_text: Understands Abyssal, Celestial, Infernal, and Primordial but can’t speak; telepathy
    120 ft.
  cr_text: 23 (XP 50,000, or 62,000 in lair; PB +7)
  hit_points_text: 481 (26d20 + 208)
splat_stats:
  strength:
    value: 30
  save_strength:
    value: 17
  dexterity:
    value: 11
  save_dexterity:
    value: 7
  constitution:
    value: 26
  save_constitution:
    value: 15
  intelligence:
    value: 22
  save_intelligence:
    value: 6
  wisdom:
    value: 18
  save_wisdom:
    value: 11
  charisma:
    value: 20
  save_charisma:
    value: 5
  armor_class:
    value: 18
  max_hp:
    value: 481
  initiative:
    value: 14
  speed:
    value: 30
    unit: ft
  swim_speed:
    value: 120
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 73f5f049a9dd06b3c569d71a3fd3246698650d69ae02cc86c4a2d94135d0f262
  source_path: builtin/srd-5.2.1/creature/kraken.md
  pages:
  - 303
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Kraken Gargantuan Monstrosity (Titan), Chaotic Evil AC 18 Initiative +14 (24) HP 481 (26d20
    + 208) Speed 30 ft., Swim 120 ft. MOD SAVE MOD SAVE MOD SAVE Str 30 +10 +17 Dex 11 +0 +7 Con 26 +8
    +15 Int 22 +6 +6 Wis 18 +4 +11 Cha 20 +5 +5 Skills History +13, Perception +11 Immunities Cold, Lightning;
    Frightened, Grappled, Paralyzed, Restrained Senses Truesight 120 ft.; Passive Perception 21 Languages
    Understands Abyssal, Celestial, Infernal, and Primordial but can’t speak; telepathy 120 ft. CR 23
    (XP 50,000, or 62,000 in lair; PB +7) ### Traits Amphibious. The kraken can breathe air and water.
    Legendary Resistance (4/Day, or 5/Day in Lair). If the kraken fails a saving throw, it can choose
    to succeed instead. Siege Monster. The kraken deals double damage to objects and structures. ### Actions
    Multiattack. The kraken makes two Tentacle attacks and uses Fling, Lightning Strike, or Swallow. Tentacle.
    Melee Attack Roll: +17, reach 30 ft. Hit: 24 (4d6 + 10) Bludgeoning damage. The target has the Grappled
    condition (escape DC 20) from one of ten tentacles, and it has the Restrained condition until the
    grapple ends. Fling. The kraken throws a Large or smaller creature Grappled by it to a space it can
    see within 60 feet of itself that isn’t in the air. Dexterity Saving Throw: DC 25, the creature thrown
    and each creature in the destination space. Failure: 18 (4d8) Bludgeoning damage, and the target has
    the Prone condition. Success: Half damage only. Lightning Strike. Dexterity Saving Throw: DC 23, one
    creature the kraken can see within 120 feet. Failure: 33 (6d10) Lightning damage. Success: Half damage.
    Swallow. Dexterity Saving Throw: DC 25, one creature Grappled by the kraken (it can have up to four
    creatures swallowed at a time). Failure: 23 (3d8 + 10) Piercing damage. If the target is Large or
    smaller, it is swallowed and no longer Grappled. A swallowed creature has the Restrained condition,
    has Total Cover against attacks and other effects outside the kraken, and takes 24 (7d6) Acid damage
    at the start of each of its turns. If the kraken takes 50 damage or more on a single turn from a creature
    inside it, the kraken must succeed on a DC 25 Constitution saving throw at the end of that turn or
    regurgitate all swallowed creatures, each of which falls in a space within 10 feet of the kraken with
    the Prone condition. If the kraken dies, any swallowed creature no longer has the Restrained condition
    and can escape from the corpse using 15 feet of movement, exiting Prone. ### Legendary Actions Legendary
    Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the kraken can expend a use
    to take one of the following actions. The kraken regains all expended uses at the start of each of
    its turns. Storm Bolt. The kraken uses Lightning Strike. Toxic Ink. Constitution Saving Throw: DC
    23, each creature in a 15-foot Emanation originating from the kraken while it is underwater. Failure:
    The target has the Blinded and Poisoned conditions until the end of the kraken’s next turn. The kraken
    then moves up to its Speed. Failure or Success: The kraken can’t take this action again until the
    start of its next turn.'
  sha256: 873dee7ef210292837a6c85cac28b77fd19c8799aca14970331ad578c0d9377c
  status: needs_semantic_conversion
---

# Kraken

<!-- SRD page 303 -->

Gargantuan Monstrosity (Titan), Chaotic Evil\
AC 18 Initiative +14 (24)\
HP 481 (26d20 + 208)\
Speed 30 ft., Swim 120 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 30 +10 +17 Dex 11 +0 +7 Con 26 +8 +15\
Int 22 +6 +6 Wis 18 +4 +11 Cha 20 +5 +5\
Skills History +13, Perception +11\
Immunities Cold, Lightning; Frightened, Grappled,\
Paralyzed, Restrained\
Senses Truesight 120 ft.; Passive Perception 21\
Languages Understands Abyssal, Celestial, Infernal, and\
Primordial but can’t speak; telepathy 120 ft.\
CR 23 (XP 50,000, or 62,000 in lair; PB +7)\

### Traits

Amphibious. The kraken can breathe air and water.\
Legendary Resistance (4/Day, or 5/Day in Lair). If\
the kraken fails a saving throw, it can choose to suc-\
ceed instead.\
Siege Monster. The kraken deals double damage to ob-\
jects and structures.\

### Actions

Multiattack. The kraken makes two Tentacle attacks\
and uses Fling, Lightning Strike, or Swallow.\
Tentacle. Melee Attack Roll: +17, reach 30 ft. Hit: 24\
(4d6 + 10) Bludgeoning damage. The target has the\
Grappled condition (escape DC 20) from one of ten\
tentacles, and it has the Restrained condition until the\
grapple ends.\
Fling. The kraken throws a Large or smaller creature\
Grappled by it to a space it can see within 60 feet of\
itself that isn’t in the air. Dexterity Saving Throw: DC\
25, the creature thrown and each creature in the des-\
tination space. Failure: 18 (4d8) Bludgeoning damage,\
and the target has the Prone condition. Success: Half\
damage only.\
Lightning Strike. Dexterity Saving Throw: DC 23, one\
creature the kraken can see within 120 feet. Failure: 33\
(6d10) Lightning damage. Success: Half damage.\
Swallow. Dexterity Saving Throw: DC 25, one creature\
Grappled by the kraken (it can have up to four crea-\
tures swallowed at a time). Failure: 23 (3d8 + 10) Pierc-\
ing damage. If the target is Large or smaller, it is swal-\
lowed and no longer Grappled. A swallowed creature\
has the Restrained condition, has Total Cover against\
attacks and other effects outside the kraken, and takes\
24 (7d6) Acid damage at the start of each of its turns.\
If the kraken takes 50 damage or more on a single\
turn from a creature inside it, the kraken must succeed\
on a DC 25 Constitution saving throw at the end of\
that turn or regurgitate all swallowed creatures, each of\
which falls in a space within 10 feet of the kraken with\
the Prone condition. If the kraken dies, any swallowed\
creature no longer has the Restrained condition and\
can escape from the corpse using 15 feet of movement,\
exiting Prone.\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the kraken can expend a use to\
take one of the following actions. The kraken regains all\
expended uses at the start of each of its turns.\
Storm Bolt. The kraken uses Lightning Strike.\
Toxic Ink. Constitution Saving Throw: DC 23, each\
creature in a 15-foot Emanation originating from the\
kraken while it is underwater. Failure: The target has\
the Blinded and Poisoned conditions until the end of\
the kraken’s next turn. The kraken then moves up to\
its Speed. Failure or Success: The kraken can’t take this\
action again until the start of its next turn.\

Original layout: [[../../srd-5.2.1/pages/303|page 303]].
