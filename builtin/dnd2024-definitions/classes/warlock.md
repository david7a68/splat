---
splat_schema: 1
splat_id: b2637fe0-2148-5654-b2c3-916cac461834
splat_kind: rule_group
splat_key: dnd2024:classes/warlock
splat_name: Warlock
splat_origin:
  key: dnd2024:reference/pages/070
splat_sources:
- title: SRD 5.2.1
  locator: pp. 70
  url: https://www.dndbeyond.com/srd
---

# Warlock

## Source text

<!-- SRD page 70 -->

Core Warlock Traits\
Primary Ability Charisma\
Hit Point Die D8 per Warlock level\
Saving Throw Wisdom and Charisma\
Proficiencies\
Skill Proficiencies Choose 2: Arcana, Deception,\
History, Intimidation, Investi-\
gation, Nature, or Religion\
Weapon Proficiencies Simple weapons\
Armor Training Light armor\
Starting Equipment Choose A or B: (A) Leather\
Armor, Sickle, 2 Daggers, Ar-\
cane Focus (orb), Book (occult\
lore), Scholar’s Pack, and 15\
GP; or (B) 100 GP\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` class ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:classes/warlock ` |
| ` implementation ` | ` dnd2024:classes_warlock ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` warlock ` |
| ` primary_abilities ` | ` ["charisma"] ` |

#### Hit point die

<!-- splat:values path=splat_rule/parameters/hit_point_die -->

| Field | Value |
| --- | --- |
| ` count_per_class_level ` | ` 1 ` |
| ` sides ` | ` 8 ` |

#### First class traits

<!-- splat:yaml path=splat_rule/parameters/first_class_traits -->

```yaml
save_proficiencies:
- wisdom
- charisma
skill_proficiencies:
  count: 2
  choices:
  - arcana
  - deception
  - history
  - intimidation
  - investigation
  - nature
  - religion
weapon_proficiencies:
  simple: true
armor_training:
- light
```

#### Starting equipment

<!-- splat:yaml path=splat_rule/parameters/starting_equipment -->

```yaml
choose: 1
options:
  equipment:
    items:
    - definition:
        key: dnd2024:equipment/leather_armor
      quantity: 1
    - definition:
        key: dnd2024:equipment/sickle
      quantity: 1
    - definition:
        key: dnd2024:equipment/dagger
      quantity: 2
    - definition:
        key: dnd2024:equipment/arcane_focus_varies
      quantity: 1
      variant: orb
    - definition:
        key: dnd2024:equipment/book_25_gp
      quantity: 1
      variant: occult_lore
    - definition:
        key: dnd2024:equipment/scholar_s_pack_40_gp
      quantity: 1
    currency:
      gp: 15
  currency:
    currency:
      gp: 100
```

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/warlock_progression ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | minimum_class_level |
| --- | --- |
| ` 0 ` | ` 11 ` |
| ` 1 ` | ` 19 ` |
| ` 2 ` | ` 1 ` |
| ` 3 ` | ` 1 ` |
| ` 4 ` | ` 20 ` |
| ` 5 ` | ` 2 ` |
| ` 6 ` | ` 3 ` |
| ` 7 ` | ` 4 ` |
| ` 8 ` | ` 9 ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/0/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_11_mystic_arcanum ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/1/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_19_epic_boon ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/2/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_1_eldritch_invocations ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/3/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_1_pact_magic ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/4/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_20_eldritch_master ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/5/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_2_magical_cunning ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/6/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_3_warlock_subclass ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/7/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_4_ability_score_improvement ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/8/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_9_contact_patron ` |
