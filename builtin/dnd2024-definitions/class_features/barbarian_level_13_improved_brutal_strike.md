---
splat_schema: 1
splat_id: f33d1574-3ff6-5ba9-b574-615ece66b436
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_13_improved_brutal_strike
splat_name: 'Level 13: Improved Brutal Strike'
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
---

# Level 13: Improved Brutal Strike

## Source text

<!-- SRD page 30 -->

You have honed new ways to attack furiously. The following effects are now among your Brutal Strike options.

Staggering Blow. The target has Disadvantage on the next saving throw it makes, and it can’t make Opportunity Attacks until the start of your next turn.

Sundering Blow. Before the start of your next turn, the next attack roll made by another creature against the target gains a +5 bonus to the roll. An attack roll can gain only one Sundering Blow bonus.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/barbarian_level_13_improved_brutal_strike ` |
| ` implementation ` | ` dnd2024:class_features_barbarian_level_13_improved_brutal_strike ` |

### Parameters

#### Extends

<!-- splat:values path=splat_rule/parameters/extends -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_9_brutal_strike ` |

#### Options

<!-- splat:yaml path=splat_rule/parameters/options -->

```yaml
staggering_blow:
  next_saving_throw: disadvantage
  opportunity_attacks: false
  opportunity_attack_prohibition_end: actor.next_turn.start
sundering_blow:
  next_attack:
    attacker_not_actor: true
    target: struck_creature
    bonus: 5
    before: actor.next_turn.start
    maximum_stacking_instances: 1
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` barbarian ` |
| ` minimum_level ` | ` 13 ` |
