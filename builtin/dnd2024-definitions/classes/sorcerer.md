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

<!-- splat:values path=splat_rule/parameters/starting_equipment -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |

##### Options

<!-- splat:table
path: splat_rule/parameters/starting_equipment/options
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
| ` A ` | [Rules](#^splat-93071aae-a "section:a") |
| ` B ` | [Rules](#^splat-93071aae-b "section:b") |

###### A

^splat-93071aae-a

###### Grants

<!-- splat:table
path: splat_rule/parameters/starting_equipment/options/@a/grants
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
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
| ` row_1 ` | ` Spear ` | ` 1 ` |
| ` row_2 ` | ` Dagger ` | ` 2 ` |
| ` row_3 ` | ` Arcane focus crystal ` | ` 1 ` |
| ` row_4 ` | ` Dungeoneer s pack ` | ` 1 ` |
| ` row_5 ` | ` Gp ` | ` 28 ` |

###### B

^splat-93071aae-b

###### Grants

<!-- splat:rows path=splat_rule/parameters/starting_equipment/options/@b/grants -->

| Row | item_type | quantity |
| --- | --- | --- |
| ` 0 ` | ` gp ` | ` 50 ` |

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/sorcerer_progression ` |

#### Feature grants

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Feature
order:
- sorcerer_level_19_epic_boon
- sorcerer_level_1_innate_sorcery
- sorcerer_level_1_spellcasting
- sorcerer_level_20_arcane_apotheosis
- sorcerer_level_2_font_of_magic
- sorcerer_level_2_metamagic
- sorcerer_level_3_sorcerer_subclass
- sorcerer_level_4_ability_score_improvement
- sorcerer_level_5_sorcerous_restoration
- sorcerer_level_7_sorcery_incarnate
columns:
  Feature:
    path: feature
    type: reference
  Minimum class level:
    path: minimum_class_level
    type: integer
-->

| Feature | Minimum class level |
| --- | --- |
| [Level 19: Epic Boon](<../class_features/sorcerer_level_19_epic_boon.md> "dnd2024:class_features/sorcerer_level_19_epic_boon") | ` 19 ` |
| [Level 1: Innate Sorcery](<../class_features/sorcerer_level_1_innate_sorcery.md> "dnd2024:class_features/sorcerer_level_1_innate_sorcery") | ` 1 ` |
| [Level 1: Spellcasting](<../class_features/sorcerer_level_1_spellcasting.md> "dnd2024:class_features/sorcerer_level_1_spellcasting") | ` 1 ` |
| [Level 20: Arcane Apotheosis](<../class_features/sorcerer_level_20_arcane_apotheosis.md> "dnd2024:class_features/sorcerer_level_20_arcane_apotheosis") | ` 20 ` |
| [Level 2: Font of Magic](<../class_features/sorcerer_level_2_font_of_magic.md> "dnd2024:class_features/sorcerer_level_2_font_of_magic") | ` 2 ` |
| [Level 2: Metamagic](<../class_features/sorcerer_level_2_metamagic.md> "dnd2024:class_features/sorcerer_level_2_metamagic") | ` 2 ` |
| [Level 3: Sorcerer Subclass](<../class_features/sorcerer_level_3_sorcerer_subclass.md> "dnd2024:class_features/sorcerer_level_3_sorcerer_subclass") | ` 3 ` |
| [Level 4: Ability Score Improvement](<../class_features/sorcerer_level_4_ability_score_improvement.md> "dnd2024:class_features/sorcerer_level_4_ability_score_improvement") | ` 4 ` |
| [Level 5: Sorcerous Restoration](<../class_features/sorcerer_level_5_sorcerous_restoration.md> "dnd2024:class_features/sorcerer_level_5_sorcerous_restoration") | ` 5 ` |
| [Level 7: Sorcery Incarnate](<../class_features/sorcerer_level_7_sorcery_incarnate.md> "dnd2024:class_features/sorcerer_level_7_sorcery_incarnate") | ` 7 ` |
