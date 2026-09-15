---
splat_schema: 1
splat_id: 04fc51cd-ffc5-5b16-81be-acdde63dd5eb
splat_kind: item
splat_key: dnd2024:items/ammunition_of_slaying
splat_name: Ammunition of Slaying
splat_origin:
  key: dnd2024:reference/magic_item/ammunition_of_slaying
splat_sources:
- title: SRD 5.2.1
  locator: pp. 209
  url: https://www.dndbeyond.com/srd
---

# Ammunition of Slaying

## Source text

<!-- SRD page 209 -->

Weapon (Any Ammunition), Very Rare\
This magic ammunition is meant to slay creatures\
of a particular type, which the GM chooses or de-\
termines randomly by rolling on the table below. If\
a creature of that type takes damage from the am-\
munition, the creature makes a DC 17 Constitution\
saving throw, taking an extra 6d10 Force damage\
on a failed save or half as much extra damage on a\
successful one.\
After dealing its extra damage to a creature, the\
ammunition becomes nonmagical.\
1d100 Creature Type 1d100 Creature Type\
01–10 Aberrations 51–60 Fey\
11–15 Beasts 61–70 Fiends\
16–20 Celestials 71–75 Giants\
21–25 Constructs 76–80 Monstrosities\
26–35 Dragons 81–85 Oozes\
36–45 Elementals 86–90 Plants\
46–50 Humanoids 91–00 Undead\

Original layout: [[../../srd-5.2.1/pages/209|page 209]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` weapon ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ammunition_of_slaying ` |
| ` implementation ` | ` dnd2024:item_ammunition_of_slaying ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` item.damage_dealt ` |
| ` if_target_matches ` | ` true ` |
| ` attunement_required ` | ` false ` |

#### Target type

<!-- splat:yaml path=splat_rule/parameters/target_type -->

```yaml
selection: gm_choice_or_roll
die: 100
rows:
- - 1
  - 10
  - aberration
- - 11
  - 15
  - beast
- - 16
  - 20
  - celestial
- - 21
  - 25
  - construct
- - 26
  - 35
  - dragon
- - 36
  - 45
  - elemental
- - 46
  - 50
  - humanoid
- - 51
  - 60
  - fey
- - 61
  - 70
  - fiend
- - 71
  - 75
  - giant
- - 76
  - 80
  - monstrosity
- - 81
  - 85
  - ooze
- - 86
  - 90
  - plant
- - 91
  - 100
  - undead
columns:
- minimum_roll
- maximum_roll
- creature_type
```

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
dc: 17
failure:
  damage:
    amount:
      roll:
        count: 6
        sides: 10
    type: force
success:
  damage_fraction:
  - 1
  - 2
```

#### After extra damage

<!-- splat:values path=splat_rule/parameters/after_extra_damage -->

| Field | Value |
| --- | --- |
| ` item_magic ` | ` false ` |
