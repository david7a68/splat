---
splat_schema: 1
splat_id: dc6c4b33-642c-5453-91e0-379837ef9905
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_advantage_disadvantage
splat_name: Advantage/Disadvantage
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7, 8
  url: https://www.dndbeyond.com/srd
---

# Advantage/Disadvantage

## Source text

<!-- SRD page 7 -->

Sometimes a D20 Test is modified by Advantage or Disadvantage. Advantage reflects the positive circumstances surrounding a d20 roll, while Disadvantage reflects negative circumstances.

You usually acquire Advantage or Disadvantage through the use of special abilities and actions. The GM can also decide that circumstances grant Advantage or impose Disadvantage.

### Heroic Inspiration

Sometimes the GM or a rule gives you Heroic Inspiration. If you have Heroic Inspiration, you can expend it to reroll any die immediately after rolling it, and you must use the new roll.

Only One at a Time. You can never have more than one instance of Heroic Inspiration. If something gives you Heroic Inspiration and you already have it, you can give it to a player character in your group who lacks it.

Gaining Heroic Inspiration. Your GM can give you Heroic Inspiration for a variety of reasons. Typically, GMs award it when you do something particularly heroic, in character, or entertaining. It’s a reward for making the game more fun for everyone playing.

Other rules might allow your character to gain Heroic Inspiration independent of the GM’s decision. For example, Human characters start each day with Heroic Inspiration.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests_advantage_disadvantage ` |
| ` implementation ` | ` dnd2024:rule_d20_tests_advantage_disadvantage ` |

### Parameters

#### Apply definitions

<!-- splat:rows path=splat_rule/parameters/apply_definitions -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:glossary/advantage ` |
| ` 1 ` | ` dnd2024:glossary/disadvantage ` |
| ` 2 ` | ` dnd2024:glossary/heroic_inspiration ` |

#### Circumstantial advantage

<!-- splat:values path=splat_rule/parameters/circumstantial_advantage -->

| Field | Value |
| --- | --- |
| ` actor ` | ` gm ` |
| ` choices ` | ` ["advantage", "disadvantage", "neither"] ` |

#### Heroic inspiration

<!-- splat:yaml path=splat_rule/parameters/heroic_inspiration -->

```yaml
maximum: 1
reroll:
  die_count: 1
  timing: immediately_after_roll
  must_use_new_roll: true
  consume: true
overflow:
  optional: true
  recipient: party_player_character_without_inspiration
gm_award:
  optional: true
human_daily_grant: true
```
