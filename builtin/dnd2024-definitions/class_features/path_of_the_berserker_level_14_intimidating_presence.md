---
splat_schema: 1
splat_id: 36badd6b-3407-5b39-9565-dd4d74569f8c
splat_kind: rule
splat_key: dnd2024:class_features/path_of_the_berserker_level_14_intimidating_presence
splat_name: 'Level 14: Intimidating Presence'
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
---

# Level 14: Intimidating Presence

## Source text

<!-- SRD page 30 -->

As a Bonus Action, you can strike terror into others with your menacing presence and primal power. When you do so, each creature of your choice in a 30-foot Emanation originating from you must make a Wisdom saving throw (DC 8 plus your Strength modifier and Proficiency Bonus). On a failed save, a creature has the Frightened condition for 1 minute. At the end of each of the Frightened creature’s turns, the creature repeats the save, ending the effect on itself on a success.

Once you use this feature, you can’t use it again until you finish a Long Rest unless you expend a use of your Rage (no action required) to restore your use of it.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/path_of_the_berserker_level_14_intimidating_presence ` |
| ` implementation ` | ` dnd2024:class_features_path_of_the_berserker_level_14_intimidating_presence ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |
| ` cost ` | ` 1 ` |

#### Resource

<!-- splat:yaml path=splat_rule/parameters/resource -->

```yaml
id: berserker.intimidating_presence
maximum: 1
recovery:
- event: rest.long_completed
  amount: all
```

#### Recharge alternative

<!-- splat:yaml path=splat_rule/parameters/recharge_alternative -->

```yaml
action: none
cost:
  resource: barbarian.rage
  amount: 1
restore_uses: 1
```

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` emanation ` |
| ` radius_feet ` | ` 30 ` |
| ` origin ` | ` actor ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` type ` | ` creature ` |
| ` selection ` | ` actor_choice ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc:
  op: add
  args:
  - 8
  - var: actor.abilities.strength.modifier
  - var: actor.proficiency_bonus
failure:
  condition: frightened
  source: actor
  duration:
    minutes: 1
repeat:
  event: target.turn.end
  success: end_effect
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` barbarian ` |
| ` minimum_level ` | ` 14 ` |
