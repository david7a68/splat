---
splat_schema: 2
splat_id: 697788ce-3b3f-5d8c-959b-61491a3eb620
splat_kind: rule_group
splat_key: dnd2024:backgrounds/sage
splat_name: Sage
splat_origin: dnd2024:reference/pages/083
splat_sources:
- SRD 5.2.1, pp. 83, https://www.dndbeyond.com/srd
---

# Sage

## Source text

<!-- SRD page 83 -->

Ability Scores: Constitution, Intelligence, Wisdom\
Feat: Magic Initiate (Wizard) (see “Feats”)\
Skill Proficiencies: Arcana and History\
Tool Proficiency: Calligrapher’s Supplies\
Equipment: Choose A or B: (A) Quarterstaff, Calligrapher’s Supplies, Book (history), Parchment (8 sheets), Robe, 8 GP; or (B) 50 GP

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` background ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:backgrounds/sage ` |
| ` implementation ` | ` dnd2024:backgrounds_sage ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` skill_proficiencies ` | ` ["arcana", "history"] ` |
| ` tool_proficiencies ` | ` ["calligraphers_supplies"] ` |

#### Ability score choices

<!-- splat:yaml path=splat_rule/parameters/ability_score_choices -->

```yaml
eligible:
- constitution
- intelligence
- wisdom
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
spell_list: wizard
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
| ` row_1 ` | ` Quarterstaff ` | ` 1 ` |
| ` row_2 ` | ` Calligraphers supplies ` | ` 1 ` |
| ` row_3 ` | ` Book history ` | ` 1 ` |
| ` row_4 ` | ` Parchment sheet ` | ` 8 ` |
| ` row_5 ` | ` Robe ` | ` 1 ` |
| ` row_6 ` | ` Gp ` | ` 8 ` |

###### B

^splat-7a51fef2-b

###### Grants

<!-- splat:rows path=splat_rule/parameters/equipment_choice/options/@b/grants -->

| Row | item_type | quantity |
| --- | --- | --- |
| ` 0 ` | ` gp ` | ` 50 ` |
