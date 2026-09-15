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
Equipment: Choose A or B: (A) Calligrapher’s Supplies,\
Book (prayers), Holy Symbol, Parchment (10 sheets),\
Robe, 8 GP; or (B) 50 GP\

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

<!-- splat:yaml path=splat_rule/parameters/equipment_choice -->

```yaml
count: 1
options:
- id: a
  grants:
  - item_type: calligraphers_supplies
    quantity: 1
  - item_type: book_prayers
    quantity: 1
  - item_type: holy_symbol
    quantity: 1
  - item_type: parchment_sheet
    quantity: 10
  - item_type: robe
    quantity: 1
  - item_type: gp
    quantity: 8
- id: b
  grants:
  - item_type: gp
    quantity: 50
```
