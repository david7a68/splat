---
splat_schema: 1
splat_id: 07914b89-3c94-5093-865a-cde91556bbcd
splat_kind: rule
splat_key: dnd2024:glossary/falling_hazard
splat_name: Falling [Hazard]
splat_origin:
  key: dnd2024:reference/glossary/falling_hazard
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182
  url: https://www.dndbeyond.com/srd
---

# Falling [Hazard]

## Source text

<!-- SRD page 182 -->

A creature that falls takes 1d6 Bludgeoning dam-\
age at the end of the fall for every 10 feet it fell, to\
a maximum of 20d6. When the creature lands, it\
has the Prone condition unless it avoids taking any\
damage from the fall.\
A creature that falls into water or another liquid\
can use its Reaction to make a DC 15 Strength (Ath-\
letics) or Dexterity (Acrobatics) check to hit the\
surface head or feet first. On a successful check, any\
damage resulting from the fall is halved.\

Original layout: [[../../srd-5.2.1/pages/182|page 182]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/falling_hazard ` |
| ` implementation ` | ` dnd2024:glossary_falling_hazard ` |

### Parameters

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
timing: fall_end
type: bludgeoning
die_sides: 6
dice_count:
  op: min
  args:
  - 20
  - op: floor
    args:
    - op: divide
      args:
      - var: fall_distance_feet
      - 10
```

#### Landing condition

<!-- splat:values path=splat_rule/parameters/landing_condition -->

| Field | Value |
| --- | --- |
| ` condition ` | ` prone ` |
| ` unless_no_fall_damage ` | ` true ` |

#### Liquid landing

<!-- splat:values path=splat_rule/parameters/liquid_landing -->

| Field | Value |
| --- | --- |
| ` eligible_liquids ` | ` any ` |
| ` optional ` | ` true ` |
| ` cost ` | ` reaction ` |

##### Check

<!-- splat:values path=splat_rule/parameters/liquid_landing/check -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` subject ` |
| ` dc ` | ` 15 ` |

###### Options

<!-- splat:table
path: splat_rule/parameters/liquid_landing/check/options
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Ability:
    path: ability
    type: symbol
  Skill:
    path: skill
    type: symbol
-->

| Key | Ability | Skill |
| --- | --- | --- |
| ` row_1 ` | ` Strength ` | ` Athletics ` |
| ` row_2 ` | ` Dexterity ` | ` Acrobatics ` |

##### Success

<!-- splat:values path=splat_rule/parameters/liquid_landing/success -->

| Field | Value |
| --- | --- |
| ` entry_orientation ` | ` ["head_first", "feet_first"] ` |
| ` fall_damage_multiplier ` | ` 0.5 ` |
