---
splat_schema: 1
splat_id: c69937e7-0b05-56d7-a464-f13afb2a5542
splat_kind: rule_group
splat_key: dnd2024:classes/wizard
splat_name: Wizard
splat_origin:
  key: dnd2024:reference/pages/077
splat_sources:
- title: SRD 5.2.1
  locator: pp. 77
  url: https://www.dndbeyond.com/srd
---

# Wizard

## Source text

<!-- SRD page 77 -->

Core Wizard Traits\
Primary Ability Intelligence\
Hit Point Die D6 per Wizard level\
Saving Throw Intelligence and Wisdom\
Proficiencies\
Skill Proficiencies Choose 2: Arcana, History, In-\
sight, Investigation, Medicine,\
Nature, or Religion\
Weapon Proficiencies Simple weapons\
Armor Training None\
Starting Equipment Choose A or B: (A) 2 Daggers,\
Arcane Focus (Quarterstaff),\
Robe, Spellbook, Scholar’s\
Pack, and 5 GP; or (B) 55 GP\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` class ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:classes/wizard ` |
| ` implementation ` | ` dnd2024:classes_wizard ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` wizard ` |
| ` primary_abilities ` | ` ["intelligence"] ` |

#### Hit point die

<!-- splat:values path=splat_rule/parameters/hit_point_die -->

| Field | Value |
| --- | --- |
| ` count_per_class_level ` | ` 1 ` |
| ` sides ` | ` 6 ` |

#### First class traits

<!-- splat:yaml path=splat_rule/parameters/first_class_traits -->

```yaml
save_proficiencies:
- intelligence
- wisdom
skill_proficiencies:
  count: 2
  choices:
  - arcana
  - history
  - insight
  - investigation
  - medicine
  - nature
  - religion
weapon_proficiencies:
  simple: true
armor_training: []
```

#### Starting equipment

<!-- splat:yaml path=splat_rule/parameters/starting_equipment -->

```yaml
choose: 1
options:
  equipment:
    items:
    - definition:
        key: dnd2024:equipment/dagger
      quantity: 2
    - definition:
        key: dnd2024:equipment/arcane_focus_varies
      quantity: 1
      variant: quarterstaff
    - definition:
        key: dnd2024:equipment/robe_1_gp
      quantity: 1
    - definition:
        key: dnd2024:equipment/scholar_s_pack_40_gp
      quantity: 1
    currency:
      gp: 5
    additional_items:
    - kind: spellbook
      quantity: 1
  currency:
    currency:
      gp: 55
```

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/wizard_progression ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | minimum_class_level |
| --- | --- |
| ` 0 ` | ` 18 ` |
| ` 1 ` | ` 19 ` |
| ` 2 ` | ` 1 ` |
| ` 3 ` | ` 1 ` |
| ` 4 ` | ` 1 ` |
| ` 5 ` | ` 20 ` |
| ` 6 ` | ` 2 ` |
| ` 7 ` | ` 3 ` |
| ` 8 ` | ` 4 ` |
| ` 9 ` | ` 5 ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/0/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_18_spell_mastery ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/1/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_19_epic_boon ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/2/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_1_arcane_recovery ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/3/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_1_ritual_adept ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/4/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_1_spellcasting ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/5/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_20_signature_spells ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/6/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_2_scholar ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/7/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_3_wizard_subclass ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/8/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_4_ability_score_improvement ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/9/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_5_memorize_spell ` |
