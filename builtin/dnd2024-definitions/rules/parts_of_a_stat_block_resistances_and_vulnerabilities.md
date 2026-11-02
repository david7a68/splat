---
splat_schema: 1
splat_id: 9c19c257-7d4f-5754-aed2-0b8870ad2861
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_resistances_and_vulnerabilities
splat_name: Resistances and Vulnerabilities
splat_origin:
  key: dnd2024:reference/pages/255
splat_sources:
- title: SRD 5.2.1
  locator: pp. 255
  url: https://www.dndbeyond.com/srd
---

# Resistances and Vulnerabilities

## Source text

<!-- SRD page 255 -->

These entries list a monster’s Resistances and Vulnerabilities, if any. See “Playing the Game” for more information.

### Running A Monster

To ensure a monster acts in accordance with its Challenge Rating, follow these rules during combat:

Special Abilities. If the monster has a special ability that deals a lot of damage but has a limited number of uses, such as a recharging breath weapon or a spell it can cast only once per day, have it use that special ability as quickly and as often as possible.

Multiattack. If the monster has Multiattack, have it use Multiattack on any of its turns in which it’s not using one of its more powerful abilities.

Bonus Actions, Reactions, Legendary Actions. If the monster has Bonus Actions, Reactions, or Legendary Actions in its stat block, make sure it uses them as often as it can.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_resistances_and_vulnerabilities ` |
| ` implementation ` | ` dnd2024:rules_parts_of_a_stat_block_resistances_and_vulnerabilities ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_modifiers_source ` | ` stat_block_resistance_and_vulnerability_entries ` |

#### Damage rule

<!-- splat:values path=splat_rule/parameters/damage_rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/damage_and_healing_resistance_and_vulnerability ` |

#### Combat tactics guidance

<!-- splat:values path=splat_rule/parameters/combat_tactics_guidance -->

| Field | Value |
| --- | --- |
| ` special_limited_abilities ` | ` as_early_and_often_as_possible ` |
| ` multiattack ` | ` unless_more_powerful_ability_used ` |
| ` bonus_reaction_legendary_actions ` | ` as_often_as_possible ` |
