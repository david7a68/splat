---
splat_schema: 2
splat_id: 92d39316-b0ec-5655-a7c1-c5df64ea7786
splat_kind: rule
splat_key: dnd2024:rules/combat_cover
splat_name: Cover
splat_origin: dnd2024:reference/pages/015
splat_sources:
- SRD 5.2.1, pp. 15, https://www.dndbeyond.com/srd
---

# Cover

## Source text

<!-- SRD page 15 -->

Walls, trees, creatures, and other obstacles can provide cover, making a target more difficult to harm. As detailed in the Cover table, there are three degrees of cover, each of which gives a different benefit to a target.

A target can benefit from cover only when an attack or other effect originates on the opposite side of the cover. If a target is behind multiple sources of cover, only the most protective degree of cover applies; the degrees aren’t added together. For example, if a target is behind a creature that gives Half Cover and a tree trunk that gives Three-Quarters Cover, the target has Three-Quarters Cover.

**Cover**

| Degree | Benefit to Target | Offered By … |
| --- | --- | --- |
| Half | +2 bonus to AC and Dexterity saving throws | Another creature or an object that covers at least half of the target |
| Three-Quarters | +5 bonus to AC and Dexterity saving throws | An object that covers at least three-quarters of the target |
| Total | Can’t be targeted directly | An object that covers the whole target |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_cover ` |
| ` implementation ` | ` dnd2024:rule_combat_cover ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` requires ` | ` effect_origin_opposite_side_of_cover ` |
| ` stacking ` | ` highest_degree_only ` |

#### Degrees

<!-- splat:table
path: splat_rule/parameters/degrees
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Degree:
    path: degree
    type: symbol
  Ac bonus:
    path: ac_bonus
    type: integer
    optional: true
  Dexterity save bonus:
    path: dexterity_save_bonus
    type: integer
    optional: true
  Creature can supply:
    path: creature_can_supply
    type: boolean
    optional: true
  Direct targeting:
    path: direct_targeting
    type: boolean
    optional: true
  Details:
    type: section
    optional: true
-->

| Key | Degree | Ac bonus | Dexterity save bonus | Creature can supply | Direct targeting | Details |
| --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` Half ` | ` 2 ` | ` 2 ` | true |  | [Rules](#^splat-d3e28ab8-row_1 "section:row_1") |
| ` row_2 ` | ` Three quarters ` | ` 5 ` | ` 5 ` |  |  | [Rules](#^splat-d3e28ab8-row_2 "section:row_2") |
| ` row_3 ` | ` Total ` |  |  |  | false | [Rules](#^splat-d3e28ab8-row_3 "section:row_3") |

##### Row 1

^splat-d3e28ab8-row_1

###### Minimum covered fraction

<!-- splat:yaml path=splat_rule/parameters/degrees/@row_1/minimum_covered_fraction -->

```yaml
0.5
```

##### Row 2

^splat-d3e28ab8-row_2

###### Minimum covered fraction

<!-- splat:yaml path=splat_rule/parameters/degrees/@row_2/minimum_covered_fraction -->

```yaml
0.75
```

##### Row 3

^splat-d3e28ab8-row_3

###### Minimum covered fraction

<!-- splat:yaml path=splat_rule/parameters/degrees/@row_3/minimum_covered_fraction -->

```yaml
1
```
