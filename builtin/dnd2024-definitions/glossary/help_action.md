---
splat_schema: 1
splat_id: 7ae08674-eedb-525a-8926-276aeb4843fd
splat_kind: rule
splat_key: dnd2024:glossary/help_action
splat_name: Help [Action]
splat_origin:
  key: dnd2024:reference/glossary/help_action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182, 183
  url: https://www.dndbeyond.com/srd
---

# Help [Action]

## Source text

<!-- SRD page 182 -->

When you take the Help action, you do one of the following.

Assist an Ability Check. Choose one of your skill or tool proficiencies and one ally who is near enough for you to assist verbally or physically when they make an ability check. That ally has Advantage on the next ability check they make with the chosen skill or tool. This benefit expires if the ally doesn’t use it before the start of your next turn. The GM has final say on whether your assistance is possible.

<!-- SRD page 183 -->

Assist an Attack Roll. You momentarily distract an enemy within 5 feet of you, giving Advantage to the next attack roll by one of your allies against that enemy. This benefit expires at the start of your next turn.

Original layout: [[../../srd-5.2.1/pages/182|page 182]], [[../../srd-5.2.1/pages/183|page 183]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/help_action ` |
| ` implementation ` | ` dnd2024:glossary_help_action ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost ` | ` action ` |

#### Select one

<!-- splat:yaml path=splat_rule/parameters/select_one -->

```yaml
ability_check:
  choose_own_proficiency_kind:
  - skill
  - tool
  choose_one_ally: true
  proximity:
    verbal_or_physical_assistance_possible: true
    final_decision: gm
  advantage: next_ally_check_using_chosen_proficiency
  expires: own_next_turn_start
  consumed_after_matching_check: true
attack_roll:
  enemy_distance_feet_max: 5
  advantage: next_attack_by_any_one_ally_against_enemy
  expires: own_next_turn_start
  consumed_after_matching_attack: true
```
