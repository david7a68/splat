---
splat_schema: 2
splat_id: 2b24f1ab-a317-5e70-8e90-e8de54d7e1d6
splat_kind: rule_group
splat_key: dnd2024:backgrounds/soldier
splat_name: Soldier
splat_origin: dnd2024:reference/pages/083
splat_sources:
- SRD 5.2.1, pp. 83, https://www.dndbeyond.com/srd
---

# Soldier

## Source text

<!-- SRD page 83 -->

Ability Scores: Strength, Dexterity, Constitution\
Feat: Savage Attacker (see “Feats”)\
Skill Proficiencies: Athletics and Intimidation\
Tool Proficiency: Choose one kind of Gaming Set (see “Equipment”)\
Equipment: Choose A or B: (A) Spear, Shortbow, 20 Arrows, Gaming Set (same as above), Healer’s Kit, Quiver, Traveler’s Clothes, 14 GP; or (B) 50 GP

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

<!-- splat:values path=splat_rule/parameters/equipment -->

| Field | Value |
| --- | --- |
| ` choose ` | ` 1 ` |

##### Options

###### Equipment

<!-- splat:values path=splat_rule/parameters/equipment/options/equipment -->

| Field | Value |
| --- | --- |
| ` ammunition_variant ` | ` arrows ` |

###### Items

<!-- splat:table
path: splat_rule/parameters/equipment/options/equipment/items
key: Item
order:
- spear
- shortbow
- ammunition_varies
- healer_s_kit_5_gp
- quiver_1_gp
- clothes_traveler_s_2_gp
columns:
  Item:
    path: definition
    type: reference
  Quantity:
    path: quantity
    type: integer
-->

| Item | Quantity |
| --- | --- |
| [Spear](<../equipment/spear.md> "dnd2024:equipment/spear") | ` 1 ` |
| [Shortbow](<../equipment/shortbow.md> "dnd2024:equipment/shortbow") | ` 1 ` |
| [Ammunition (Varies)](<../equipment/ammunition_varies.md> "dnd2024:equipment/ammunition_varies") | ` 20 ` |
| [Healer’s Kit (5 GP)](<../equipment/healer_s_kit_5_gp.md> "dnd2024:equipment/healer_s_kit_5_gp") | ` 1 ` |
| [Quiver (1 GP)](<../equipment/quiver_1_gp.md> "dnd2024:equipment/quiver_1_gp") | ` 1 ` |
| [Clothes, Traveler’s (2 GP)](<../equipment/clothes_traveler_s_2_gp.md> "dnd2024:equipment/clothes_traveler_s_2_gp") | ` 1 ` |

###### Currency

<!-- splat:values path=splat_rule/parameters/equipment/options/equipment/currency -->

| Field | Value |
| --- | --- |
| ` gp ` | ` 14 ` |

###### Matching proficient gaming set

<!-- splat:values path=splat_rule/parameters/equipment/options/equipment/matching_proficient_gaming_set -->

| Field | Value |
| --- | --- |
| ` quantity ` | ` 1 ` |

###### Currency

<!-- splat:values path=splat_rule/parameters/equipment/options/currency -->

| Field | Value |
| --- | --- |
| ` gp ` | ` 50 ` |
