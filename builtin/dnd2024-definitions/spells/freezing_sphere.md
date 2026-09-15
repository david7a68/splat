---
splat_schema: 1
splat_id: f8c81e44-6985-58fe-a3fa-c3fda7f60022
splat_kind: spell
splat_key: dnd2024:spells/freezing_sphere
splat_name: Freezing Sphere
splat_origin:
  key: dnd2024:reference/spell/freezing_sphere
splat_sources:
- title: SRD 5.2.1
  locator: pp. 134
  url: https://www.dndbeyond.com/srd
---

# Freezing Sphere

## Source text

<!-- SRD page 134 -->

Level 6 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 300 feet\
Components: V, S, M (a miniature crystal sphere)\
Duration: Instantaneous\
A frigid globe streaks from you to a point of\
your choice within range, where it explodes in a\
60-foot-radius Sphere. Each creature in that area\
makes a Constitution saving throw, taking 10d6\
Cold damage on failed save or half as much damage\
on a successful one.\
If the globe strikes a body of water, it freezes the\
water to a depth of 6 inches over an area 30 feet\
square. This ice lasts for 1 minute. Creatures that\
were swimming on the surface of frozen water are\
trapped in the ice and have the Restrained condi-\
tion. A trapped creature can take an action to make\
a Strength (Athletics) check against your spell save\
DC to break free.\
You can refrain from firing the globe after com-\
pleting the spell’s casting. If you do so, a globe about\
the size of a sling bullet, cool to the touch, appears\
in your hand. At any time, you or a creature you\
give the globe to can throw the globe (to a range of\
40 feet) or hurl it with a sling (to the sling’s normal\
range). It shatters on impact, with the same effect\
as a normal casting of the spell. You can also set the\
globe down without shattering it. After 1 minute, if\
the globe hasn’t already shattered, it explodes.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d6 for each spell slot level above 6.\

Original layout: [[../../srd-5.2.1/pages/134|page 134]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 6 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 300 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a miniature crystal sphere ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/freezing_sphere ` |
| ` implementation ` | ` dnd2024:spell_freezing_sphere ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures_in_area ` |
| ` origin ` | ` chosen_point_in_range ` |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 60 ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
failure:
  damage:
    dice: 10d6
    type: cold
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

#### Water impact

<!-- splat:yaml path=splat_rule/parameters/water_impact -->

```yaml
freeze_depth_inches: 6
square_side_feet: 30
duration_minutes: 1
targets: surface_swimming_creatures
condition: restrained
escape:
  economy: action
  check:
    ability: strength
    skill: athletics
    dc: spell_save_dc
  success: break_free
```

#### Delayed choice

<!-- splat:values path=splat_rule/parameters/delayed_choice -->

| Field | Value |
| --- | --- |
| ` spawn ` | ` caster_hand ` |
| ` size ` | ` sling_bullet ` |
| ` temperature ` | ` cool ` |
| ` transfer_to_other_creature ` | ` true ` |
| ` launch_timing ` | ` any_time ` |
| ` impact ` | ` same_spell_explosion ` |
| ` set_down_without_shattering ` | ` true ` |
| ` automatic_explosion_after_minutes ` | ` 1 ` |

##### Launch options

<!-- splat:table
path: splat_rule/parameters/delayed_choice/launch_options
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Kind:
    path: kind
    type: symbol
  Range feet:
    path: range_feet
    type: integer
    optional: true
  Range:
    path: range
    type: symbol
    optional: true
-->

| Key | Kind | Range feet | Range |
| --- | --- | --- | --- |
| ` row_1 ` | ` Throw ` | ` 40 ` |  |
| ` row_2 ` | ` Sling ` |  | ` Sling normal ` |

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 6 ` |
| ` damage_per_level ` | ` 1d6 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
