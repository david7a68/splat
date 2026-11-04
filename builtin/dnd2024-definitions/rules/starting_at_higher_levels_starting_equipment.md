---
splat_schema: 2
splat_id: 5e635839-74d4-5538-a595-b3fde065bab6
splat_kind: rule
splat_key: dnd2024:rules/starting_at_higher_levels_starting_equipment
splat_name: Starting Equipment
splat_origin: dnd2024:reference/pages/024
splat_sources:
- SRD 5.2.1, pp. 24, https://www.dndbeyond.com/srd
---

# Starting Equipment

## Source text

<!-- SRD page 24 -->

The GM decides whether your character starts with more than the standard equipment for a level 1 character, possibly even one or more magic items. The Starting Equipment at Higher Levels table is a guide for the GM.

Also, check with your GM about what equipment is available for you to buy with your starting money. For example, the firearms described in “Equipment” are too expensive for level 1 characters, but they might be available for purchase if your GM allows them.

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

<!-- splat:table
path: splat_rule/parameters/starting_level_equipment
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
  Magic items common:
    path: magic_items/common
    type: integer
  Additional gp base:
    path: additional_gp/base
    type: integer
    optional: true
  Additional gp roll:
    path: additional_gp/roll
    type: dice
    optional: true
  Additional gp multiplier:
    path: additional_gp/multiplier
    type: integer
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

| Key | Levels | Magic items common | Additional gp base | Additional gp roll | Additional gp multiplier | Magic items uncommon | Magic items rare | Magic items very rare | Details |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` [2, 4] ` | ` 1 ` |  |  |  |  |  |  | [Rules](#^splat-3a618873-row_1 "section:row_1") |
| ` row_2 ` | ` [5, 10] ` | ` 1 ` | ` 500 ` | ` 1d10 ` | ` 25 ` | ` 1 ` |  |  |  |
| ` row_3 ` | ` [11, 16] ` | ` 2 ` | ` 5000 ` | ` 1d10 ` | ` 250 ` | ` 3 ` | ` 1 ` |  |  |
| ` row_4 ` | ` [17, 20] ` | ` 2 ` | ` 20000 ` | ` 1d10 ` | ` 250 ` | ` 4 ` | ` 3 ` | ` 1 ` |  |

##### Row 1

^splat-3a618873-row_1

###### Additional gp

<!-- splat:yaml path=splat_rule/parameters/starting_level_equipment/@row_1/additional_gp -->

```yaml
0
```
