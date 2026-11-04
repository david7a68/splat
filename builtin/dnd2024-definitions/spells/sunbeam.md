---
splat_schema: 2
splat_id: c73aa5d8-490d-50b8-b117-d86eccae5c96
splat_kind: spell
splat_key: dnd2024:spells/sunbeam
splat_name: Sunbeam
splat_origin: dnd2024:reference/spell/sunbeam
splat_sources:
- SRD 5.2.1, pp. 166, https://www.dndbeyond.com/srd
---

# Sunbeam


## Spell

| Field | Value |
| --- | --- |
| Level | 6 |
| School | Evocation |
| Classes | Cleric, Druid, Sorcerer, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (a magnifying glass) |
| Duration | Concentration, up to 1 minute |


## Source text

<!-- SRD page 166 -->

Level 6 Evocation (Cleric, Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a magnifying glass)\
Duration: Concentration, up to 1 minute

You launch a sunbeam in a 5-foot-wide, 60-foot-long Line. Each creature in the Line makes a Constitution saving throw. On a failed save, a creature takes 6d8 Radiant damage and has the Blinded condition until the start of your next turn. On a successful save, it takes half as much damage only.

Until the spell ends, you can take a Magic action to create a new Line of radiance.

For the duration, a mote of brilliant radiance shines above you. It sheds Bright Light in a 30-foot radius and Dim Light for an additional 30 feet. This light is sunlight.

Original layout: [[../../srd-5.2.1/pages/166|page 166]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/sunbeam ` |
| ` implementation ` | ` dnd2024:spell_sunbeam ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures_in_area ` |
| ` shape ` | ` line ` |
| ` width_feet ` | ` 5 ` |
| ` length_feet ` | ` 60 ` |
| ` origin ` | ` caster ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
failure:
  damage:
    dice: 6d8
    type: radiant
  condition: blinded
  until: caster_next_turn_start
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

#### Repeat

<!-- splat:values path=splat_rule/parameters/repeat -->

| Field | Value |
| --- | --- |
| ` economy ` | ` magic_action ` |
| ` new_line ` | ` true ` |
| ` duration ` | ` spell_duration ` |

#### Mote

<!-- splat:values path=splat_rule/parameters/mote -->

| Field | Value |
| --- | --- |
| ` location ` | ` above_caster ` |
| ` bright_radius_feet ` | ` 30 ` |
| ` additional_dim_feet ` | ` 30 ` |
| ` sunlight ` | ` true ` |
| ` duration ` | ` spell_duration ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: minute
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: self
> ```

