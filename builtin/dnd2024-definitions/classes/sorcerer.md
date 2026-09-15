---
splat_schema: 1
splat_id: 09ee2ce0-6490-5787-a99e-ab16a9386f75
splat_kind: rule_group
splat_key: dnd2024:classes/sorcerer
splat_name: Sorcerer
splat_origin:
  key: dnd2024:reference/pages/064
splat_sources:
- title: SRD 5.2.1
  locator: pp. 64
  url: https://www.dndbeyond.com/srd
---

# Sorcerer

## Source text

<!-- SRD page 64 -->

Core Sorcerer Traits\
Primary Ability Charisma\
Hit Point Die D6 per Sorcerer level\
Saving Throw Constitution and Charisma\
Proficiencies\
Skill Proficiencies Choose 2: Arcana, Deception,\
Insight, Intimidation, Persua-\
sion, or Religion\
Weapon Proficiencies Simple weapons\
Armor Training None\
Starting Equipment Choose A or B: (A) Spear,\
2 Daggers, Arcane Focus\
(crystal), Dungeoneer’s Pack,\
and 28 GP; or (B) 50 GP\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` class ` |

### Members

<!-- splat:rows path=splat_rule_group/members -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/sorcerer_level_19_epic_boon ` |
| ` 1 ` | ` dnd2024:class_features/sorcerer_level_1_innate_sorcery ` |
| ` 2 ` | ` dnd2024:class_features/sorcerer_level_1_spellcasting ` |
| ` 3 ` | ` dnd2024:class_features/sorcerer_level_20_arcane_apotheosis ` |
| ` 4 ` | ` dnd2024:class_features/sorcerer_level_2_font_of_magic ` |
| ` 5 ` | ` dnd2024:class_features/sorcerer_level_2_metamagic ` |
| ` 6 ` | ` dnd2024:class_features/sorcerer_level_3_sorcerer_subclass ` |
| ` 7 ` | ` dnd2024:class_features/sorcerer_level_4_ability_score_improvement ` |
| ` 8 ` | ` dnd2024:class_features/sorcerer_level_5_sorcerous_restoration ` |
| ` 9 ` | ` dnd2024:class_features/sorcerer_level_7_sorcery_incarnate ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:classes/sorcerer ` |
| ` implementation ` | ` dnd2024:classes_sorcerer ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` sorcerer ` |
| ` primary_abilities ` | ` ["charisma"] ` |
| ` saving_throw_proficiencies ` | ` ["constitution", "charisma"] ` |
| ` weapon_proficiencies ` | ` ["simple"] ` |

#### Hit point die

<!-- splat:values path=splat_rule/parameters/hit_point_die -->

| Field | Value |
| --- | --- |
| ` sides ` | ` 6 ` |
| ` per_class_level ` | ` 1 ` |

#### Skill proficiencies

<!-- splat:values path=splat_rule/parameters/skill_proficiencies -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` options ` | ` ["arcana", "deception", "insight", "intimidation", "persuasion", "religion"] ` |

#### Armor training

<!-- splat:yaml path=splat_rule/parameters/armor_training -->

```yaml
[]
```

#### Tool proficiencies

<!-- splat:yaml path=splat_rule/parameters/tool_proficiencies -->

```yaml
[]
```

#### Starting equipment

<!-- splat:yaml path=splat_rule/parameters/starting_equipment -->

```yaml
count: 1
options:
- id: a
  grants:
  - item_type: spear
    quantity: 1
  - item_type: dagger
    quantity: 2
  - item_type: arcane_focus_crystal
    quantity: 1
  - item_type: dungeoneer_s_pack
    quantity: 1
  - item_type: gp
    quantity: 28
- id: b
  grants:
  - item_type: gp
    quantity: 50
```

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/sorcerer_progression ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | minimum_class_level |
| --- | --- |
| ` 0 ` | ` 19 ` |
| ` 1 ` | ` 1 ` |
| ` 2 ` | ` 1 ` |
| ` 3 ` | ` 20 ` |
| ` 4 ` | ` 2 ` |
| ` 5 ` | ` 2 ` |
| ` 6 ` | ` 3 ` |
| ` 7 ` | ` 4 ` |
| ` 8 ` | ` 5 ` |
| ` 9 ` | ` 7 ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_19_epic_boon ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_1_innate_sorcery ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_1_spellcasting ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/3/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_20_arcane_apotheosis ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/4/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_2_font_of_magic ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/5/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_2_metamagic ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/6/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_3_sorcerer_subclass ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/7/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_4_ability_score_improvement ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/8/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_5_sorcerous_restoration ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/9/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_7_sorcery_incarnate ` |
