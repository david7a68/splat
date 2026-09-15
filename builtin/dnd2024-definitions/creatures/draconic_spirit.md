---
splat_schema: 1
splat_id: 447f69de-fddc-5113-8938-50e6f1ebe886
splat_kind: creature
splat_key: dnd2024:creatures/draconic_spirit
splat_name: Draconic Spirit
splat_origin:
  key: dnd2024:reference/creature/draconic_spirit
splat_sources:
- title: SRD 5.2.1
  locator: pp. 166
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: dragon
  alignment: Neutral
  movement_text: 30 ft., Fly 60 ft., Swim 30 ft.
  resistances_text: Acid, Cold, Fire, Lightning, Poison
  immunities_text: Charmed, Frightened, Poisoned
  senses_text: Blindsight 30 ft., Darkvision 60 ft.; Passive Perception 12
  languages_text: Draconic, understands the languages you know
  cr_text: None (XP 0; PB equals your Proficiency Bonus)
  hit_points_text: 50 + 10 for each spell level above 5
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 14
  save_charisma:
    value: 2
  armor_class:
    value: 14
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 60
    unit: ft
  swim_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: fc4d323c81b90c1ae4d392593ccd6c2b74b2c502c712b366a5e9a1b181785c8d
  source_path: builtin/srd-5.2.1/creature/draconic_spirit.md
  pages:
  - 166
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Draconic Spirit Large Dragon, Neutral AC 14 + the spell’s level HP 50 + 10 for each spell level
    above 5 Speed 30 ft., Fly 60 ft., Swim 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 14 +2 +2
    Con17 +3 +3 Int 10 +0 +0 Wis 14 +2 +2 Cha 14 +2 +2 Resistances Acid, Cold, Fire, Lightning, Poison
    Immunities Charmed, Frightened, Poisoned Senses Blindsight 30 ft., Darkvision 60 ft.; Passive Perception
    12 Languages Draconic, understands the languages you know CR None (XP 0; PB equals your Proficiency
    Bonus) ### Traits Shared Resistances. When you summon the spirit, choose one of its Resistances. You
    have Resistance to the chosen damage type until the spell ends. ### Actions Multiattack. The spirit
    makes a number of Rend attacks equal to half the spell’s level (round down), and it uses Breath Weapon.
    Rend. Melee Attack Roll: Bonus equals your spell attack modifier, reach 10 feet. Hit: 1d6 + 4 + the
    spell’s level Piercing damage. Breath Weapon. Dexterity Saving Throw: DC equals your spell save DC,
    each creature in a 30-foot Cone. Failure: 2d6 damage of a type this spirit has Resistance to (your
    choice when you cast the spell). Success: Half damage.'
  sha256: aab24bcc36ef77fbf5df7dd424593e716aa7d3b8df2246ead76978f9ca28fcc6
  status: needs_semantic_conversion
---

# Draconic Spirit

<!-- SRD page 166 -->

Large Dragon, Neutral\
AC 14 + the spell’s level\
HP 50 + 10 for each spell level above 5\
Speed 30 ft., Fly 60 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 14 +2 +2 Con17 +3 +3\
Int 10 +0 +0 Wis 14 +2 +2 Cha 14 +2 +2\
Resistances Acid, Cold, Fire, Lightning, Poison\
Immunities Charmed, Frightened, Poisoned\
Senses Blindsight 30 ft., Darkvision 60 ft.;\
Passive Perception 12\
Languages Draconic, understands the languages you know\
CR None (XP 0; PB equals your Proficiency Bonus)\

### Traits

Shared Resistances. When you summon the spirit,\
choose one of its Resistances. You have Resistance to\
the chosen damage type until the spell ends.\

### Actions

Multiattack. The spirit makes a number of Rend attacks\
equal to half the spell’s level (round down), and it uses\
Breath Weapon.\
Rend. Melee Attack Roll: Bonus equals your spell attack\
modifier, reach 10 feet. Hit: 1d6 + 4 + the spell’s level\
Piercing damage.\
Breath Weapon. Dexterity Saving Throw: DC equals\
your spell save DC, each creature in a 30-foot Cone.\
Failure: 2d6 damage of a type this spirit has Resistance\
to (your choice when you cast the spell). Success:\
Half damage.\

Original layout: [[../../srd-5.2.1/pages/166|page 166]].
