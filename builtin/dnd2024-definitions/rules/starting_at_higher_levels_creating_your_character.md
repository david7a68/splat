---
splat_schema: 1
splat_id: 1366dc42-9425-52c3-a79c-6c0a7fe46c73
splat_kind: rule
splat_key: dnd2024:rules/starting_at_higher_levels_creating_your_character
splat_name: Creating Your Character
splat_origin:
  key: dnd2024:reference/pages/024
splat_sources:
- title: SRD 5.2.1
  locator: pp. 24
  url: https://www.dndbeyond.com/srd
---

# Creating Your Character

## Source text

<!-- SRD page 24 -->

Creating a higher-level character uses the same character-creation steps outlined in this chapter and the rules for advancing beyond level 1 provided in the “Level Advancement” section. You begin with the minimum amount of XP required to reach your starting level. For example, if the GM starts you at level 10, you have 64,000 XP.

### Bonus Feats At Level 20

A GM can use feats as a form of advancement after characters reach level 20 to provide greater power to characters who have no more levels to gain. With this approach, each character gains one feat of their choice for every 30,000 XP the character earns above 355,000 XP. Epic Boon feats are especially appropriate for these bonus feats, but a player can choose any feat for which their level 20 character qualifies.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/starting_at_higher_levels_creating_your_character ` |
| ` implementation ` | ` dnd2024:rule_starting_at_higher_levels_creating_your_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` normal_character_creation_and_advancement ` | ` true ` |
| ` starting_xp ` | ` minimum_for_starting_level ` |
| ` normal_starting_equipment ` | ` true ` |

#### Starting equipment table

<!-- splat:table
path: splat_rule/parameters/starting_equipment_table
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Levels:
    path: levels
    type: yaml
  Base gp:
    path: base_gp
    type: integer
  Magic items common:
    path: magic_items/common
    type: integer
  Random gp op:
    path: random_gp/op
    type: symbol
    optional: true
  Magic items uncommon:
    path: magic_items/uncommon
    type: integer
    optional: true
  Magic items rare:
    path: magic_items/rare
    type: integer
    optional: true
  Magic items very rare:
    path: magic_items/very_rare
    type: integer
    optional: true
  Details:
    type: section
    optional: true
-->

| Key | Levels | Base gp | Magic items common | Random gp op | Magic items uncommon | Magic items rare | Magic items very rare | Details |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` [2, 4] ` | ` 0 ` | ` 1 ` |  |  |  |  | [Rules](#^splat-d91f87fa-row_1 "section:row_1") |
| ` row_2 ` | ` [5, 10] ` | ` 500 ` | ` 1 ` | ` Multiply ` | ` 1 ` |  |  | [Rules](#^splat-d91f87fa-row_2 "section:row_2") |
| ` row_3 ` | ` [11, 16] ` | ` 5000 ` | ` 2 ` | ` Multiply ` | ` 3 ` | ` 1 ` |  | [Rules](#^splat-d91f87fa-row_3 "section:row_3") |
| ` row_4 ` | ` [17, 20] ` | ` 20000 ` | ` 2 ` | ` Multiply ` | ` 4 ` | ` 3 ` | ` 1 ` | [Rules](#^splat-d91f87fa-row_4 "section:row_4") |

##### Row 1

^splat-d91f87fa-row_1

###### Random gp

<!-- splat:yaml path=splat_rule/parameters/starting_equipment_table/@row_1/random_gp -->

```yaml
0
```

##### Row 2

^splat-d91f87fa-row_2

###### Args

<!-- splat:yaml path=splat_rule/parameters/starting_equipment_table/@row_2/random_gp/args -->

```yaml
- dice:
    count: 1
    sides: 10
- 25
```

##### Row 3

^splat-d91f87fa-row_3

###### Args

<!-- splat:yaml path=splat_rule/parameters/starting_equipment_table/@row_3/random_gp/args -->

```yaml
- dice:
    count: 1
    sides: 10
- 250
```

##### Row 4

^splat-d91f87fa-row_4

###### Args

<!-- splat:yaml path=splat_rule/parameters/starting_equipment_table/@row_4/random_gp/args -->

```yaml
- dice:
    count: 1
    sides: 10
- 250
```

#### Level twenty bonus feats

<!-- splat:values path=splat_rule/parameters/level_twenty_bonus_feats -->

| Field | Value |
| --- | --- |
| ` gm_option ` | ` true ` |
| ` xp_baseline ` | ` 355000 ` |
| ` xp_per_feat ` | ` 30000 ` |
| ` choice ` | ` any_qualified_feat ` |
| ` level_for_prerequisites ` | ` 20 ` |
