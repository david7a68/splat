---
splat_schema: 1
splat_id: 4d6e7068-1a87-5043-864b-72f2bd1dc6fd
splat_kind: item
splat_key: dnd2024:items/thunderous_greatclub
splat_name: Thunderous Greatclub
splat_origin:
  key: dnd2024:reference/magic_item/thunderous_greatclub
splat_sources:
- title: SRD 5.2.1
  locator: pp. 249
  url: https://www.dndbeyond.com/srd
---

# Thunderous Greatclub

## Source text

<!-- SRD page 249 -->

Weapon (Greatclub), Very Rare (Requires Attunement)

While you are attuned to this magic weapon, your Strength is 20 unless your Strength is already equal to or greater than that score. The weapon deals an extra 1d8 Thunder damage to any creature it hits and an extra 3d8 Thunder damage to objects it hits that aren’t being worn or carried.

The weapon has the following additional properties.

Clap of Thunder. As a Magic action, you can strike the weapon against a hard surface to create a loud clap of thunder audible out to 300 feet. You also create a 30-foot Cone of thunderous energy. Each creature in the Cone must succeed on a DC 15 Strength saving throw or have the Prone condition. Nonmagical objects in the Cone that aren’t being worn or carried take 3d8 Thunder damage.

Earthquake. As a Magic action, you can strike the weapon against the ground to create an intense seismic disturbance in a 50-foot-radius circle centered on the point of impact. Structures in contact with the ground in that area take 50 Bludgeoning damage, and each creature on the ground in that area must succeed on a DC 20 Dexterity saving throw or have the Prone condition. If that creature is also concentrating, it must succeed on a DC 20 Constitution saving throw, or its Concentration is broken. In addition, you can cause a 30-foot-deep, 10-foot-wide fissure to open up on the ground anywhere in the area. Any creature on a spot where the fissure opens must make a DC 20 Dexterity saving throw, falling into the fissure on a failed save or moving with the fissure’s edge on a successful one. Any structure on a spot where the fissure opens collapses into the fissure. Once you use this property, it can’t be used again until the next dawn.

Original layout: [[../../srd-5.2.1/pages/249|page 249]].

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
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/thunderous_greatclub ` |
| ` implementation ` | ` dnd2024:item_thunderous_greatclub ` |

### Parameters

#### Attuned

<!-- splat:yaml path=splat_rule/parameters/attuned -->

```yaml
strength:
  set: 20
  only_if_unmodified_score_below: 20
```

#### On hit

<!-- splat:yaml path=splat_rule/parameters/on_hit -->

```yaml
creature_extra_damage:
  roll:
    dice: 1
    sides: 8
    bonus: 0
  type: thunder
unworn_uncarried_object_extra_damage:
  roll:
    dice: 3
    sides: 8
    bonus: 0
  type: thunder
```

#### Clap of thunder

<!-- splat:yaml path=splat_rule/parameters/clap_of_thunder -->

```yaml
action: magic
requires: strike_hard_surface
audible_range_ft: 300
shape: cone
length_ft: 30
creatures:
  save:
    ability: strength
    dc: 15
  failure_condition: prone
objects:
  nonmagical: true
  worn_or_carried: false
  damage:
    roll:
      dice: 3
      sides: 8
      bonus: 0
    type: thunder
```

#### Earthquake

<!-- splat:yaml path=splat_rule/parameters/earthquake -->

```yaml
action: magic
requires: strike_ground
resource:
  uses: 1
  recovery: dawn
shape: circle
radius_ft: 50
origin: impact_point
ground_contact_structures_damage:
  amount: 50
  type: bludgeoning
ground_creatures:
  save:
    ability: dexterity
    dc: 20
  failure_condition: prone
  if_concentrating:
    save:
      ability: constitution
      dc: 20
    failure: concentration_ends
optional_fissure:
  depth_ft: 30
  width_ft: 10
  placement: user_choice_within_area
  creatures_at_opening:
    save:
      ability: dexterity
      dc: 20
    failure: fall_into_fissure
    success: move_with_edge
  structures_at_opening: collapse_into_fissure
  length:
    source_specified: false
    input: chosen_fissure_location
    constraint: within_earthquake_area
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
