---
splat_schema: 1
splat_id: c2d1bac7-a3a3-56c6-9f6b-1acc90a0d9b6
splat_kind: item
splat_key: dnd2024:items/bead_of_force
splat_name: Bead of Force
splat_origin:
  key: dnd2024:reference/magic_item/bead_of_force
splat_sources:
- title: SRD 5.2.1
  locator: pp. 212, 213
  url: https://www.dndbeyond.com/srd
---

# Bead of Force

## Source text

<!-- SRD page 212 -->

Wondrous Item, Rare\
This small black sphere measures 3/4 of an inch in\
diameter and weighs an ounce. Typically, 1d4 + 4\
Beads of Force are found together.\
You can take a Magic action to throw the bead up\
to 60 feet. The bead explodes in a 10-foot-radius\
Sphere on impact and is destroyed. Each creature\

<!-- SRD page 213 -->

in the Sphere must succeed on a DC 15 Dexterity\
saving throw or take 5d4 Force damage. A sphere\
of transparent force then encloses the area for 1\
minute. Any creature that failed the save and is\
completely within the area is trapped inside this\
sphere. Creatures that succeeded on the save or are\
partially within the area are pushed away from the\
center of the sphere until they are no longer inside\
it. Only breathable air can pass through the sphere’s\
wall. No attack or other effect can pass through.\
An enclosed creature can take a Utilize action to\
push against the sphere’s wall, moving the sphere\
up to half the creature’s Speed. The sphere can be\
picked up, and its magic causes it to weigh only 1\
pound, regardless of the weight of creatures inside.\

Original layout: [[../../srd-5.2.1/pages/212|page 212]], [[../../srd-5.2.1/pages/213|page 213]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/bead_of_force ` |
| ` implementation ` | ` dnd2024:item_bead_of_force ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` diameter_inches ` | ` 0.75 ` |
| ` weight_oz ` | ` 1 ` |
| ` action ` | ` magic ` |
| ` throw_range_ft ` | ` 60 ` |
| ` consumed_on ` | ` impact ` |
| ` attunement_required ` | ` false ` |

#### Found quantity

<!-- splat:yaml path=splat_rule/parameters/found_quantity -->

```yaml
roll:
  count: 1
  sides: 4
  add: 4
```

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` sphere ` |
| ` radius_ft ` | ` 10 ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: dexterity
dc: 15
failure:
  damage:
    amount:
      roll:
        count: 5
        sides: 4
    type: force
success:
  damage: 0
```

#### Barrier

<!-- splat:values path=splat_rule/parameters/barrier -->

| Field | Value |
| --- | --- |
| ` transparent ` | ` true ` |
| ` passable ` | ` ["breathable_air"] ` |
| ` blocks ` | ` ["attacks", "other_effects", "other_matter"] ` |
| ` can_be_picked_up ` | ` true ` |
| ` weight_lb ` | ` 1 ` |
| ` weight_includes_contents ` | ` true ` |

##### Duration

<!-- splat:values path=splat_rule/parameters/barrier/duration -->

| Field | Value |
| --- | --- |
| ` amount ` | ` 1 ` |
| ` unit ` | ` minutes ` |

##### Trapped

<!-- splat:values path=splat_rule/parameters/barrier/trapped -->

| Field | Value |
| --- | --- |
| ` save ` | ` failure ` |
| ` wholly_inside ` | ` true ` |

##### Push out

<!-- splat:values path=splat_rule/parameters/barrier/push_out -->

| Field | Value |
| --- | --- |
| ` direction ` | ` away_from_center ` |
| ` until ` | ` outside ` |

###### Or

<!-- splat:table
path: splat_rule/parameters/barrier/push_out/or
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Save:
    path: save
    type: symbol
    optional: true
  Partially inside:
    path: partially_inside
    type: boolean
    optional: true
-->

| Key | Save | Partially inside |
| --- | --- | --- |
| ` row_1 ` | ` Success ` |  |
| ` row_2 ` |  | true |

##### Movement

<!-- splat:yaml path=splat_rule/parameters/barrier/movement -->

```yaml
actor: enclosed_creature
action: utilize
maximum_distance:
  multiply:
  - actor.speed
  - 0.5
```
