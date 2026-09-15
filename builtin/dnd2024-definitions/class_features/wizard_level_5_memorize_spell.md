---
splat_schema: 1
splat_id: 4d84c83d-12cd-5247-a158-4881b408170c
splat_kind: rule
splat_key: dnd2024:class_features/wizard_level_5_memorize_spell
splat_name: 'Level 5: Memorize Spell'
splat_origin:
  key: dnd2024:reference/pages/079
splat_sources:
- title: SRD 5.2.1
  locator: pp. 79
  url: https://www.dndbeyond.com/srd
---

# Level 5: Memorize Spell

## Source text

<!-- SRD page 79 -->

Whenever you finish a Short Rest, you can study\
your spellbook and replace one of the level 1+ Wiz-\
ard spells you have prepared for your Spellcasting\
feature with another level 1+ spell from the book.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/wizard_level_5_memorize_spell ` |
| ` implementation ` | ` dnd2024:class_features_wizard_level_5_memorize_spell ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` requires ` | ` study_own_spellbook ` |

#### Replace prepared spells

<!-- splat:values path=splat_rule/parameters/replace_prepared_spells -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 1 ` |
| ` feature ` | ` wizard_spellcasting ` |
| ` minimum_spell_level ` | ` 1 ` |
| ` replacement_from ` | ` own_spellbook ` |
| ` replacement_minimum_level ` | ` 1 ` |
