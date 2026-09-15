---
splat_schema: 1
splat_id: 1e20ad87-337a-5f20-bd89-a170439e60fc
splat_kind: creature
splat_key: dnd2024:creatures/giant_insect
splat_name: Giant Insect
splat_origin:
  key: dnd2024:reference/creature/giant_insect
splat_sources:
- title: SRD 5.2.1
  locator: pp. 136
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 40 ft., Climb 40 ft., Fly 40 ft. (Wasp only)
  senses_text: Darkvision 60 ft.; Passive Perception 12
  languages_text: Understands the languages you know
  cr_text: None (XP 0; PB equals your Proficiency Bonus)
  hit_points_text: 30 + 10 for each spell level above 4
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 4
  save_intelligence:
    value: -3
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 11
  speed:
    value: 40
    unit: ft
  climb_speed:
    value: 40
    unit: ft
  fly_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 44af60f6c2c349b80e27adb47dba51ec9ec1deef6b6a8ae462bd57a3078239ba
  source_path: builtin/srd-5.2.1/creature/giant_insect.md
  pages:
  - 136
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Giant Insect Large Beast, Unaligned AC 11 + the spell’s level HP 30 + 10 for each spell level
    above 4 Speed 40 ft., Climb 40 ft., Fly 40 ft. (Wasp only) MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3
    Dex 13 +1 +1 Con15 +2 +2 Int 4 -3 -3 Wis 14 +2 +2 Cha 3 -4 -4 Senses Darkvision 60 ft.; Passive Perception
    12 Languages Understands the languages you know CR None (XP 0; PB equals your Proficiency Bonus) ###
    Traits Spider Climb. The insect can climb difficult surfaces, including along ceilings, without needing
    to make an ability check. ### Actions Multiattack. The insect makes a number of attacks equal to half
    this spell’s level (round down). Poison Jab. Melee Attack Roll: Bonus equals your spell attack modifier,
    reach 10 ft. Hit: 1d6 + 3 plus the spell’s level Piercing damage plus 1d4 Poison damage. Web Bolt
    (Spider Only). Ranged Attack Roll: Bonus equals your spell attack modifier, range 60 ft. Hit: 1d10
    + 3 plus the spell’s level Bludgeoning damage, and the target’s Speed is reduced to 0 until the start
    of the insect’s next turn. ### Bonus Actions Venomous Spew (Centipede Only). Constitution Saving Throw:
    Your spell save DC, one creature the insect can see within 10 feet. Failure: The target has the Poisoned
    condition until the start of the insect’s next turn.'
  sha256: 5b40e20cb349c1f619e7396496d6219b6dac428ab464b62cc4e5768d4ed54d4d
  status: needs_semantic_conversion
---

# Giant Insect

<!-- SRD page 136 -->

Large Beast, Unaligned\
AC 11 + the spell’s level\
HP 30 + 10 for each spell level above 4\
Speed 40 ft., Climb 40 ft., Fly 40 ft. (Wasp only)\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 13 +1 +1 Con15 +2 +2\
Int 4 −3 −3 Wis 14 +2 +2 Cha 3 −4 −4\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages Understands the languages you know\
CR None (XP 0; PB equals your Proficiency Bonus)\

### Traits

Spider Climb. The insect can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\

### Actions

Multiattack. The insect makes a number of attacks\
equal to half this spell’s level (round down).\
Poison Jab. Melee Attack Roll: Bonus equals your spell\
attack modifier, reach 10 ft. Hit: 1d6 + 3 plus the spell’s\
level Piercing damage plus 1d4 Poison damage.\
Web Bolt (Spider Only). Ranged Attack Roll: Bonus\
equals your spell attack modifier, range 60 ft. Hit: 1d10\
+ 3 plus the spell’s level Bludgeoning damage, and the\
target’s Speed is reduced to 0 until the start of the in-\
sect’s next turn.\

### Bonus Actions

Venomous Spew (Centipede Only). Constitution Saving\
Throw: Your spell save DC, one creature the insect can\
see within 10 feet. Failure: The target has the Poisoned\
condition until the start of the insect’s next turn.\

Original layout: [[../../srd-5.2.1/pages/136|page 136]].
