---
splat_schema: 1
splat_id: 0af128d9-a737-5412-b69f-21628b0ba7b2
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_2_reckless_attack
splat_name: 'Level 2: Reckless Attack'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29
  url: https://www.dndbeyond.com/srd
---

# Level 2: Reckless Attack

## Source text

<!-- SRD page 29 -->

You can throw aside all concern for defense to at-\
tack with increased ferocity. When you make your\
first attack roll on your turn, you can decide to at-\
tack recklessly. Doing so gives you Advantage on at-\
tack rolls using Strength until the start of your next\
turn, but attack rolls against you have Advantage\
during that time.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/barbarian_level_2_reckless_attack ` |
| ` implementation ` | ` dnd2024:class_features_barbarian_level_2_reckless_attack ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` optional ` | ` true ` |
| ` trigger ` | ` actor.turn.first_attack_roll ` |
| ` duration_end ` | ` actor.next_turn.start ` |

#### Advantage

<!-- splat:yaml path=splat_rule/parameters/advantage -->

```yaml
actor_attacks:
  ability: strength
attacks_against_actor: true
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` barbarian ` |
| ` minimum_level ` | ` 2 ` |
