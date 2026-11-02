---
splat_schema: 1
splat_id: 0ceb2480-62d9-5c92-819c-c28f6ff8a286
splat_kind: rule
splat_key: dnd2024:rules/social_interaction
splat_name: Social Interaction
splat_origin:
  key: dnd2024:reference/pages/010
splat_sources:
- title: SRD 5.2.1
  locator: pp. 10
  url: https://www.dndbeyond.com/srd
---

# Social Interaction

## Source text

<!-- SRD page 10 -->

During their adventures, player characters meet many different people and face some monsters that would rather talk than fight. In those situations, it’s time for social interaction, which takes many forms. For example, you might try to convince a burglar to confess to wrongdoing or try to flatter a guard. The Game Master assumes the roles of any nonplayer characters who are participating.

An NPC’s attitude toward your character is Friendly, Indifferent, or Hostile, as defined in “Rules Glossary.” Friendly NPCs are predisposed to help, and Hostile ones are inclined to hinder.

Social interactions progress in two ways: through roleplaying and ability checks.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/social_interaction ` |
| ` implementation ` | ` dnd2024:rule_social_interaction ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` npc_controller ` | ` gm ` |
| ` resolution_methods ` | ` ["roleplay", "ability_check"] ` |

#### Apply definitions

<!-- splat:rows path=splat_rule/parameters/apply_definitions -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:glossary/friendly_attitude ` |
| ` 1 ` | ` dnd2024:glossary/indifferent_attitude ` |
| ` 2 ` | ` dnd2024:glossary/hostile_attitude ` |
