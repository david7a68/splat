---
splat_schema: 2
splat_id: 8c1a7c72-b2d4-5cae-90b9-f383b5ecd878
splat_kind: rule
splat_key: dnd2024:rules/wizard_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin: dnd2024:reference/pages/077
splat_sources:
- SRD 5.2.1, pp. 77, https://www.dndbeyond.com/srd
---

# As a Level 1 Character

## Source text

<!-- SRD page 77 -->

•  Gain all the traits in the Core Wizard Traits table.

•  Gain the Wizard’s level 1 features, which are listed in the Wizard Features table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/wizard_as_a_level_1_character ` |
| ` implementation ` | ` dnd2024:rules_wizard_as_a_level_1_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` grant_core_traits ` | ` all ` |

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/wizard ` |

#### Grant features

<!-- splat:values path=splat_rule/parameters/grant_features -->

| Field | Value |
| --- | --- |
| ` class_level ` | ` 1 ` |
| ` from ` | ` class_definition ` |

#### Feature unlocks

<!-- splat:table
path: splat_rule/parameters/feature_unlocks
key: Feature
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
  Feature:
    path: feature
    type: reference
  Class level:
    path: class_level
    type: integer
-->

| Feature | Class level |
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

#### Recurring feature levels

<!-- splat:values path=splat_rule/parameters/recurring_feature_levels -->

| Field | Value |
| --- | --- |
| ` ability_score_improvement ` | ` [4, 8, 12, 16] ` |
| ` subclass ` | ` [3, 6, 10, 14] ` |
