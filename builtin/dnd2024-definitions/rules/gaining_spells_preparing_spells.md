---
splat_schema: 1
splat_id: 0c178b9c-6074-5e7b-aaff-f370eb1331f1
splat_kind: rule
splat_key: dnd2024:rules/gaining_spells_preparing_spells
splat_name: Preparing Spells
splat_origin:
  key: dnd2024:reference/pages/104
splat_sources:
- title: SRD 5.2.1
  locator: pp. 104
  url: https://www.dndbeyond.com/srd
---

# Preparing Spells

## Source text

<!-- SRD page 104 -->

If you have a list of level 1+ spells you prepare, your\
spellcasting feature specifies when you can change\
the list and the number of spells you can change, as\
summarized in the Spell Preparation by Class table.\
Spell Preparation by Class\
Class Change When You Number of Spells\
…\
Bard Gain a level One\
Cleric Finish a Long Rest Any\
Druid Finish a Long Rest Any\
Paladin Finish a Long Rest One\
Ranger Finish a Long Rest One\
Sorcerer Gain a level One\
Warlock Gain a level One\
Wizard Finish a Long Rest Any\
Most spellcasting monsters don’t change their lists\
of prepared spells, but the GM is free to alter them.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/gaining_spells_preparing_spells ` |
| ` implementation ` | ` dnd2024:rules_gaining_spells_preparing_spells ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` prepared_spells_minimum_level ` | ` 1 ` |

#### Class replacement rules

<!-- splat:rows path=splat_rule/parameters/class_replacement_rules -->

| Row | class | event | maximum_replacements |
| --- | --- | --- | --- |
| ` 0 ` | ` bard ` | ` level_gained ` | ` 1 ` |
| ` 1 ` | ` cleric ` | ` long_rest_finished ` | ` any ` |
| ` 2 ` | ` druid ` | ` long_rest_finished ` | ` any ` |
| ` 3 ` | ` paladin ` | ` long_rest_finished ` | ` 1 ` |
| ` 4 ` | ` ranger ` | ` long_rest_finished ` | ` 1 ` |
| ` 5 ` | ` sorcerer ` | ` level_gained ` | ` 1 ` |
| ` 6 ` | ` warlock ` | ` level_gained ` | ` 1 ` |
| ` 7 ` | ` wizard ` | ` long_rest_finished ` | ` any ` |

#### Monster prepared list change

<!-- splat:values path=splat_rule/parameters/monster_prepared_list_change -->

| Field | Value |
| --- | --- |
| ` default ` | ` false ` |
| ` gm_override ` | ` true ` |
