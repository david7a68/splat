---
splat_schema: 2
splat_id: c3f9a0ea-e00b-5f87-b496-b9e9185450a7
splat_kind: item
splat_key: dnd2024:items/dust_of_dryness
splat_name: Dust of Dryness
splat_origin: dnd2024:reference/magic_item/dust_of_dryness
splat_sources:
- SRD 5.2.1, pp. 219, https://www.dndbeyond.com/srd
---

# Dust of Dryness


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 219 -->

Wondrous Item, Uncommon

This small packet contains 1d6 + 4 pinches of dust. As a Utilize action, you can sprinkle a pinch of the dust over water, turning up to a 15-foot Cube of water into one marble-sized pellet, which floats or rests near where the dust was sprinkled. The pellet’s weight is negligible. A creature can take a Utilize action to smash the pellet against a hard surface, causing the pellet to shatter and release the water the dust absorbed. Doing so destroys the pellet and ends its magic.

As a Utilize action, you can sprinkle a pinch of the dust on an Elemental within 5 feet of yourself that is composed mostly of water (such as a Water Elemental). Such a creature exposed to a pinch of the dust makes a DC 13 Constitution saving throw, taking 10d6 Necrotic damage on a failed save or half as much damage on a successful one.

Original layout: [[../../srd-5.2.1/pages/219|page 219]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/dust_of_dryness ` |
| ` implementation ` | ` dnd2024:item_dust_of_dryness ` |

### Parameters

#### Pinches

<!-- splat:yaml path=splat_rule/parameters/pinches -->

```yaml
initial:
  dice: 1
  sides: 6
  bonus: 4
recovery: null
```

#### Water absorption

<!-- splat:yaml path=splat_rule/parameters/water_absorption -->

```yaml
action: utilize
cost_pinches: 1
maximum_cube_side_ft: 15
output:
  kind: pellet
  size: marble
  weight: negligible
  placement: near_sprinkled_dust
  floats_or_rests: true
  stores: absorbed_water
release:
  action: utilize
  requires: smash_against_hard_surface
  water: all_stored
  destroy_pellet: true
  end_magic: true
```

#### Elemental damage

<!-- splat:yaml path=splat_rule/parameters/elemental_damage -->

```yaml
action: utilize
cost_pinches: 1
range_ft: 5
target:
  type: elemental
  composition: mostly_water
save:
  ability: constitution
  dc: 13
damage:
  roll:
    dice: 10
    sides: 6
    bonus: 0
  type: necrotic
  success_multiplier: 0.5
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
