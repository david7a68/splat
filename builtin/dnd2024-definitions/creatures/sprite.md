---
splat_schema: 1
splat_id: 5ff9859e-7b55-5d3c-9841-f7f6300087bb
splat_kind: creature
splat_key: dnd2024:creatures/sprite
splat_name: Sprite
splat_origin:
  key: dnd2024:reference/creature/sprite
splat_sources:
- title: SRD 5.2.1
  locator: pp. 329
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: fey
  alignment: Neutral Good
  movement_text: 10 ft., Fly 40 ft.
  skills_text: Perception +3, Stealth +8
  senses_text: Passive Perception 13
  languages_text: Common, Elvish, Sylvan
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 10 (4d4)
splat_stats:
  strength:
    value: 3
  save_strength:
    value: -4
  dexterity:
    value: 18
  save_dexterity:
    value: 4
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 14
  save_intelligence:
    value: 2
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 11
  save_charisma:
    value: 0
  armor_class:
    value: 15
  max_hp:
    value: 10
  initiative:
    value: 4
  speed:
    value: 10
    unit: ft
  fly_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 1f4b677ff8efafac0ae17aeeb83c15170970dd15916f24f72cdcb9095fe33824
  source_path: builtin/srd-5.2.1/creature/sprite.md
  pages:
  - 329
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Sprite Tiny Fey, Neutral Good AC 15 Initiative +4 (14) HP 10 (4d4) Speed 10 ft., Fly 40 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 3 -4 -4 Dex 18 +4 +4 Con10 +0 +0 Int 14 +2 +2 Wis 13 +1 +1 Cha 11 +0
    +0 Skills Perception +3, Stealth +8 Senses Passive Perception 13 Languages Common, Elvish, Sylvan
    CR 1/4 (XP 50; PB +2) ### Actions Needle Sword. Melee Attack Roll: +6, reach 5 ft. Hit: 6 (1d4 + 4)
    Piercing damage. Enchanting Bow. Ranged Attack Roll: +6, range 40/160 ft. Hit: 1 Piercing damage,
    and the target has the Charmed condition until the start of the sprite’s next turn. Heart Sight. Charisma
    Saving Throw: DC 10, one creature within 5 feet the sprite can see (Celestials, Fiends, and Undead
    automatically fail the save). Failure: The sprite knows the target’s emotions and alignment. Invisibility.
    The sprite casts Invisibility on itself, requiring no spell components and using Charisma as the spellcasting
    ability.'
  sha256: 92e8d98f979affe55d0fd9200a0ce859ed55d52b0783a19309322c45e1efd915
  status: needs_semantic_conversion
---

# Sprite

<!-- SRD page 329 -->

Tiny Fey, Neutral Good\
AC 15  Initiative +4 (14)\
HP 10 (4d4)\
Speed 10 ft., Fly 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 3 −4 −4 Dex 18 +4 +4 Con10 +0 +0\
Int 14 +2 +2 Wis 13 +1 +1 Cha 11 +0 +0\
Skills Perception +3, Stealth +8\
Senses Passive Perception 13\
Languages Common, Elvish, Sylvan\
CR 1/4 (XP 50; PB +2)\

### Actions

Needle Sword. Melee Attack Roll: +6, reach 5 ft. Hit: 6\
(1d4 + 4) Piercing damage.\
Enchanting Bow. Ranged Attack Roll: +6, range\
40/160 ft. Hit: 1 Piercing damage, and the target has\
the Charmed condition until the start of the sprite’s\
next turn.\
Heart Sight. Charisma Saving Throw: DC 10, one crea-\
ture within 5 feet the sprite can see (Celestials, Fiends,\
and Undead automatically fail the save). Failure: The\
sprite knows the target’s emotions and alignment.\
Invisibility. The sprite casts Invisibility on itself, requir-\
ing no spell components and using Charisma as the\
spellcasting ability.\

Original layout: [[../../srd-5.2.1/pages/329|page 329]].
