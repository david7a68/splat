---
splat_schema: 1
splat_id: 3690316d-fb14-5b76-b9f8-5230a0c69e47
splat_kind: creature
splat_key: dnd2024:creatures/knight
splat_name: Knight
splat_origin:
  key: dnd2024:reference/creature/knight
splat_sources:
- title: SRD 5.2.1
  locator: pp. 302
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  immunities_text: Frightened
  gear_text: Greatsword, Heavy Crossbow, Plate Armor
  senses_text: Passive Perception 10
  languages_text: Common plus one other language
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 52 (8d8 + 16)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 14
  save_constitution:
    value: 4
  intelligence:
    value: 11
  save_intelligence:
    value: 0
  wisdom:
    value: 11
  save_wisdom:
    value: 2
  charisma:
    value: 15
  save_charisma:
    value: 2
  armor_class:
    value: 18
  max_hp:
    value: 52
  initiative:
    value: 0
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 3f2c0dfd8c2d83959f55617c1114fde10137a47b2d952d43f7352f1ba99b77a4
  source_path: builtin/srd-5.2.1/creature/knight.md
  pages:
  - 302
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Knight Medium or Small Humanoid, Neutral AC 18 Initiative +0 (10) HP 52 (8d8 + 16) Speed 30
    ft. MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 11 +0 +0 Con 14 +2 +4 Int 11 +0 +0 Wis 11 +0 +2 Cha
    15 +2 +2 Immunities Frightened Gear Greatsword, Heavy Crossbow, Plate Armor Senses Passive Perception
    10 Languages Common plus one other language CR 3 (XP 700; PB +2) ### Actions Multiattack. The knight
    makes two attacks, using Greatsword or Heavy Crossbow in any combination. Greatsword. Melee Attack
    Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Slashing damage plus 4 (1d8) Radiant damage. Heavy Crossbow.
    Ranged Attack Roll: +2, range 100/400 ft. Hit: 11 (2d10) Piercing damage plus 4 (1d8) Radiant damage.
    ### Reactions Parry. Trigger: The knight is hit by a melee attack roll while holding a weapon. Response:
    The knight adds 2 to its AC against that attack, possibly causing it to miss.'
  sha256: 8de63fb51e195ce73877b8bf88556c61698ed7fa36a27c5cc5f65ddc16767312
  status: needs_semantic_conversion
---

# Knight

<!-- SRD page 302 -->

Medium or Small Humanoid, Neutral\
AC 18 Initiative +0 (10)\
HP 52 (8d8 + 16)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str  16 +3 +3 Dex 11 +0 +0 Con 14 +2 +4\
Int  11 +0 +0 Wis  11 +0 +2 Cha 15 +2 +2\
Immunities Frightened\
Gear Greatsword, Heavy Crossbow, Plate Armor\
Senses Passive Perception 10\
Languages Common plus one other language\
CR 3 (XP 700; PB +2)\

### Actions

Multiattack. The knight makes two attacks, using\
Greatsword or Heavy Crossbow in any combination.\
Greatsword. Melee Attack Roll: +5, reach 5 ft. Hit: 10\
(2d6 + 3) Slashing damage plus 4 (1d8) Radiant damage.\
Heavy Crossbow. Ranged Attack Roll: +2, range\
100/400 ft. Hit: 11 (2d10) Piercing damage plus 4 (1d8)\
Radiant damage.\

### Reactions

Parry. Trigger: The knight is hit by a melee attack roll\
while holding a weapon. Response: The knight adds 2\
to its AC against that attack, possibly causing it to miss.\

Original layout: [[../../srd-5.2.1/pages/302|page 302]].
