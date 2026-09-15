---
splat_schema: 1
splat_id: b6f3840e-2704-5a6c-bfc9-36632f08459a
splat_kind: rule_group
splat_key: dnd2024:backgrounds/criminal
splat_name: Criminal
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83
  url: https://www.dndbeyond.com/srd
---

# Criminal

## Source text

<!-- SRD page 83 -->

Ability Scores: Dexterity, Constitution, Intelligence\
Feat: Alert (see “Feats”)\
Skill Proficiencies: Sleight of Hand and Stealth\
Tool Proficiency: Thieves’ Tools\
Equipment: Choose A or B: (A) 2 Daggers, Thieves’\
Tools, Crowbar, 2 Pouches, Traveler’s Clothes,\
16 GP; or (B) 50 GP\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` background ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:backgrounds/criminal ` |
| ` implementation ` | ` dnd2024:backgrounds_criminal ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` ability_score_choices ` | ` ["dexterity", "constitution", "intelligence"] ` |
| ` skill_proficiencies ` | ` ["sleight_of_hand", "stealth"] ` |

#### Ability increase

<!-- splat:yaml path=splat_rule/parameters/ability_increase -->

```yaml
patterns:
- - 2
  - 1
- - 1
  - 1
  - 1
distinct_abilities: true
maximum_score: 20
```

#### Feat

<!-- splat:values path=splat_rule/parameters/feat -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:feats/origin_feats_alert ` |

#### Tool proficiency

<!-- splat:values path=splat_rule/parameters/tool_proficiency -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/other_tools_thieves_tools_25_gp ` |

#### Equipment

<!-- splat:yaml path=splat_rule/parameters/equipment -->

```yaml
choose: 1
options:
  equipment:
    items:
    - definition:
        key: dnd2024:equipment/dagger
      quantity: 2
    - definition:
        key: dnd2024:equipment/other_tools_thieves_tools_25_gp
      quantity: 1
    - definition:
        key: dnd2024:equipment/crowbar_2_gp
      quantity: 1
    - definition:
        key: dnd2024:equipment/pouch_5_sp
      quantity: 2
    - definition:
        key: dnd2024:equipment/clothes_traveler_s_2_gp
      quantity: 1
    currency:
      gp: 16
  currency:
    gp: 50
```
