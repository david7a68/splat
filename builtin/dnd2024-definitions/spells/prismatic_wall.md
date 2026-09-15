---
splat_schema: 1
splat_id: 2618e55d-2863-5233-8639-f3def3fded3d
splat_kind: spell
splat_key: dnd2024:spells/prismatic_wall
splat_name: Prismatic Wall
splat_origin:
  key: dnd2024:reference/spell/prismatic_wall
splat_sources:
- title: SRD 5.2.1
  locator: pp. 155
  url: https://www.dndbeyond.com/srd
---

# Prismatic Wall

## Source text

<!-- SRD page 155 -->

Level 9 Abjuration (Bard, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: 10 minutes\
A shimmering, multicolored plane of light forms\
a vertical opaque wall—up to 90 feet long, 30 feet\
high, and 1 inch thick—centered on a point within\
range. Alternatively, you shape the wall into a globe\
up to 30 feet in diameter centered on a point within\
range. The wall lasts for the duration. If you posi-\
tion the wall in a space occupied by a creature, the\
spell ends instantly without effect.\
The wall sheds Bright Light within 100 feet and\
Dim Light for an additional 100 feet. You and crea-\
tures you designate when you cast the spell can\
pass through and be near the wall without harm. If\
another creature that can see the wall moves within\
20 feet of it or starts its turn there, the creature\
must succeed on a Constitution saving throw or\
have the Blinded condition for 1 minute.\
The wall consists of seven layers, each with a\
different color. When a creature reaches into or\
passes through the wall, it does so one layer at a\
time through all the layers. Each layer forces the\
creature to make a Dexterity saving throw or be af-\
fected by that layer’s properties as described in the\
Prismatic Layers table.\
The wall, which has AC 10, can be destroyed one\
layer at a time, in order from red to violet, by means\
specific to each layer. If a layer is destroyed, it is\
gone for the duration. Antimagic Field has no effect\
on the wall, and Dispel Magic can affect only the vio-\
let layer.\
Prismatic Layers\
Order Effects\
1 Red. Failed Save: 12d6 Fire damage. Successful\
Save: Half as much damage. Additional Effects:\
Nonmagical ranged attacks can’t pass through\
this layer, which is destroyed if it takes at\
least 25 Cold damage.\
2 Orange. Failed Save: 12d6 Acid damage. Suc-\
cessful Save: Half as much damage. Additional\
Effects: Magical ranged attacks can’t pass\
through this layer, which is destroyed by a\
strong wind (such as the one created by Gust\
of Wind).\
3 Yellow. Failed Save: 12d6 Lightning damage.\
Successful Save: Half as much damage. Addi-\
tional Effects: The layer is destroyed if it takes\
at least 60 Force damage.\
4 Green. Failed Save: 12d6 Poison damage. Suc-\
cessful Save: Half as much damage. Additional\
Effects: A Passwall spell, or another spell of\
equal or greater level that can open a portal\
on a solid surface, destroys this layer.\
5 Blue. Failed Save: 12d6 Cold damage. Success-\
ful Save: Half as much damage. Additional Ef-\
fects: The layer is destroyed if it takes at least\
25 Fire damage.\
6 Indigo. Failed Save: The target has the Re-\
strained condition and makes a Constitution\
saving throw at the end of each of its turns. If\
it successfully saves three times, the condition\
ends. If it fails three times, it has the Petrified\
condition until it is freed by an effect like the\
Greater Restoration spell. The successes and\
failures needn’t be consecutive; keep track of\
both until the target collects three of a kind.\
Additional Effects: Spells can’t be cast through\
this layer, which is destroyed by Bright Light\
shed by the Daylight spell.\
7 Violet. Failed Save: The target has the Blinded\
condition and makes a Wisdom saving throw\
at the start of your next turn. On a successful\
save, the condition ends. On a failed save, the\
condition ends, and the creature teleports\
to another plane of existence (GM’s choice).\
Additional Effects: This layer is destroyed by\
Dispel Magic.\

Original layout: [[../../srd-5.2.1/pages/155|page 155]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 9 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["bard", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 10 minutes ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/prismatic_wall ` |
| ` implementation ` | ` dnd2024:spell_prismatic_wall ` |

### Parameters

#### Geometry choice

<!-- splat:yaml path=splat_rule/parameters/geometry_choice -->

```yaml
wall:
  vertical: true
  maximum_length_feet: 90
  maximum_height_feet: 30
  thickness_inches: 1
globe:
  maximum_diameter_feet: 30
```

#### Geometry

<!-- splat:values path=splat_rule/parameters/geometry -->

| Field | Value |
| --- | --- |
| ` center ` | ` point_in_range ` |
| ` opaque ` | ` true ` |
| ` creature_occupied_at_creation ` | ` instant_end_no_effect ` |

#### Light

<!-- splat:values path=splat_rule/parameters/light -->

| Field | Value |
| --- | --- |
| ` bright_radius_feet ` | ` 100 ` |
| ` additional_dim_feet ` | ` 100 ` |

#### Exempt

<!-- splat:values path=splat_rule/parameters/exempt -->

| Field | Value |
| --- | --- |
| ` caster ` | ` true ` |
| ` designated_creatures_at_cast ` | ` true ` |
| ` near_and_pass_through_unharmed ` | ` true ` |

#### Near effect

<!-- splat:yaml path=splat_rule/parameters/near_effect -->

```yaml
triggers:
- creature.moves_within_20_feet
- turn.start
can_see_wall_required: true
save:
  ability: constitution
  dc:
    binding: save_dc
  failure:
    condition: blinded
    minutes: 1
  success:
    effect: none
```

#### Passage

<!-- splat:values path=splat_rule/parameters/passage -->

| Field | Value |
| --- | --- |
| ` triggers ` | ` ["creature.reaches_into_wall", "creature.passes_through_wall"] ` |
| ` sequential_layers ` | ` true ` |
| ` save_per_layer ` | ` dexterity ` |

#### Destruction

<!-- splat:values path=splat_rule/parameters/destruction -->

| Field | Value |
| --- | --- |
| ` ac ` | ` 10 ` |
| ` one_layer_at_time ` | ` true ` |
| ` order ` | ` ["red", "orange", "yellow", "green", "blue", "indigo", "violet"] ` |
| ` destroyed_layer_absent_until_spell_end ` | ` true ` |
| ` antimagic_field_effect ` | ` false ` |
| ` dispel_magic_layers ` | ` ["violet"] ` |

#### Layers

<!-- splat:rows path=splat_rule/parameters/layers -->

| Row | order | color | blocks |
| --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` red ` | ` nonmagical_ranged_attacks ` |
| ` 1 ` | ` 2 ` | ` orange ` | ` magical_ranged_attacks ` |
| ` 2 ` | ` 3 ` | ` yellow ` |  |
| ` 3 ` | ` 4 ` | ` green ` |  |
| ` 4 ` | ` 5 ` | ` blue ` |  |
| ` 5 ` | ` 6 ` | ` indigo ` | ` spells_cast_through ` |
| ` 6 ` | ` 7 ` | ` violet ` |  |

##### Save

<!-- splat:yaml path=splat_rule/parameters/layers/0/save -->

```yaml
ability: dexterity
dc:
  binding: save_dc
failure:
  damage:
    amount:
      dice:
        count: 12
        sides: 6
        add: 0
    type: fire
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

##### Destroy

<!-- splat:values path=splat_rule/parameters/layers/0/destroy -->

| Field | Value |
| --- | --- |
| ` damage_type ` | ` cold ` |
| ` minimum_damage ` | ` 25 ` |

##### Save

<!-- splat:yaml path=splat_rule/parameters/layers/1/save -->

```yaml
ability: dexterity
dc:
  binding: save_dc
failure:
  damage:
    amount:
      dice:
        count: 12
        sides: 6
        add: 0
    type: acid
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

##### Destroy

<!-- splat:values path=splat_rule/parameters/layers/1/destroy -->

| Field | Value |
| --- | --- |
| ` effect ` | ` strong_wind ` |

##### Save

<!-- splat:yaml path=splat_rule/parameters/layers/2/save -->

```yaml
ability: dexterity
dc:
  binding: save_dc
failure:
  damage:
    amount:
      dice:
        count: 12
        sides: 6
        add: 0
    type: lightning
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

##### Destroy

<!-- splat:values path=splat_rule/parameters/layers/2/destroy -->

| Field | Value |
| --- | --- |
| ` damage_type ` | ` force ` |
| ` minimum_damage ` | ` 60 ` |

##### Save

<!-- splat:yaml path=splat_rule/parameters/layers/3/save -->

```yaml
ability: dexterity
dc:
  binding: save_dc
failure:
  damage:
    amount:
      dice:
        count: 12
        sides: 6
        add: 0
    type: poison
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

##### Destroy

<!-- splat:yaml path=splat_rule/parameters/layers/3/destroy -->

```yaml
spell: passwall
or_spell:
  minimum_level: 5
  effect: open_portal_on_solid_surface
```

##### Save

<!-- splat:yaml path=splat_rule/parameters/layers/4/save -->

```yaml
ability: dexterity
dc:
  binding: save_dc
failure:
  damage:
    amount:
      dice:
        count: 12
        sides: 6
        add: 0
    type: cold
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

##### Destroy

<!-- splat:values path=splat_rule/parameters/layers/4/destroy -->

| Field | Value |
| --- | --- |
| ` damage_type ` | ` fire ` |
| ` minimum_damage ` | ` 25 ` |

##### Save

<!-- splat:yaml path=splat_rule/parameters/layers/5/save -->

```yaml
ability: dexterity
dc:
  binding: save_dc
failure:
  condition: restrained
  repeat_save:
    event: turn.end
    ability: constitution
    count_success_and_failure_separately: true
    consecutive_required: false
    successes_to_end_restrained: 3
    failures_to_petrify: 3
    petrified_until: freed_by_restoring_effect
success:
  effect: none
```

##### Destroy

<!-- splat:values path=splat_rule/parameters/layers/5/destroy -->

| Field | Value |
| --- | --- |
| ` effect ` | ` bright_light ` |
| ` spell ` | ` daylight ` |

##### Save

<!-- splat:yaml path=splat_rule/parameters/layers/6/save -->

```yaml
ability: dexterity
dc:
  binding: save_dc
failure:
  condition: blinded
  repeat_save:
    event: caster.next_turn.start
    ability: wisdom
    success:
      end_blinded: true
    failure:
      end_blinded: true
      teleport:
        destination: other_plane
        chooser: gm
success:
  effect: none
```

##### Destroy

<!-- splat:values path=splat_rule/parameters/layers/6/destroy -->

| Field | Value |
| --- | --- |
| ` spell ` | ` dispel_magic ` |
