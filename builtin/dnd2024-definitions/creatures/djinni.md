---
splat_schema: 1
splat_id: 7adeeade-0bcf-5c22-a420-eb750d4da8f6
splat_kind: creature
splat_key: dnd2024:creatures/djinni
splat_name: Djinni
splat_origin:
  key: dnd2024:reference/creature/djinni
splat_sources:
- title: SRD 5.2.1
  locator: pp. 280
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: elemental
  alignment: Neutral
  tags_text: (Genie)
  movement_text: 30 ft., Fly 90 ft. (hover)
  immunities_text: Lightning, Thunder
  senses_text: Darkvision 120 ft.; Passive Perception 13
  languages_text: Primordial (Auran)
  cr_text: 11 (XP 7,200; PB +4)
  hit_points_text: 218 (19d10 + 114)
splat_stats:
  strength:
    value: 21
  save_strength:
    value: 5
  dexterity:
    value: 15
  save_dexterity:
    value: 6
  constitution:
    value: 22
  save_constitution:
    value: 6
  intelligence:
    value: 15
  save_intelligence:
    value: 2
  wisdom:
    value: 16
  save_wisdom:
    value: 7
  charisma:
    value: 20
  save_charisma:
    value: 5
  armor_class:
    value: 17
  max_hp:
    value: 218
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 90
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 1fbb270a5e437e4ab4fc146a6668e11f68883b70b9a4947e026ff74850500272
  source_path: builtin/srd-5.2.1/creature/djinni.md
  pages:
  - 280
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Djinni Large Elemental (Genie), Neutral AC 17 Initiative +2 (12) HP 218 (19d10 + 114) Speed
    30 ft., Fly 90 ft. (hover) MOD SAVE MOD SAVE MOD SAVE Str 21 +5 +5 Dex 15 +2 +6 Con 22 +6 +6 Int 15
    +2 +2 Wis 16 +3 +7 Cha 20 +5 +5 Immunities Lightning, Thunder Senses Darkvision 120 ft.; Passive Perception
    13 Languages Primordial (Auran) CR 11 (XP 7,200; PB +4) ### Traits Elemental Restoration. If the djinni
    dies outside the Elemental Plane of Air, its body dissolves into mist, and it gains a new body in
    1d4 days, reviving with all its Hit Points somewhere on the Plane of Air. Magic Resistance. The djinni
    has Advantage on saving throws against spells and other magical effects. Wishes. The djinni has a
    30 percent chance of knowing the Wish spell. If the djinni knows it, the djinni can cast it only on
    behalf of a non-genie creature who communicates a wish in a way the djinni can understand. If the
    djinni casts the spell for the creature, the djinni suffers none of the spell’s stress. Once the djinni
    has cast it three times, the djinni can’t do so again for 365 days. ### Actions Multiattack. The djinni
    makes three attacks, using Storm Blade or Storm Bolt in any combination. Storm Blade. Melee Attack
    Roll: +9, reach 5 feet. Hit: 12 (2d6 + 5) Slashing damage plus 7 (2d6) Lightning damage. Storm Bolt.
    Ranged Attack Roll: +9, range 120 feet. Hit: 13 (3d8) Thunder damage. If the target is a Large or
    smaller creature, it has the Prone condition. Create Whirlwind. The djinni conjures a whirlwind at
    a point it can see within 120 feet. The whirlwind fills a 20-foot-radius, 60-foot-high Cylinder centered
    on that point. The whirlwind lasts until the djinni’s Concentration on it ends. The djinni can move
    the whirlwind up to 20 feet at the start of each of its turns. Whenever the whirlwind enters a creature’s
    space or a creature enters the whirlwind, that creature is subjected to the following effect. Strength
    Saving Throw: DC 17 (a creature makes this save only once per turn, and the djinni is unaffected).
    Failure: While in the whirlwind, the target has the Restrained condition and moves with the whirlwind.
    At the start of each of its turns, the Restrained target takes 21 (6d6) Thunder damage. At the end
    of each of its turns, the target repeats the save, ending the effect on itself on a success. Spellcasting.
    The djinni casts one of the following spells, requiring no Material components and using Charisma
    as the spellcasting ability (spell save DC 17): At Will: Detect Evil and Good, Detect Magic 2/Day
    Each: Create Food and Water (can create wine instead of water), Tongues, Wind Walk 1/Day Each: Creation,
    Gaseous Form, Invisibility, Major Image, Plane Shift'
  sha256: 88a4a52845f03cf698b7e3a5f7199d4e44ca778c486d2f87daf264bfbc9f900e
  status: needs_semantic_conversion
---

# Djinni

<!-- SRD page 280 -->

Large Elemental (Genie), Neutral\
AC 17  Initiative +2 (12)\
HP 218 (19d10 + 114)\
Speed 30 ft., Fly 90 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +5 Dex 15 +2 +6 Con 22 +6 +6\
Int 15 +2 +2 Wis 16 +3 +7 Cha 20 +5 +5\
Immunities Lightning, Thunder\
Senses Darkvision 120 ft.; Passive Perception 13\
Languages Primordial (Auran)\
CR 11 (XP 7,200; PB +4)\

### Traits

Elemental Restoration. If the djinni dies outside the\
Elemental Plane of Air, its body dissolves into mist, and\
it gains a new body in 1d4 days, reviving with all its Hit\
Points somewhere on the Plane of Air.\
Magic Resistance. The djinni has Advantage on saving\
throws against spells and other magical effects.\
Wishes. The djinni has a 30 percent chance of knowing\
the Wish spell. If the djinni knows it, the djinni can cast\
it only on behalf of a non-genie creature who commu-\
nicates a wish in a way the djinni can understand. If the\
djinni casts the spell for the creature, the djinni suffers\
none of the spell’s stress. Once the djinni has cast it\
three times, the djinni can’t do so again for 365 days.\

### Actions

Multiattack. The djinni makes three attacks, using\
Storm Blade or Storm Bolt in any combination.\
Storm Blade. Melee Attack Roll: +9, reach 5 feet.\
Hit: 12 (2d6 + 5) Slashing damage plus 7 (2d6) Light-\
ning damage.\
Storm Bolt. Ranged Attack Roll: +9, range 120 feet.\
Hit: 13 (3d8) Thunder damage. If the target is a Large or\
smaller creature, it has the Prone condition.\
Create Whirlwind. The djinni conjures a whirlwind at\
a point it can see within 120 feet. The whirlwind fills a\
20-foot-radius, 60-foot-high Cylinder centered on that\
point. The whirlwind lasts until the djinni’s Concentra-\
tion on it ends. The djinni can move the whirlwind up\
to 20 feet at the start of each of its turns.\
Whenever the whirlwind enters a creature’s space or\
a creature enters the whirlwind, that creature is sub-\
jected to the following effect. Strength Saving Throw:\
DC 17 (a creature makes this save only once per turn,\
and the djinni is unaffected). Failure: While in the\
whirlwind, the target has the Restrained condition\
and moves with the whirlwind. At the start of each of\
its turns, the Restrained target takes 21 (6d6) Thunder\
damage. At the end of each of its turns, the target re-\
peats the save, ending the effect on itself on a success.\
Spellcasting. The djinni casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 17):\
At Will: Detect Evil and Good, Detect Magic\
2/Day Each: Create Food and Water (can create wine\
instead of water), Tongues, Wind Walk\
1/Day Each: Creation, Gaseous Form, Invisibility, Major\
Image, Plane Shift\

Original layout: [[../../srd-5.2.1/pages/280|page 280]].
