---
splat_schema: 1
splat_id: 697788ce-3b3f-5d8c-959b-61491a3eb620
splat_kind: rule_group
splat_key: dnd2024:backgrounds/sage
splat_name: Sage
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83
  url: https://www.dndbeyond.com/srd
---

# Sage

## Source text

<!-- SRD page 83 -->

Ability Scores: Constitution, Intelligence, Wisdom\
Feat: Magic Initiate (Wizard) (see “Feats”)\
Skill Proficiencies: Arcana and History\
Tool Proficiency: Calligrapher’s Supplies\
Equipment: Choose A or B: (A) Quarterstaff, Callig-\
rapher’s Supplies, Book (history), Parchment (8\
sheets), Robe, 8 GP; or (B) 50 GP\

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

<!-- splat:yaml path=splat_rule/parameters/equipment_choice -->

```yaml
count: 1
options:
- id: a
  grants:
  - item_type: quarterstaff
    quantity: 1
  - item_type: calligraphers_supplies
    quantity: 1
  - item_type: book_history
    quantity: 1
  - item_type: parchment_sheet
    quantity: 8
  - item_type: robe
    quantity: 1
  - item_type: gp
    quantity: 8
- id: b
  grants:
  - item_type: gp
    quantity: 50
```
