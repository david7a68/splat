---
splat_schema: 1
splat_id: 6f298547-ba5a-5ff4-ab6f-7ddf3ce6e04a
splat_kind: rule
splat_key: dnd2024:rules/social_interaction_ability_checks
splat_name: Ability Checks
splat_origin:
  key: dnd2024:reference/pages/011
splat_sources:
- title: SRD 5.2.1
  locator: pp. 11
  url: https://www.dndbeyond.com/srd
---

# Ability Checks

## Source text

<!-- SRD page 11 -->

Ability checks can be key in determining the out-\
come of a social interaction. Your roleplaying efforts\
can alter an NPC’s attitude, but there might still be\
an element of chance if the GM wants dice to play\
a role in determining an NPC’s response to you. In\
such situations, the GM will typically ask you to\
take the Influence action.\
Pay attention to your skill proficiencies when\
thinking of how you will interact with an NPC; use\
an approach that relies on your group’s skill pro-\
ficiencies. For example, if the group needs to trick\
a guard into letting them into a castle, the Rogue\
who is proficient in Deception should lead the\
discussion.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/social_interaction_ability_checks ` |
| ` implementation ` | ` dnd2024:rules_social_interaction_ability_checks ` |

### Parameters

#### Roleplaying attitude change

<!-- splat:values path=splat_rule/parameters/roleplaying_attitude_change -->

| Field | Value |
| --- | --- |
| ` authority ` | ` gm ` |
| ` attitude ` | ` choice ` |

#### Uncertain response

<!-- splat:yaml path=splat_rule/parameters/uncertain_response -->

```yaml
authority: gm
roll_optional: true
action:
  key: dnd2024:glossary/influence_action
```
