---
splat_schema: 1
splat_id: 148f9e56-912a-5c76-a42e-de36e6d7c9eb
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_5_stunning_strike
splat_name: 'Level 5: Stunning Strike'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51
  url: https://www.dndbeyond.com/srd
---

# Level 5: Stunning Strike

## Source text

<!-- SRD page 51 -->

Once per turn when you hit a creature with a Monk weapon or an Unarmed Strike, you can expend 1 Focus Point to attempt a stunning strike. The target must make a Constitution saving throw. On a failed save, the target has the Stunned condition until the start of your next turn. On a successful save, the target’s Speed is halved until the start of your next turn, and the next attack roll made against the target before then has Advantage.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_5_stunning_strike ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_5_stunning_strike ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` focus_cost ` | ` 1 ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` hit_creature ` | ` true ` |
| ` attack_types ` | ` ["monk_weapon", "unarmed_strike"] ` |

#### Limit

<!-- splat:values path=splat_rule/parameters/limit -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` period ` | ` turn ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
dc:
  op: add
  args:
  - 8
  - stat: abilities.wisdom.modifier
  - stat: proficiency_bonus
failure:
  condition: stunned
  until: source.next_turn_start
success:
  speed_multiplier: 0.5
  next_attack_against_target_advantage: true
  until: source.next_turn_start
```
