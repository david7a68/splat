---
splat_schema: 2
splat_id: b6f3840e-2704-5a6c-bfc9-36632f08459a
splat_kind: rule_group
splat_key: dnd2024:backgrounds/criminal
splat_name: Criminal
splat_origin: dnd2024:reference/pages/083
splat_sources:
- SRD 5.2.1, pp. 83, https://www.dndbeyond.com/srd
---

# Criminal

## Source text

<!-- SRD page 83 -->

Ability Scores: Dexterity, Constitution, Intelligence\
Feat: Alert (see “Feats”)\
Skill Proficiencies: Sleight of Hand and Stealth\
Tool Proficiency: Thieves’ Tools\
Equipment: Choose A or B: (A) 2 Daggers, Thieves’ Tools, Crowbar, 2 Pouches, Traveler’s Clothes, 16 GP; or (B) 50 GP

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

<!-- splat:values path=splat_rule/parameters/equipment -->

| Field | Value |
| --- | --- |
| ` choose ` | ` 1 ` |

##### Options

###### Equipment

###### Items

<!-- splat:table
path: splat_rule/parameters/equipment/options/equipment/items
key: Item
order:
- dagger
- other_tools_thieves_tools_25_gp
- crowbar_2_gp
- pouch_5_sp
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
| [Dagger](<../equipment/dagger.md> "dnd2024:equipment/dagger") | ` 2 ` |
| [Thieves’ Tools (25 GP)](<../equipment/other_tools_thieves_tools_25_gp.md> "dnd2024:equipment/other_tools_thieves_tools_25_gp") | ` 1 ` |
| [Crowbar (2 GP)](<../equipment/crowbar_2_gp.md> "dnd2024:equipment/crowbar_2_gp") | ` 1 ` |
| [Pouch (5 SP)](<../equipment/pouch_5_sp.md> "dnd2024:equipment/pouch_5_sp") | ` 2 ` |
| [Clothes, Traveler’s (2 GP)](<../equipment/clothes_traveler_s_2_gp.md> "dnd2024:equipment/clothes_traveler_s_2_gp") | ` 1 ` |

###### Currency

<!-- splat:values path=splat_rule/parameters/equipment/options/equipment/currency -->

| Field | Value |
| --- | --- |
| ` gp ` | ` 16 ` |

###### Currency

<!-- splat:values path=splat_rule/parameters/equipment/options/currency -->

| Field | Value |
| --- | --- |
| ` gp ` | ` 50 ` |
