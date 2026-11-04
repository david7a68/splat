---
splat_schema: 2
splat_id: 82c65dea-f963-5882-8a3b-20ec4be53d74
splat_kind: rule
splat_key: dnd2024:glossary/concentration
splat_name: Concentration
splat_origin: dnd2024:reference/glossary/concentration
splat_sources:
- SRD 5.2.1, pp. 179, https://www.dndbeyond.com/srd
---

# Concentration

## Source text

<!-- SRD page 179 -->

Some spells and other effects require Concentration to remain active, as specified in their descriptions. If the effect’s creator loses Concentration, the effect ends. If the effect has a maximum duration, the effect’s description specifies how long the creator can concentrate on it: up to 1 minute, 1 hour, or some other duration. The creator can end Concentration at any time (no action required). The following factors break Concentration.

Another Concentration Effect. You lose Concentration on an effect the moment you start casting a spell that requires Concentration or activate another effect that requires Concentration.

Damage. If you take damage, you must succeed on a Constitution saving throw to maintain Concentration. The DC equals 10 or half the damage taken (round down), whichever number is higher, up to a maximum DC of 30.

Incapacitated or Dead. Your Concentration ends if you have the Incapacitated condition or you die.

Original layout: [[../../srd-5.2.1/pages/179|page 179]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/concentration ` |
| ` implementation ` | ` dnd2024:glossary_concentration ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` maximum_simultaneous_effects ` | ` 1 ` |
| ` effect_ends_on_concentration_loss ` | ` true ` |
| ` maximum_duration ` | ` effect_specified ` |

#### Voluntary end

<!-- splat:values path=splat_rule/parameters/voluntary_end -->

| Field | Value |
| --- | --- |
| ` timing ` | ` any_time ` |
| ` cost ` | ` none ` |

#### Breaks when

<!-- splat:table
path: splat_rule/parameters/breaks_when
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Event:
    path: event
    type: string
  Requires concentration:
    path: requires_concentration
    type: boolean
    optional: true
  Condition:
    path: condition
    type: symbol
    optional: true
-->

| Key | Event | Requires concentration | Condition |
| --- | --- | --- | --- |
| ` row_1 ` | ` spell.cast_started ` | true |  |
| ` row_2 ` | ` effect.activated ` | true |  |
| ` row_3 ` | ` condition.applied ` |  | ` Incapacitated ` |
| ` row_4 ` | ` creature.died ` |  |  |

#### Damage save

<!-- splat:yaml path=splat_rule/parameters/damage_save -->

```yaml
ability: constitution
dc:
  op: min
  args:
  - 30
  - op: max
    args:
    - 10
    - op: floor
      args:
      - op: divide
        args:
        - var: damage_taken
        - 2
failure: end_concentration
```
