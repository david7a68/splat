---
splat_schema: 1
splat_id: 0aa80369-235f-55b4-97f6-a1479ebe1f5b
splat_kind: creature
splat_key: dnd2024:creatures/treant
splat_name: Treant
splat_origin:
  key: dnd2024:reference/creature/treant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 333
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: plant
  alignment: Chaotic Good
  movement_text: 30 ft.
  resistances_text: Bludgeoning, Piercing
  vulnerabilities_text: Fire
  senses_text: Passive Perception 13
  languages_text: Common, Druidic, Elvish, Sylvan
  cr_text: 9 (XP 5,000; PB +4)
  hit_points_text: 138 (12d12 + 60)
splat_stats:
  strength:
    value: 23
  save_strength:
    value: 6
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 21
  save_constitution:
    value: 5
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 16
  save_wisdom:
    value: 3
  charisma:
    value: 12
  save_charisma:
    value: 1
  armor_class:
    value: 16
  max_hp:
    value: 138
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 8a1f7e7837ea4bcde1e0d59bbfae609e59941f8ced1a6d5639ce9c1d08b50530
  source_path: builtin/srd-5.2.1/creature/treant.md
  pages:
  - 333
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Treant Huge Plant, Chaotic Good AC 16 Initiative +3 (13) HP 138 (12d12 + 60) Speed 30 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 23 +6 +6 Dex 8 -1 -1 Con 21 +5 +5 Int 12 +1 +1 Wis 16 +3 +3 Cha 12 +1 +1
    Vulnerabilities Fire Resistances Bludgeoning, Piercing Senses Passive Perception 13 Languages Common,
    Druidic, Elvish, Sylvan CR 9 (XP 5,000; PB +4) ### Traits Siege Monster. The treant deals double damage
    to objects and structures. ### Actions Multiattack. The treant makes two Slam attacks. Slam. Melee
    Attack Roll: +10, reach 5 ft. Hit: 16 (3d6 + 6) Bludgeoning damage. Hail of Bark. Ranged Attack Roll:
    +10, range 180 ft. Hit: 28 (4d10 + 6) Piercing damage. Animate Trees (1/Day). The treant magically
    animates up to two trees it can see within 60 feet of itself. Each tree uses the Treant stat block,
    except it has Intelligence and Charisma scores of 1, it can’t speak, and it lacks this action. The
    tree takes its turn immediately after the treant on the same Initiative count, and it obeys the treant.
    A tree remains animate for 1 day or until it dies, the treant dies, or it is more than 120 feet from
    the treant. The tree then takes root if possible.'
  sha256: 13346f416ca3c5d2152b23b0020996fb548d0d8def783d3b109b01f62f95e422
  status: needs_semantic_conversion
---

# Treant

<!-- SRD page 333 -->

Huge Plant, Chaotic Good\
AC 16 Initiative +3 (13)\
HP 138 (12d12 + 60)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 8 −1 −1 Con 21 +5 +5\
Int 12 +1 +1 Wis 16 +3 +3 Cha 12 +1 +1\
Vulnerabilities Fire\
Resistances Bludgeoning, Piercing\
Senses Passive Perception 13\
Languages Common, Druidic, Elvish, Sylvan\
CR 9 (XP 5,000; PB +4)\

### Traits

Siege Monster. The treant deals double damage to ob-\
jects and structures.\

### Actions

Multiattack. The treant makes two Slam attacks.\
Slam. Melee Attack Roll: +10, reach 5 ft. Hit: 16 (3d6\
+ 6) Bludgeoning damage.\
Hail of Bark. Ranged Attack Roll: +10, range 180 ft.\
Hit: 28 (4d10 + 6) Piercing damage.\
Animate Trees (1/Day). The treant magically animates\
up to two trees it can see within 60 feet of itself. Each\
tree uses the Treant stat block, except it has Intelligence\
and Charisma scores of 1, it can’t speak, and it lacks\
this action. The tree takes its turn immediately after the\
treant on the same Initiative count, and it obeys the tre-\
ant. A tree remains animate for 1 day or until it dies, the\
treant dies, or it is more than 120 feet from the treant.\
The tree then takes root if possible.\

Original layout: [[../../srd-5.2.1/pages/333|page 333]].
