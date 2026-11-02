---
splat_schema: 1
splat_id: b21da6c6-a53f-5ffc-82b9-b1bb6fa0acf3
splat_kind: rule_group
splat_key: dnd2024:backgrounds/acolyte
splat_name: Acolyte
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83
  url: https://www.dndbeyond.com/srd
---

# Acolyte

## Source text

<!-- SRD page 83 -->

Ability Scores: Intelligence, Wisdom, Charisma\
Feat: Magic Initiate (Cleric) (see “Feats”)\
Skill Proficiencies: Insight and Religion\
Tool Proficiency: Calligrapher’s Supplies\
Equipment: Choose A or B: (A) Calligrapher’s Supplies, Book (prayers), Holy Symbol, Parchment (10 sheets), Robe, 8 GP; or (B) 50 GP

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` background ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:backgrounds/acolyte ` |
| ` implementation ` | ` dnd2024:backgrounds_acolyte ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` skill_proficiencies ` | ` ["insight", "religion"] ` |
| ` tool_proficiencies ` | ` ["calligraphers_supplies"] ` |

#### Ability score choices

<!-- splat:yaml path=splat_rule/parameters/ability_score_choices -->

```yaml
eligible:
- intelligence
- wisdom
- charisma
allocations:
- - 2
  - 1
- - 1
  - 1
  - 1
distinct_abilities: true
maximum_score: 20
```

#### Feat grant

<!-- splat:yaml path=splat_rule/parameters/feat_grant -->

```yaml
definition:
  key: dnd2024:feats/origin_feats_magic_initiate
spell_list: cleric
```

#### Equipment choice

<!-- splat:values path=splat_rule/parameters/equipment_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |

##### Options

<!-- splat:table
path: splat_rule/parameters/equipment_choice/options
key: Variant
order:
- a
- b
columns:
  Variant:
    path: id
    type: symbol
  Details:
    type: section
    optional: true
-->

| Variant | Details |
| --- | --- |
| ` A ` | [Rules](#^splat-7a51fef2-a "section:a") |
| ` B ` | [Rules](#^splat-7a51fef2-b "section:b") |

###### A

^splat-7a51fef2-a

###### Grants

<!-- splat:table
path: splat_rule/parameters/equipment_choice/options/@a/grants
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
  Item type:
    path: item_type
    type: symbol
  Quantity:
    path: quantity
    type: integer
-->

| Key | Item type | Quantity |
| --- | --- | --- |
| ` row_1 ` | ` Calligraphers supplies ` | ` 1 ` |
| ` row_2 ` | ` Book prayers ` | ` 1 ` |
| ` row_3 ` | ` Holy symbol ` | ` 1 ` |
| ` row_4 ` | ` Parchment sheet ` | ` 10 ` |
| ` row_5 ` | ` Robe ` | ` 1 ` |
| ` row_6 ` | ` Gp ` | ` 8 ` |

###### B

^splat-7a51fef2-b

###### Grants

<!-- splat:rows path=splat_rule/parameters/equipment_choice/options/@b/grants -->

| Row | item_type | quantity |
| --- | --- | --- |
| ` 0 ` | ` gp ` | ` 50 ` |
