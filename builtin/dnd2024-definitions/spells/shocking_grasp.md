---
splat_schema: 2
splat_id: 2f42c7a2-36ab-5b1d-9a9d-968c579a6c4c
splat_kind: spell
splat_key: dnd2024:spells/shocking_grasp
splat_name: Shocking Grasp
splat_origin: dnd2024:reference/spell/shocking_grasp
splat_sources:
- SRD 5.2.1, pp. 162, https://www.dndbeyond.com/srd
---

# Shocking Grasp


## Spell

| Field | Value |
| --- | --- |
| Level | Cantrip |
| School | Evocation |
| Classes | Sorcerer, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | Instantaneous |


## Source text

<!-- SRD page 162 -->

Evocation Cantrip (Sorcerer, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Instantaneous

Lightning springs from you to a creature that you try to touch. Make a melee spell attack against the target. On a hit, the target takes 1d8 Lightning damage, and it can’t make Opportunity Attacks until the start of its next turn.

Cantrip Upgrade. The damage increases by 1d8 when you reach levels 5 (2d8), 11 (3d8), and 17 (4d8).

Original layout: [[../../srd-5.2.1/pages/162|page 162]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/shocking_grasp ` |
| ` implementation ` | ` dnd2024:spell_shocking_grasp ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |

#### Attack

<!-- splat:values path=splat_rule/parameters/attack -->

| Field | Value |
| --- | --- |
| ` kind ` | ` melee_spell ` |

##### Hit

<!-- splat:values path=splat_rule/parameters/attack/hit -->

| Field | Value |
| --- | --- |
| ` prohibit ` | ` opportunity_attacks ` |
| ` until ` | ` target_next_turn_start ` |

###### Damage

<!-- splat:values path=splat_rule/parameters/attack/hit/damage -->

| Field | Value |
| --- | --- |
| ` type ` | ` lightning ` |

###### Dice

<!-- splat:values path=splat_rule/parameters/attack/hit/damage/dice -->

| Field | Value |
| --- | --- |
| ` sides ` | ` 8 ` |
| ` add ` | ` 0 ` |

###### Count

###### By caster level

<!-- splat:table
path: splat_rule/parameters/attack/hit/damage/dice/count/by_caster_level
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Minimum:
    path: minimum
    type: integer
  Count:
    path: count
    type: integer
-->

| Key | Minimum | Count |
| --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 1 ` |
| ` row_2 ` | ` 5 ` | ` 2 ` |
| ` row_3 ` | ` 11 ` | ` 3 ` |
| ` row_4 ` | ` 17 ` | ` 4 ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: touch
> ```

