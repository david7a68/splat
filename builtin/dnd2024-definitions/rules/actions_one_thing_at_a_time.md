---
splat_schema: 2
splat_id: 1a06b21e-f742-52dd-88b4-0b8d6480bb1a
splat_kind: rule
splat_key: dnd2024:rules/actions_one_thing_at_a_time
splat_name: One Thing at a Time
splat_origin: dnd2024:reference/pages/010
splat_sources:
- SRD 5.2.1, pp. 10, https://www.dndbeyond.com/srd
---

# One Thing at a Time

## Source text

<!-- SRD page 10 -->

The game uses actions to govern how much you can do at one time. You can take only one action at a time. This principle is most important in combat, as explained in “Combat” later in “Playing the Game.”

Actions can come up in other situations, too: in a social interaction, you can try to Influence a creature or use the Search action to read the creature’s body language, but you can’t do both at the same time. And when you’re exploring a dungeon, you can’t simultaneously use the Search action to look for traps and use the Help action to aid another character who’s trying to open a stuck door (with the Utilize action).

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/actions_one_thing_at_a_time ` |
| ` implementation ` | ` dnd2024:rule_actions_one_thing_at_a_time ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` maximum_simultaneous_actions ` | ` 1 ` |
| ` scope ` | ` ["combat", "social_interaction", "exploration"] ` |
