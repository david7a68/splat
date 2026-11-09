---
splat_schema: 2
splat_id: b7643b11-2a43-559e-971f-aae50cc8f01f
splat_kind: spell
splat_key: dnd2024:spells/thunderwave
splat_name: Thunderwave
splat_origin: dnd2024:reference/spell/thunderwave
splat_sources:
- SRD 5.2.1, pp. 169, https://www.dndbeyond.com/srd
---

# Thunderwave


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Evocation |
| Classes | Bard, Druid, Sorcerer, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 169 -->

Level 1 Evocation (Bard, Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Instantaneous

You unleash a wave of thunderous energy. Each creature in a 15-foot Cube originating from you makes a Constitution saving throw. On a failed save, a creature takes 2d8 Thunder damage and is pushed 10 feet away from you. On a successful save, a creature takes half as much damage only.

In addition, unsecured objects that are entirely within the Cube are pushed 10 feet away from you, and a thunderous boom is audible within 300 feet.

Using a Higher-Level Spell Slot. The damage increases by 1d8 for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/169|page 169]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/thunderwave ` |
| ` implementation ` | ` dnd2024:spell_thunderwave ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures_in_area ` |
| ` shape ` | ` cube ` |
| ` side_feet ` | ` 15 ` |
| ` origin ` | ` caster ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
failure:
  damage:
    dice: 2d8
    type: thunder
  push:
    distance_feet: 10
    direction: away_from_caster
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

#### Objects

<!-- splat:values path=splat_rule/parameters/objects -->

| Field | Value |
| --- | --- |
| ` unsecured ` | ` true ` |
| ` entirely_within_area ` | ` true ` |
| ` push_feet ` | ` 10 ` |
| ` direction ` | ` away_from_caster ` |

#### Sound

<!-- splat:values path=splat_rule/parameters/sound -->

| Field | Value |
| --- | --- |
| ` kind ` | ` thunderous_boom ` |
| ` audible_radius_feet ` | ` 300 ` |

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 1 ` |
| ` damage_per_level ` | ` 1d8 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: self
> ```

