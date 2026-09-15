---
splat_schema: 1
splat_id: 4fab2519-088c-5319-b1f3-ba8b5640e688
splat_kind: rule
splat_key: dnd2024:rules/combat_creature_size
splat_name: Creature Size
splat_origin:
  key: dnd2024:reference/pages/014
splat_sources:
- title: SRD 5.2.1
  locator: pp. 14
  url: https://www.dndbeyond.com/srd
---

# Creature Size

## Source text

<!-- SRD page 14 -->

A creature belongs to a size category, which deter-\
mines the width of the square space the creature\
occupies on a map, as shown on the Creature Size\
and Space table. That table lists the sizes from\
smallest (Tiny) to largest (Gargantuan). A creature’s\
space is the area that it effectively controls in com-\
bat and the area it needs to fight effectively.\
A character’s size is determined by species, and\
a monster’s size is specified in the monster’s stat\
block.\
Creature Size and Space\
Size Space (Feet) Space (Squares)\
Tiny 2½ by 2½ feet 4 per square\
Small 5 by 5 feet 1 square\
Medium 5 by 5 feet 1 square\
Large 10 by 10 feet 4 squares (2 by 2)\
Huge 15 by 15 feet 9 squares (3 by 3)\
Gargantuan 20 by 20 feet 16 squares (4 by 4)\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_creature_size ` |
| ` implementation ` | ` dnd2024:rule_combat_creature_size ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` player_size_source ` | ` species ` |
| ` monster_size_source ` | ` stat_block ` |

#### Sizes

<!-- splat:table
path: splat_rule/parameters/sizes
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
columns:
  Key:
    type: key
  Size:
    path: size
    type: symbol
  Details:
    type: section
    optional: true
-->

| Key | Size | Details |
| --- | --- | --- |
| ` row_1 ` | ` Tiny ` | [Rules](#^splat-3892f377-row_1 "section:row_1") |
| ` row_2 ` | ` Small ` | [Rules](#^splat-3892f377-row_2 "section:row_2") |
| ` row_3 ` | ` Medium ` | [Rules](#^splat-3892f377-row_3 "section:row_3") |
| ` row_4 ` | ` Large ` | [Rules](#^splat-3892f377-row_4 "section:row_4") |
| ` row_5 ` | ` Huge ` | [Rules](#^splat-3892f377-row_5 "section:row_5") |
| ` row_6 ` | ` Gargantuan ` | [Rules](#^splat-3892f377-row_6 "section:row_6") |

##### Row 1

^splat-3892f377-row_1

###### Space width feet

<!-- splat:yaml path=splat_rule/parameters/sizes/@row_1/space_width_feet -->

```yaml
2.5
```

###### Occupied squares

<!-- splat:yaml path=splat_rule/parameters/sizes/@row_1/occupied_squares -->

```yaml
0.25
```

##### Row 2

^splat-3892f377-row_2

###### Space width feet

<!-- splat:yaml path=splat_rule/parameters/sizes/@row_2/space_width_feet -->

```yaml
5
```

###### Occupied squares

<!-- splat:yaml path=splat_rule/parameters/sizes/@row_2/occupied_squares -->

```yaml
1
```

##### Row 3

^splat-3892f377-row_3

###### Space width feet

<!-- splat:yaml path=splat_rule/parameters/sizes/@row_3/space_width_feet -->

```yaml
5
```

###### Occupied squares

<!-- splat:yaml path=splat_rule/parameters/sizes/@row_3/occupied_squares -->

```yaml
1
```

##### Row 4

^splat-3892f377-row_4

###### Space width feet

<!-- splat:yaml path=splat_rule/parameters/sizes/@row_4/space_width_feet -->

```yaml
10
```

###### Occupied squares

<!-- splat:yaml path=splat_rule/parameters/sizes/@row_4/occupied_squares -->

```yaml
4
```

##### Row 5

^splat-3892f377-row_5

###### Space width feet

<!-- splat:yaml path=splat_rule/parameters/sizes/@row_5/space_width_feet -->

```yaml
15
```

###### Occupied squares

<!-- splat:yaml path=splat_rule/parameters/sizes/@row_5/occupied_squares -->

```yaml
9
```

##### Row 6

^splat-3892f377-row_6

###### Space width feet

<!-- splat:yaml path=splat_rule/parameters/sizes/@row_6/space_width_feet -->

```yaml
20
```

###### Occupied squares

<!-- splat:yaml path=splat_rule/parameters/sizes/@row_6/occupied_squares -->

```yaml
16
```
