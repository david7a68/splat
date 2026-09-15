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

<!-- splat:values path=splat_rule/parameters/starting_equipment -->

| Field | Value |
| --- | --- |
| ` choose ` | ` 1 ` |

##### Options

###### Equipment

###### Items

<!-- splat:table
path: splat_rule/parameters/starting_equipment/options/equipment/items
key: Item
order:
- dagger
- arcane_focus_varies
- robe_1_gp
- scholar_s_pack_40_gp
columns:
  Item:
    path: definition
    type: reference
  Quantity:
    path: quantity
    type: integer
  Variant:
    path: variant
    type: symbol
    optional: true
-->

| Item | Quantity | Variant |
| --- | --- | --- |
| [Dagger](<../equipment/dagger.md> "dnd2024:equipment/dagger") | ` 2 ` |  |
| [Arcane Focus (Varies)](<../equipment/arcane_focus_varies.md> "dnd2024:equipment/arcane_focus_varies") | ` 1 ` | ` Quarterstaff ` |
| [Robe (1 GP)](<../equipment/robe_1_gp.md> "dnd2024:equipment/robe_1_gp") | ` 1 ` |  |
| [Scholar’s Pack (40 GP)](<../equipment/scholar_s_pack_40_gp.md> "dnd2024:equipment/scholar_s_pack_40_gp") | ` 1 ` |  |

###### Currency

<!-- splat:values path=splat_rule/parameters/starting_equipment/options/equipment/currency -->

| Field | Value |
| --- | --- |
| ` gp ` | ` 5 ` |

###### Additional items

<!-- splat:rows path=splat_rule/parameters/starting_equipment/options/equipment/additional_items -->

| Row | kind | quantity |
| --- | --- | --- |
| ` 0 ` | ` spellbook ` | ` 1 ` |

###### Currency

<!-- splat:yaml path=splat_rule/parameters/starting_equipment/options/currency -->

```yaml
currency:
  gp: 55
```

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/wizard_progression ` |

#### Feature grants

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Item
order:
- wizard_level_18_spell_mastery
- wizard_level_19_epic_boon
- wizard_level_1_arcane_recovery
- wizard_level_1_ritual_adept
- wizard_level_1_spellcasting
- wizard_level_20_signature_spells
- wizard_level_2_scholar
- wizard_level_3_wizard_subclass
- wizard_level_4_ability_score_improvement
- wizard_level_5_memorize_spell
columns:
  Item:
    path: definition
    type: reference
  Minimum class level:
    path: minimum_class_level
    type: integer
-->

| Item | Minimum class level |
| --- | --- |
| [Level 18: Spell Mastery](<../class_features/wizard_level_18_spell_mastery.md> "dnd2024:class_features/wizard_level_18_spell_mastery") | ` 18 ` |
| [Level 19: Epic Boon](<../class_features/wizard_level_19_epic_boon.md> "dnd2024:class_features/wizard_level_19_epic_boon") | ` 19 ` |
| [Level 1: Arcane Recovery](<../class_features/wizard_level_1_arcane_recovery.md> "dnd2024:class_features/wizard_level_1_arcane_recovery") | ` 1 ` |
| [Level 1: Ritual Adept](<../class_features/wizard_level_1_ritual_adept.md> "dnd2024:class_features/wizard_level_1_ritual_adept") | ` 1 ` |
| [Level 1: Spellcasting](<../class_features/wizard_level_1_spellcasting.md> "dnd2024:class_features/wizard_level_1_spellcasting") | ` 1 ` |
| [Level 20: Signature Spells](<../class_features/wizard_level_20_signature_spells.md> "dnd2024:class_features/wizard_level_20_signature_spells") | ` 20 ` |
| [Level 2: Scholar](<../class_features/wizard_level_2_scholar.md> "dnd2024:class_features/wizard_level_2_scholar") | ` 2 ` |
| [Level 3: Wizard Subclass](<../class_features/wizard_level_3_wizard_subclass.md> "dnd2024:class_features/wizard_level_3_wizard_subclass") | ` 3 ` |
| [Level 4: Ability Score Improvement](<../class_features/wizard_level_4_ability_score_improvement.md> "dnd2024:class_features/wizard_level_4_ability_score_improvement") | ` 4 ` |
| [Level 5: Memorize Spell](<../class_features/wizard_level_5_memorize_spell.md> "dnd2024:class_features/wizard_level_5_memorize_spell") | ` 5 ` |
