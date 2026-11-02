---
splat_schema: 1
splat_id: 1352e174-a8f3-5417-a28c-5aa8a211ab0d
splat_kind: rule
splat_key: dnd2024:rules/proficiency_equipment_proficiencies
splat_name: Equipment Proficiencies
splat_origin:
  key: dnd2024:reference/pages/009
splat_sources:
- title: SRD 5.2.1
  locator: pp. 9
  url: https://www.dndbeyond.com/srd
---

# Equipment Proficiencies

## Source text

<!-- SRD page 9 -->

A character gains proficiency with various weapons and tools from their class and background. There are two categories of equipment proficiency:

Weapons. Anyone can wield a weapon, but proficiency makes you better at wielding it. If you have proficiency with a weapon, you add your Proficiency Bonus to attack rolls you make with it.

Tools. If you have proficiency with a tool, you can add your Proficiency Bonus to any ability check you make that uses the tool. If you have proficiency in the skill that’s also used with that check, you have Advantage on the check too. This means you can benefit from both skill proficiency and tool proficiency on the same ability check.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/proficiency_equipment_proficiencies ` |
| ` implementation ` | ` dnd2024:rules_proficiency_equipment_proficiencies ` |

### Parameters

#### Weapon proficiency

<!-- splat:values path=splat_rule/parameters/weapon_proficiency -->

| Field | Value |
| --- | --- |
| ` required_to_wield ` | ` false ` |
| ` attack_bonus ` | ` proficiency_bonus ` |

#### Tool proficiency

<!-- splat:values path=splat_rule/parameters/tool_proficiency -->

| Field | Value |
| --- | --- |
| ` ability_check_bonus ` | ` proficiency_bonus ` |
| ` matching_skill_proficiency ` | ` advantage ` |
| ` proficiency_bonus_times ` | ` 1 ` |
