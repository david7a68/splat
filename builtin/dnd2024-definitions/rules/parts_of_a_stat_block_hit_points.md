---
splat_schema: 1
splat_id: e9611215-c32f-5b24-b81a-e17349713e6f
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_hit_points
splat_name: Hit Points
splat_origin:
  key: dnd2024:reference/pages/255
splat_sources:
- title: SRD 5.2.1
  locator: pp. 255
  url: https://www.dndbeyond.com/srd
---

# Hit Points

## Source text

<!-- SRD page 255 -->

A monster’s Hit Points are presented as a number\
followed by parentheses, where the monster’s Hit\
Point Dice are provided, along with any contribu-\
tion from its Constitution. Either use the number for\
the monster’s Hit Points or roll the die expression in\
parentheses to determine the monster’s Hit Points\
randomly; don’t use both.\
A monster’s size typically determines the die used\
to calculate its Hit Points, as shown in the Hit Dice\
by Size table.\
Hit Dice by Size\
Monster Size Hit Die Average HP per Die\
Tiny d4 2½\
Small d6 3½\
Medium d8 4½\
Large d10 5½\
Huge d12 6½\
Gargantuan d20 10½\
A monster’s Constitution modifier is multiplied by\
the number of Hit Dice it possesses, and the result\
is added to its Hit Points. For example, if a monster\
has a Constitution of 12 (+1 modifier) and 2d8 Hit\
Dice, it has 2d8 + 2 Hit Points (average 11).\
For more on Hit Points, see “Playing the Game.”\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_hit_points ` |
| ` implementation ` | ` dnd2024:rule_parts_of_a_stat_block_hit_points ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` hit_point_generation_choice ` | ` ["printed_number", "roll_printed_expression"] ` |
| ` combine_methods ` | ` false ` |

#### Constitution contribution

<!-- splat:yaml path=splat_rule/parameters/constitution_contribution -->

```yaml
op: multiply
args:
- variable: constitution_modifier
- variable: hit_dice_count
```

#### Typical hit die by size

<!-- splat:rows path=splat_rule/parameters/typical_hit_die_by_size -->

| Row | size | die_sides | average_per_die |
| --- | --- | --- | --- |
| ` 0 ` | ` tiny ` | ` 4 ` | ` 2.5 ` |
| ` 1 ` | ` small ` | ` 6 ` | ` 3.5 ` |
| ` 2 ` | ` medium ` | ` 8 ` | ` 4.5 ` |
| ` 3 ` | ` large ` | ` 10 ` | ` 5.5 ` |
| ` 4 ` | ` huge ` | ` 12 ` | ` 6.5 ` |
| ` 5 ` | ` gargantuan ` | ` 20 ` | ` 10.5 ` |
