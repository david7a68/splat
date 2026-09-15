---
splat_schema: 1
splat_id: a00af030-27f8-5d34-99f9-5b74dc3ce8e9
splat_kind: creature
splat_key: dnd2024:creatures/stone_giant
splat_name: Stone Giant
splat_origin:
  key: dnd2024:reference/creature/stone_giant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 330
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: giant
  alignment: Neutral
  movement_text: 40 ft.
  skills_text: Athletics +12, Perception +4, Stealth +5
  senses_text: Darkvision 60 ft.; Passive Perception 14
  languages_text: Giant
  cr_text: 7 (XP 2,900; PB +3)
  hit_points_text: 126 (11d12 + 55)
splat_stats:
  strength:
    value: 23
  save_strength:
    value: 6
  dexterity:
    value: 15
  save_dexterity:
    value: 5
  constitution:
    value: 20
  save_constitution:
    value: 8
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 12
  save_wisdom:
    value: 4
  charisma:
    value: 9
  save_charisma:
    value: -1
  armor_class:
    value: 17
  max_hp:
    value: 126
  initiative:
    value: 5
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: ef86f39112946feddd880874e0723bf53bbc4b70c9a0dca8c0ed8b4cd7687bcb
  source_path: builtin/srd-5.2.1/creature/stone_giant.md
  pages:
  - 330
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Stone Giant Huge Giant, Neutral AC 17 Initiative +5 (15) HP 126 (11d12 + 55) Speed 40 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 23 +6 +6 Dex 15 +2 +5 Con 20 +5 +8 Int 10 +0 +0 Wis 12 +1 +4 Cha 9 -1 -1
    Skills Athletics +12, Perception +4, Stealth +5 Senses Darkvision 60 ft.; Passive Perception 14 Languages
    Giant CR 7 (XP 2,900; PB +3) ### Actions Multiattack. The giant makes two attacks, using Stone Club
    or Boulder in any combination. Stone Club. Melee Attack Roll: +9, reach 15 ft. Hit: 22 (3d10 + 6)
    Bludgeoning damage. Boulder. Ranged Attack Roll: +9, range 60/240 ft. Hit: 15 (2d8 + 6) Bludgeoning
    damage. If the target is a Large or smaller creature, it has the Prone condition. ### Reactions Deflect
    Missile (Recharge 5–6). Trigger: The giant is hit by a ranged attack roll and takes Bludgeoning, Piercing,
    or Slashing damage from it. Response: The giant reduces the damage it takes from the attack by 11
    (1d10 + 6), and if that damage is reduced to 0, the giant can redirect some of the attack’s force.
    Dexterity Saving Throw: DC 17, one creature the giant can see within 60 feet. Failure: 11 (1d10 +
    6) Force damage.'
  sha256: 432f0e370c3c581417ddc59e6c33b508a2be776d91d443f8148ce014ac5185ae
  status: needs_semantic_conversion
---

# Stone Giant

<!-- SRD page 330 -->

Huge Giant, Neutral\
AC 17 Initiative +5 (15)\
HP 126 (11d12 + 55)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 15 +2 +5 Con 20 +5 +8\
Int 10 +0 +0 Wis 12 +1 +4 Cha 9 −1 −1\
Skills Athletics +12, Perception +4, Stealth +5\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages Giant\
CR 7 (XP 2,900; PB +3)\

### Actions

Multiattack. The giant makes two attacks, using Stone\
Club or Boulder in any combination.\
Stone Club. Melee Attack Roll: +9, reach 15 ft. Hit: 22\
(3d10 + 6) Bludgeoning damage.\
Boulder. Ranged Attack Roll: +9, range 60/240 ft. Hit:\
15 (2d8 + 6) Bludgeoning damage. If the target is a\
Large or smaller creature, it has the Prone condition.\

### Reactions

Deflect Missile (Recharge 5–6). Trigger: The giant is hit\
by a ranged attack roll and takes Bludgeoning, Pierc-\
ing, or Slashing damage from it. Response: The giant\
reduces the damage it takes from the attack by 11 (1d10\
+ 6), and if that damage is reduced to 0, the giant can\
redirect some of the attack’s force. Dexterity Saving\
Throw: DC 17, one creature the giant can see within 60\
feet. Failure: 11 (1d10 + 6) Force damage.\

Original layout: [[../../srd-5.2.1/pages/330|page 330]].
