---
splat_schema: 1
splat_id: ee0ba5e2-169f-53ab-a27c-33082a67a6e3
splat_kind: item
splat_key: dnd2024:items/energy_bow
splat_name: Energy Bow
splat_origin:
  key: dnd2024:reference/magic_item/energy_bow
splat_sources:
- title: SRD 5.2.1
  locator: pp. 220
  url: https://www.dndbeyond.com/srd
---

# Energy Bow

## Source text

<!-- SRD page 220 -->

Weapon (Longbow or Shortbow), Very Rare (Requires\
Attunement)\
You gain a +1 bonus to attack rolls and damage rolls\
made with this magic weapon, which has no string.\
Each time you pull your arm back in a firing motion,\
a magical arrow made of golden energy appears\
nocked and ready to fire. An arrow produced by\
this weapon deals Force damage instead of Pierc-\
ing damage on a hit, and it disappears after it hits\
or misses its target. Until it disappears, the arrow\
emits Bright Light in a 20-foot radius and Dim Light\
for an additional 20 feet.\
This weapon has the following additional\
properties.\
Arrow of Restraint. Whenever you use this\
weapon to make a ranged attack against a creature,\
you can try to restrain the target instead of deal-\
ing damage to it. If the arrow hits, the target must\
succeed on a DC 15 Strength saving throw or have\
the Restrained condition for 1 minute. As an action,\
a creature Restrained by an arrow can make a DC\
20 Strength (Athletics) check to try to break the\
restraint, ending the effect on itself on a successful\
check.\
Arrow of Transport. As a Magic action, you can\
fire one energy arrow from this weapon at a target\
you can see within 60 feet of yourself. The target\
can be either a willing Medium or smaller creature\
or an object that isn’t being worn or carried, pro-\
vided the object is small enough to fit inside a 5-foot\
Cube. The arrow teleports the target to an unoccu-\
pied space you can see within 10 feet of you.\
Energy Ladder. As a Magic action, you can loose a\
flurry of energy arrows from this weapon at a wall\
up to 60 feet away from yourself. The arrows be-\
come glowing rungs that stick out of the wall, form-\
ing a magical ladder up to 60 feet long on the wall.\
This ladder lasts for 1 minute before disappearing.\

Original layout: [[../../srd-5.2.1/pages/220|page 220]].

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
| ` mechanic ` | ` dnd2024:items/energy_bow ` |
| ` implementation ` | ` dnd2024:item_energy_bow ` |

### Parameters

#### Weapon bonus

<!-- splat:values path=splat_rule/parameters/weapon_bonus -->

| Field | Value |
| --- | --- |
| ` attack ` | ` 1 ` |
| ` damage ` | ` 1 ` |

#### Ammunition

<!-- splat:yaml path=splat_rule/parameters/ammunition -->

```yaml
requires_string: false
generation_trigger: firing_motion
kind: energy_arrow
damage_type: force
replaces_damage_type: piercing
disappears_on:
- hit
- miss
light:
  bright_radius_ft: 20
  additional_dim_ft: 20
```

#### Arrow of restraint

<!-- splat:yaml path=splat_rule/parameters/arrow_of_restraint -->

```yaml
trigger: ranged_attack_against_creature
replace_damage: true
on_hit:
  save:
    ability: strength
    dc: 15
  failure:
    condition: restrained
    duration:
      value: 1
      unit: minute
escape:
  actor: restrained_target
  action: action
  check:
    ability: strength
    skill: athletics
    dc: 20
  success: end_restraint
```

#### Arrow of transport

<!-- splat:yaml path=splat_rule/parameters/arrow_of_transport -->

```yaml
action: magic
range_ft: 60
target_seen: true
eligible:
  any_of:
  - kind: creature
    willing: true
    maximum_size: medium
  - kind: object
    worn_or_carried: false
    fits_cube_ft: 5
destination:
  range_from_user_ft: 10
  seen: true
  unoccupied: true
effect: teleport
attack_roll_required: false
```

#### Energy ladder

<!-- splat:yaml path=splat_rule/parameters/energy_ladder -->

```yaml
action: magic
wall_range_ft: 60
maximum_length_ft: 60
duration:
  value: 1
  unit: minute
rungs: glowing_energy_arrows
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
