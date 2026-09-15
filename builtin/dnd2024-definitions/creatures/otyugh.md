---
splat_schema: 1
splat_id: 433c4671-5681-5c3f-9a38-f7eca923b6d5
splat_kind: creature
splat_key: dnd2024:creatures/otyugh
splat_name: Otyugh
splat_origin:
  key: dnd2024:reference/creature/otyugh
splat_sources:
- title: SRD 5.2.1
  locator: pp. 313
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: aberration
  alignment: Neutral
  movement_text: 30 ft.
  senses_text: Darkvision 120 ft.; Passive Perception 11
  languages_text: Otyugh; telepathy 120 ft. (doesn’t allow the receiving creature to respond telepathically)
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 104 (11d10 + 44)
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
    value: 19
  save_constitution:
    value: 7
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 14
  max_hp:
    value: 104
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
  source_sha256: 7df039a0501bd31fdd1c1323973047d6dcc705fd97c309efcd7b8948c8323d72
  source_path: builtin/srd-5.2.1/creature/otyugh.md
  pages:
  - 313
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Otyugh Large Aberration, Neutral AC 14 Initiative +0 (10) HP 104 (11d10 + 44) Speed 30 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 11 +0 +0 Con 19 +4 +7 Int 6 -2 -2 Wis 13 +1 +1 Cha 6 -2
    -2 Senses Darkvision 120 ft.; Passive Perception 11 Languages Otyugh; telepathy 120 ft. (doesn’t allow
    the receiving creature to respond telepathically) CR 5 (XP 1,800; PB +3) ### Actions Multiattack.
    The otyugh makes one Bite attack and two Tentacle attacks. Bite. Melee Attack Roll: +6, reach 5 ft.
    Hit: 12 (2d8 + 3) Piercing damage, and the target has the Poisoned condition. Whenever the Poisoned
    target finishes a Long Rest, it is subjected to the following effect. Constitution Saving Throw: DC
    15. Failure: The target’s Hit Point maximum decreases by 5 (1d10) and doesn’t return to normal until
    the Poisoned condition ends on the target. Success: The Poisoned condition ends. Tentacle. Melee Attack
    Roll: +6, reach 10 ft. Hit: 12 (2d8 + 3) Piercing damage. If the target is a Medium or smaller creature,
    it has the Grappled condition (escape DC 13) from one of two tentacles. Tentacle Slam. Constitution
    Saving Throw: DC 14, each creature Grappled by the otyugh. Failure: 16 (3d8 + 3) Bludgeoning damage,
    and the target has the Stunned condition until the start of the otyugh’s next turn. Success: Half
    damage only.'
  sha256: 32cef42de27fbd202fcd98635a17aae84a528dbb9be8ae710b21bdcc36de22a4
  status: needs_semantic_conversion
---

# Otyugh

<!-- SRD page 313 -->

Large Aberration, Neutral\
AC 14 Initiative +0 (10)\
HP 104 (11d10 + 44)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 11 +0 +0 Con 19 +4 +7\
Int 6 −2 −2 Wis 13 +1 +1 Cha 6 −2 −2\
Senses Darkvision 120 ft.; Passive Perception 11\
Languages Otyugh; telepathy 120 ft. (doesn’t allow the\
receiving creature to respond telepathically)\
CR 5 (XP 1,800; PB +3)\

### Actions

Multiattack. The otyugh makes one Bite attack and two\
Tentacle attacks.\
Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 12 (2d8 + 3)\
Piercing damage, and the target has the Poisoned con-\
dition. Whenever the Poisoned target finishes a Long\
Rest, it is subjected to the following effect. Constitution\
Saving Throw: DC 15. Failure: The target’s Hit Point\
maximum decreases by 5 (1d10) and doesn’t return to\
normal until the Poisoned condition ends on the target.\
Success: The Poisoned condition ends.\
Tentacle. Melee Attack Roll: +6, reach 10 ft. Hit: 12\
(2d8 + 3) Piercing damage. If the target is a Medium or\
smaller creature, it has the Grappled condition (escape\
DC 13) from one of two tentacles.\
Tentacle Slam. Constitution Saving Throw: DC 14, each\
creature Grappled by the otyugh. Failure: 16 (3d8 + 3)\
Bludgeoning damage, and the target has the Stunned\
condition until the start of the otyugh’s next turn. Suc-\
cess: Half damage only.\

Original layout: [[../../srd-5.2.1/pages/313|page 313]].
