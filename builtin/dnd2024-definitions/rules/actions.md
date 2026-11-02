---
splat_schema: 1
splat_id: 84a14b5e-aa13-542c-ac4c-cda3ccef4675
splat_kind: rule
splat_key: dnd2024:rules/actions
splat_name: Actions
splat_origin:
  key: dnd2024:reference/pages/009
splat_sources:
- title: SRD 5.2.1
  locator: pp. 9, 10
  url: https://www.dndbeyond.com/srd
---

# Actions

## Source text

<!-- SRD page 9 -->

When you do something other than moving or communicating, you typically take an action. The Action table lists the game’s main actions, which are defined in more detail in “Rules Glossary.”

**Actions**

| Action | Summary |
| --- | --- |
| Attack | Attack with a weapon or an Unarmed Strike. |
| Dash | For the rest of the turn, give yourself extra movement equal to your Speed. |

<!-- SRD page 10 -->

| Action | Summary |
| --- | --- |
| Disengage | Your movement doesn’t provoke Opportunity Attacks for the rest of the turn. |
| Dodge | Until the start of your next turn, attack rolls against you have Disadvantage, and you make Dexterity saving throws with Advantage. You lose this benefit if you have the Incapacitated condition or if your Speed is 0. |
| Help | Help another creature’s ability check or attack roll, or administer first aid. |
| Hide | Make a Dexterity (Stealth) check. |
| Influence | Make a Charisma (Deception, Intimidation, Performance, or Persuasion) or Wisdom (Animal Handling) check to alter a creature’s attitude. |
| Magic | Cast a spell, use a magic item, or use a magical feature. |
| Ready | Prepare to take an action in response to a trigger you define. |
| Search | Make a Wisdom (Insight, Medicine, Perception, or Survival) check. |
| Study | Make an Intelligence (Arcana, History, Investigation, Nature, or Religion) check. |
| Utilize | Use a nonmagical object. |

Player characters and monsters can also do things not covered by these actions. Many class features and other abilities provide additional action options, and you can improvise other actions. When you describe an action not detailed elsewhere in the rules, the Game Master tells you whether that action is possible and what kind of D20 Test you need to make, if any.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/actions ` |
| ` implementation ` | ` dnd2024:rules_actions ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` default_action_required_except ` | ` ["movement", "communication"] ` |
| ` additional_actions_source ` | ` ["class_features", "other_abilities"] ` |

#### Main actions

<!-- splat:rows path=splat_rule/parameters/main_actions -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:glossary/attack_action ` |
| ` 1 ` | ` dnd2024:glossary/dash_action ` |
| ` 2 ` | ` dnd2024:glossary/disengage_action ` |
| ` 3 ` | ` dnd2024:glossary/dodge_action ` |
| ` 4 ` | ` dnd2024:glossary/help_action ` |
| ` 5 ` | ` dnd2024:glossary/hide_action ` |
| ` 6 ` | ` dnd2024:glossary/influence_action ` |
| ` 7 ` | ` dnd2024:glossary/magic_action ` |
| ` 8 ` | ` dnd2024:glossary/ready_action ` |
| ` 9 ` | ` dnd2024:glossary/search_action ` |
| ` 10 ` | ` dnd2024:glossary/study_action ` |
| ` 11 ` | ` dnd2024:glossary/utilize_action ` |

#### Improvised action

<!-- splat:yaml path=splat_rule/parameters/improvised_action -->

```yaml
authority: gm
choices:
  possible: boolean
  test: optional_d20_test
```
