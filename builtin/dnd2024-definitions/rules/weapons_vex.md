---
splat_schema: 1
splat_id: f3af9c26-a19d-59e7-ac2c-593ee01d30b0
splat_kind: rule
splat_key: dnd2024:rules/weapons_vex
splat_name: Vex
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90, 91
  url: https://www.dndbeyond.com/srd
---

# Vex

## Source text

<!-- SRD page 90 -->

If you hit a creature with this weapon and deal dam-\
age to the creature, you have Advantage on your\
next attack roll against that creature before the end\
of your next turn.\

<!-- SRD page 91 -->

Weapons\
Name Damage Properties\
Simple Melee Weapons\
Club 1d4 Bludgeoning Light\
Dagger 1d4 Piercing Finesse, Light, Thrown (Range 20/60)\
Greatclub 1d8 Bludgeoning Two-Handed\
Handaxe 1d6 Slashing Light, Thrown (Range 20/60)\
Javelin 1d6 Piercing Thrown (Range 30/120)\
Light Hammer 1d4 Bludgeoning Light, Thrown (Range 20/60)\
Mace 1d6 Bludgeoning —\
Quarterstaff 1d6 Bludgeoning Versatile (1d8)\
Sickle 1d4 Slashing Light\
Spear 1d6 Piercing Thrown (Range 20/60), Versatile (1d8)\
Simple Ranged Weapons\
Dart 1d4 Piercing Finesse, Thrown (Range 20/60)\
Light Crossbow 1d8 Piercing Ammunition (Range 80/320; Bolt), Loading,\
Two-Handed\
Shortbow 1d6 Piercing Ammunition (Range 80/320; Arrow), Two-Handed\
Sling 1d4 Bludgeoning Ammunition (Range 30/120; Bullet)\
Martial Melee Weapons\
Battleaxe 1d8 Slashing Versatile (1d10)\
Flail 1d8 Bludgeoning —\
Glaive 1d10 Slashing Heavy, Reach, Two-Handed\
Greataxe 1d12 Slashing Heavy, Two-Handed\
Greatsword 2d6 Slashing Heavy, Two-Handed\
Halberd 1d10 Slashing Heavy, Reach, Two-Handed\
Lance 1d10 Piercing Heavy, Reach, Two-Handed (unless mounted)\
Longsword 1d8 Slashing Versatile (1d10)\
Maul 2d6 Bludgeoning Heavy, Two-Handed\
Morningstar 1d8 Piercing —\
Pike 1d10 Piercing Heavy, Reach, Two-Handed\
Rapier 1d8 Piercing Finesse\
Scimitar 1d6 Slashing Finesse, Light\
Shortsword 1d6 Piercing Finesse, Light\
Trident 1d8 Piercing Thrown (Range 20/60), Versatile (1d10)\
Warhammer 1d8 Bludgeoning Versatile (1d10)\
War Pick 1d8 Piercing Versatile (1d10)\
Whip 1d4 Slashing Finesse, Reach\
Martial Ranged Weapons\
Blowgun 1 Piercing Ammunition (Range 25/100; Needle), Loading\
Hand Crossbow  1d6 Piercing Ammunition (Range 30/120; Bolt), Light, Loading\
Heavy Crossbow 1d10 Piercing Ammunition (Range 100/400; Bolt), Heavy, Loading,\
Two-Handed\
Longbow 1d8 Piercing Ammunition (Range 150/600; Arrow), Heavy,\
Two-Handed\
Musket 1d12 Piercing Ammunition (Range 40/120; Bullet), Loading,\
Two-Handed\
Pistol 1d10 Piercing Ammunition (Range 30/90; Bullet), Loading\
Mastery Weight Cost\
Slow 2 lb. 1 SP\
Nick 1 lb. 2 GP\
Push 10 lb. 2 SP\
Vex 2 lb. 5 GP\
Slow 2 lb. 5 SP\
Nick 2 lb. 2 GP\
Sap 4 lb. 5 GP\
Topple 4 lb. 2 SP\
Nick 2 lb. 1 GP\
Sap 3 lb. 1 GP\
Vex 1/4 lb. 5 CP\
Slow 5 lb. 25 GP\
Vex 2 lb. 25 GP\
Slow — 1 SP\
Topple 4 lb. 10 GP\
Sap 2 lb. 10 GP\
Graze 6 lb. 20 GP\
Cleave 7 lb. 30 GP\
Graze 6 lb. 50 GP\
Cleave 6 lb. 20 GP\
Topple 6 lb. 10 GP\
Sap 3 lb. 15 GP\
Topple 10 lb. 10 GP\
Sap 4 lb. 15 GP\
Push 18 lb. 5 GP\
Vex 2 lb. 25 GP\
Nick 3 lb. 25 GP\
Vex 2 lb. 10 GP\
Topple 4 lb. 5 GP\
Push 5 lb. 15 GP\
Sap 2 lb. 5 GP\
Slow 3 lb. 2 GP\
Vex 1 lb. 10 GP\
Vex 3 lb. 75 GP\
Push 18 lb. 50 GP\
Slow 2 lb. 50 GP\
Slow 10 lb. 500 GP\
Vex 3 lb. 250 GP\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_vex ` |
| ` implementation ` | ` dnd2024:rule_weapons_vex ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` weapon_hits_and_damages_creature ` |

#### Effect

<!-- splat:values path=splat_rule/parameters/effect -->

| Field | Value |
| --- | --- |
| ` attacker_next_attack_against_same_creature ` | ` advantage ` |
| ` expires ` | ` end_of_attacker_next_turn ` |

#### Interleaved weapon table

<!-- splat:yaml path=splat_rule/parameters/interleaved_weapon_table -->

```yaml
entries:
- key: dnd2024:equipment/club
- key: dnd2024:equipment/dagger
- key: dnd2024:equipment/greatclub
- key: dnd2024:equipment/handaxe
- key: dnd2024:equipment/javelin
- key: dnd2024:equipment/light_hammer
- key: dnd2024:equipment/mace
- key: dnd2024:equipment/quarterstaff
- key: dnd2024:equipment/sickle
- key: dnd2024:equipment/spear
- key: dnd2024:equipment/dart
- key: dnd2024:equipment/light_crossbow
- key: dnd2024:equipment/shortbow
- key: dnd2024:equipment/sling
- key: dnd2024:equipment/battleaxe
- key: dnd2024:equipment/flail
- key: dnd2024:equipment/glaive
- key: dnd2024:equipment/greataxe
- key: dnd2024:equipment/greatsword
- key: dnd2024:equipment/halberd
- key: dnd2024:equipment/lance
- key: dnd2024:equipment/longsword
- key: dnd2024:equipment/maul
- key: dnd2024:equipment/morningstar
- key: dnd2024:equipment/pike
- key: dnd2024:equipment/rapier
- key: dnd2024:equipment/scimitar
- key: dnd2024:equipment/shortsword
- key: dnd2024:equipment/trident
- key: dnd2024:equipment/warhammer
- key: dnd2024:equipment/war_pick
- key: dnd2024:equipment/whip
- key: dnd2024:equipment/blowgun
- key: dnd2024:equipment/hand_crossbow
- key: dnd2024:equipment/heavy_crossbow
- key: dnd2024:equipment/longbow
- key: dnd2024:equipment/musket
- key: dnd2024:equipment/pistol
columns:
- category
- damage
- damage_type
- properties
- mastery
- weight
- cost
```
