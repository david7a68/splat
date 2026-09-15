---
splat_schema: 1
splat_id: 2b24f1ab-a317-5e70-8e90-e8de54d7e1d6
splat_kind: rule_group
splat_key: dnd2024:backgrounds/soldier
splat_name: Soldier
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83
  url: https://www.dndbeyond.com/srd
---

# Soldier

## Source text

<!-- SRD page 83 -->

Ability Scores: Strength, Dexterity, Constitution\
Feat: Savage Attacker (see “Feats”)\
Skill Proficiencies: Athletics and Intimidation\
Tool Proficiency: Choose one kind of Gaming Set (see\
“Equipment”)\
Equipment: Choose A or B: (A) Spear, Shortbow, 20\
Arrows, Gaming Set (same as above), Healer’s Kit,\
Quiver, Traveler’s Clothes, 14 GP; or (B) 50 GP\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` background ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:backgrounds/soldier ` |
| ` implementation ` | ` dnd2024:backgrounds_soldier ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` ability_score_choices ` | ` ["strength", "dexterity", "constitution"] ` |
| ` skill_proficiencies ` | ` ["athletics", "intimidation"] ` |

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
| ` key ` | ` dnd2024:feats/origin_feats_savage_attacker ` |

#### Tool proficiency

<!-- splat:values path=splat_rule/parameters/tool_proficiency -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` category ` | ` gaming_set ` |
| ` selection ` | ` actor_choice ` |

#### Equipment

<!-- splat:yaml path=splat_rule/parameters/equipment -->

```yaml
choose: 1
options:
  equipment:
    items:
    - definition:
        key: dnd2024:equipment/spear
      quantity: 1
    - definition:
        key: dnd2024:equipment/shortbow
      quantity: 1
    - definition:
        key: dnd2024:equipment/ammunition_varies
      quantity: 20
    - definition:
        key: dnd2024:equipment/healer_s_kit_5_gp
      quantity: 1
    - definition:
        key: dnd2024:equipment/quiver_1_gp
      quantity: 1
    - definition:
        key: dnd2024:equipment/clothes_traveler_s_2_gp
      quantity: 1
    currency:
      gp: 14
    ammunition_variant: arrows
    matching_proficient_gaming_set:
      quantity: 1
  currency:
    gp: 50
```
