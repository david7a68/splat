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

Creating a higher-level character uses the same\
character-creation steps outlined in this chapter\
and the rules for advancing beyond level 1 provided\
Starting Equipment at Higher Levels\
Starting Level Equipment and Money\
2–4 Normal starting equipment\
5–10 500 GP plus 1d10 × 25 GP plus\
normal starting equipment\
11–16 5,000 gp plus 1d10 × 250 GP plus\
normal starting equipment\
17–20 20,000 GP plus 1d10 × 250 GP plus\
normal starting equipment\
Bonus FEats at lEvEl 20\
A GM can use feats as a form of advancement after\
characters reach level 20 to provide greater power\
to characters who have no more levels to gain. With\
this approach, each character gains one feat of their\
choice for every 30,000 XP the character earns\
above 355,000 XP. Epic Boon feats are especially\
appropriate for these bonus feats, but a player can\
choose any feat for which their level 20 character\
qualifies.\
in the “Level Advancement” section. You begin with\
the minimum amount of XP required to reach your\
starting level. For example, if the GM starts you at\
level 10, you have 64,000 XP.\

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

<!-- splat:rows path=splat_rule/parameters/starting_equipment_table -->

| Row | levels | base_gp | random_gp |
| --- | --- | --- | --- |
| ` 0 ` | ` [2, 4] ` | ` 0 ` | ` 0 ` |
| ` 1 ` | ` [5, 10] ` | ` 500 ` |  |
| ` 2 ` | ` [11, 16] ` | ` 5000 ` |  |
| ` 3 ` | ` [17, 20] ` | ` 20000 ` |  |

##### Magic items

<!-- splat:values path=splat_rule/parameters/starting_equipment_table/0/magic_items -->

| Field | Value |
| --- | --- |
| ` common ` | ` 1 ` |

##### Random gp

<!-- splat:yaml path=splat_rule/parameters/starting_equipment_table/1/random_gp -->

```yaml
op: multiply
args:
- dice:
    count: 1
    sides: 10
- 25
```

##### Magic items

<!-- splat:values path=splat_rule/parameters/starting_equipment_table/1/magic_items -->

| Field | Value |
| --- | --- |
| ` common ` | ` 1 ` |
| ` uncommon ` | ` 1 ` |

##### Random gp

<!-- splat:yaml path=splat_rule/parameters/starting_equipment_table/2/random_gp -->

```yaml
op: multiply
args:
- dice:
    count: 1
    sides: 10
- 250
```

##### Magic items

<!-- splat:values path=splat_rule/parameters/starting_equipment_table/2/magic_items -->

| Field | Value |
| --- | --- |
| ` common ` | ` 2 ` |
| ` uncommon ` | ` 3 ` |
| ` rare ` | ` 1 ` |

##### Random gp

<!-- splat:yaml path=splat_rule/parameters/starting_equipment_table/3/random_gp -->

```yaml
op: multiply
args:
- dice:
    count: 1
    sides: 10
- 250
```

##### Magic items

<!-- splat:values path=splat_rule/parameters/starting_equipment_table/3/magic_items -->

| Field | Value |
| --- | --- |
| ` common ` | ` 2 ` |
| ` uncommon ` | ` 4 ` |
| ` rare ` | ` 3 ` |
| ` very_rare ` | ` 1 ` |

#### Level twenty bonus feats

<!-- splat:values path=splat_rule/parameters/level_twenty_bonus_feats -->

| Field | Value |
| --- | --- |
| ` gm_option ` | ` true ` |
| ` xp_baseline ` | ` 355000 ` |
| ` xp_per_feat ` | ` 30000 ` |
| ` choice ` | ` any_qualified_feat ` |
| ` level_for_prerequisites ` | ` 20 ` |
