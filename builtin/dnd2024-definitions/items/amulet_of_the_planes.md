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

<!-- splat:yaml path=splat_rule/parameters/check -->

```yaml
ability: intelligence
skill: arcana
dc: 15
success:
  spell:
    key: dnd2024:spells/plane_shift
failure:
  transport:
  - actor
  - creatures
  - objects
  radius_ft: 15
  destination_roll:
    die: 100
    rows:
    - range:
      - 1
      - 60
      plane: named_plane
      location: random
    - range:
      - 61
      - 70
      plane_roll:
        die: 6
        values:
        - air
        - earth
        - fire
        - water
        - feywild
        - shadowfell
      location: random
    - range:
      - 71
      - 80
      plane_roll:
        die: 8
        values:
        - arborea
        - arcadia
        - beastlands
        - bytopia
        - elysium
        - mechanus
        - mount_celestia
        - ysgard
      location: random
    - range:
      - 81
      - 90
      plane_roll:
        die: 8
        values:
        - abyss
        - acheron
        - carceri
        - gehenna
        - hades
        - limbo
        - nine_hells
        - pandemonium
      location: random
    - range:
      - 91
      - 100
      plane: astral
      location: random
```
