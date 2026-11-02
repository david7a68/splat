---
splat_schema: 1
splat_id: 7f2fd838-f544-55fe-96c6-0d192341287d
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_saving_throws
splat_name: Saving Throws
splat_origin:
  key: dnd2024:reference/pages/106
splat_sources:
- title: SRD 5.2.1
  locator: pp. 106
  url: https://www.dndbeyond.com/srd
---

# Saving Throws

## Source text

<!-- SRD page 106 -->

Many spells specify that a target makes a saving throw to avoid some or all of a spell’s effects. The spell specifies the ability that the target uses for the save and what happens on a success or failure. Here’s how to calculate the DC for your spells:

Spell save DC = 8 + your spellcasting ability modifier + your Proficiency Bonus

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/casting_spells_saving_throws ` |
| ` implementation ` | ` dnd2024:rules_casting_spells_saving_throws ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` save_ability_and_outcome_source ` | ` spell ` |

#### Dc

<!-- splat:values path=splat_rule/parameters/dc -->

| Field | Value |
| --- | --- |
| ` op ` | ` sum ` |
| ` terms ` | ` [8, "spellcasting_ability_modifier", "proficiency_bonus"] ` |
