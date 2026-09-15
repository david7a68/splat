---
splat_schema: 1
splat_id: dcf124f1-da50-58b9-a70f-54dd63b8302d
splat_kind: item
splat_key: dnd2024:items/amulet_of_the_planes
splat_name: Amulet of the Planes
splat_origin:
  key: dnd2024:reference/magic_item/amulet_of_the_planes
splat_sources:
- title: SRD 5.2.1
  locator: pp. 209
  url: https://www.dndbeyond.com/srd
---

# Amulet of the Planes

## Source text

<!-- SRD page 209 -->

Wondrous Item, Very Rare (Requires Attunement)\
While wearing this amulet, you can take a Magic\
action to name a location that you are familiar with\
on another plane of existence. Then make a DC 15\
Intelligence (Arcana) check. On a successful check,\
you cast Plane Shift. On a failed check, you and each\
creature and object within 15 feet of you travel to\
a random destination determined by rolling 1d100\
and consulting the following table.\
1d100 Destination\
01–60 Random location on the plane you named\
61–70 Random location on an Inner Plane deter-\
mined by rolling 1d6: on a 1, the Plane of Air;\
on a 2, the Plane of Earth; on a 3, the Plane of\
Fire; on a 4, the Plane of Water; on a 5, the\
Feywild; on a 6, the Shadowfell\
71–80 Random location on an Outer Plane deter-\
mined by rolling 1d8: on a 1, Arborea; on a 2,\
Arcadia; on a 3, the Beastlands; on a 4, Byto-\
pia; on a 5, Elysium; on a 6, Mechanus; on a 7,\
Mount Celestia; on an 8, Ysgard\
81–90 Random location on an Outer Plane deter-\
mined by rolling 1d8: on a 1, the Abyss; on a\
2, Acheron; on a 3, Carceri; on a 4, Gehenna;\
on a 5, Hades; on a 6, Limbo; on a 7, the Nine\
Hells; on an 8, Pandemonium\
91–00 Random location on the Astral Plane\

Original layout: [[../../srd-5.2.1/pages/209|page 209]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/amulet_of_the_planes ` |
| ` implementation ` | ` dnd2024:item_amulet_of_the_planes ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` action ` | ` magic ` |
| ` attunement_required ` | ` true ` |

#### Destination

<!-- splat:values path=splat_rule/parameters/destination -->

| Field | Value |
| --- | --- |
| ` type ` | ` location ` |
| ` familiar ` | ` true ` |
| ` plane ` | ` other_than_current ` |

#### Check

<!-- splat:values path=splat_rule/parameters/check -->

| Field | Value |
| --- | --- |
| ` ability ` | ` intelligence ` |
| ` skill ` | ` arcana ` |
| ` dc ` | ` 15 ` |

##### Success

<!-- splat:yaml path=splat_rule/parameters/check/success -->

```yaml
spell:
  key: dnd2024:spells/plane_shift
```

##### Failure

<!-- splat:values path=splat_rule/parameters/check/failure -->

| Field | Value |
| --- | --- |
| ` transport ` | ` ["actor", "creatures", "objects"] ` |
| ` radius_ft ` | ` 15 ` |

###### Destination roll

<!-- splat:values path=splat_rule/parameters/check/failure/destination_roll -->

| Field | Value |
| --- | --- |
| ` die ` | ` 100 ` |

###### Rows

<!-- splat:table
path: splat_rule/parameters/check/failure/destination_roll/rows
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
columns:
  Key:
    type: key
  Range:
    path: range
    type: yaml
  Plane:
    path: plane
    type: symbol
    optional: true
  Location:
    path: location
    type: symbol
  Plane roll die:
    path: plane_roll/die
    type: integer
    optional: true
  Details:
    type: section
    optional: true
-->

| Key | Range | Plane | Location | Plane roll die | Details |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` [1, 60] ` | ` Named plane ` | ` Random ` |  |  |
| ` row_2 ` | ` [61, 70] ` |  | ` Random ` | ` 6 ` | [Rules](#^splat-2ba9a9c2-row_2 "section:row_2") |
| ` row_3 ` | ` [71, 80] ` |  | ` Random ` | ` 8 ` | [Rules](#^splat-2ba9a9c2-row_3 "section:row_3") |
| ` row_4 ` | ` [81, 90] ` |  | ` Random ` | ` 8 ` | [Rules](#^splat-2ba9a9c2-row_4 "section:row_4") |
| ` row_5 ` | ` [91, 100] ` | ` Astral ` | ` Random ` |  |  |

###### Row 2

^splat-2ba9a9c2-row_2

###### Values

<!-- splat:yaml path=splat_rule/parameters/check/failure/destination_roll/rows/@row_2/plane_roll/values -->

```yaml
- air
- earth
- fire
- water
- feywild
- shadowfell
```

###### Row 3

^splat-2ba9a9c2-row_3

###### Values

<!-- splat:yaml path=splat_rule/parameters/check/failure/destination_roll/rows/@row_3/plane_roll/values -->

```yaml
- arborea
- arcadia
- beastlands
- bytopia
- elysium
- mechanus
- mount_celestia
- ysgard
```

###### Row 4

^splat-2ba9a9c2-row_4

###### Values

<!-- splat:yaml path=splat_rule/parameters/check/failure/destination_roll/rows/@row_4/plane_roll/values -->

```yaml
- abyss
- acheron
- carceri
- gehenna
- hades
- limbo
- nine_hells
- pandemonium
```
