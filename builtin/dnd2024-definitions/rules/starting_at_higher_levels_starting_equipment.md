---
splat_schema: 1
splat_id: 5e635839-74d4-5538-a595-b3fde065bab6
splat_kind: rule
splat_key: dnd2024:rules/starting_at_higher_levels_starting_equipment
splat_name: Starting Equipment
splat_origin:
  key: dnd2024:reference/pages/024
splat_sources:
- title: SRD 5.2.1
  locator: pp. 24
  url: https://www.dndbeyond.com/srd
---

# Starting Equipment

## Source text

<!-- SRD page 24 -->

The GM decides whether your character starts with\
more than the standard equipment for a level 1\
character, possibly even one or more magic items.\
The Starting Equipment at Higher Levels table is a\
guide for the GM.\
Also, check with your GM about what equipment\
is available for you to buy with your starting money.\
For example, the firearms described in “Equipment”\
are too expensive for level 1 characters, but they\
might be available for purchase if your GM allows\
them.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/starting_at_higher_levels_starting_equipment ` |
| ` implementation ` | ` dnd2024:rules_starting_at_higher_levels_starting_equipment ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` authority ` | ` gm ` |
| ` table_is_guidance ` | ` true ` |
| ` normal_starting_equipment_included ` | ` true ` |
| ` availability_authority ` | ` gm ` |

#### Starting level equipment

<!-- splat:rows path=splat_rule/parameters/starting_level_equipment -->

| Row | levels | additional_gp |
| --- | --- | --- |
| ` 0 ` | ` [2, 4] ` | ` 0 ` |
| ` 1 ` | ` [5, 10] ` |  |
| ` 2 ` | ` [11, 16] ` |  |
| ` 3 ` | ` [17, 20] ` |  |

##### Magic items

<!-- splat:values path=splat_rule/parameters/starting_level_equipment/0/magic_items -->

| Field | Value |
| --- | --- |
| ` common ` | ` 1 ` |

##### Additional gp

<!-- splat:yaml path=splat_rule/parameters/starting_level_equipment/1/additional_gp -->

```yaml
base: 500
roll:
  count: 1
  sides: 10
  add: 0
multiplier: 25
```

##### Magic items

<!-- splat:values path=splat_rule/parameters/starting_level_equipment/1/magic_items -->

| Field | Value |
| --- | --- |
| ` common ` | ` 1 ` |
| ` uncommon ` | ` 1 ` |

##### Additional gp

<!-- splat:yaml path=splat_rule/parameters/starting_level_equipment/2/additional_gp -->

```yaml
base: 5000
roll:
  count: 1
  sides: 10
  add: 0
multiplier: 250
```

##### Magic items

<!-- splat:values path=splat_rule/parameters/starting_level_equipment/2/magic_items -->

| Field | Value |
| --- | --- |
| ` common ` | ` 2 ` |
| ` uncommon ` | ` 3 ` |
| ` rare ` | ` 1 ` |

##### Additional gp

<!-- splat:yaml path=splat_rule/parameters/starting_level_equipment/3/additional_gp -->

```yaml
base: 20000
roll:
  count: 1
  sides: 10
  add: 0
multiplier: 250
```

##### Magic items

<!-- splat:values path=splat_rule/parameters/starting_level_equipment/3/magic_items -->

| Field | Value |
| --- | --- |
| ` common ` | ` 2 ` |
| ` uncommon ` | ` 4 ` |
| ` rare ` | ` 3 ` |
| ` very_rare ` | ` 1 ` |
