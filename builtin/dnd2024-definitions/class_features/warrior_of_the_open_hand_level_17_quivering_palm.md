---
splat_schema: 1
splat_id: 9fdaa056-6bc5-5ad5-b58e-23b119e5f15a
splat_kind: rule
splat_key: dnd2024:class_features/warrior_of_the_open_hand_level_17_quivering_palm
splat_name: 'Level 17: Quivering Palm'
splat_origin:
  key: dnd2024:reference/pages/052
splat_sources:
- title: SRD 5.2.1
  locator: pp. 52
  url: https://www.dndbeyond.com/srd
---

# Level 17: Quivering Palm

## Source text

<!-- SRD page 52 -->

You gain the ability to set up lethal vibrations in\
someone’s body. When you hit a creature with an\
Unarmed Strike, you can expend 4 Focus Points\
to start these imperceptible vibrations, which last\
for a number of days equal to your Monk level. The\
vibrations are harmless unless you take an action\
to end them. Alternatively, when you take the At-\
tack action on your turn, you can forgo one of the\
attacks to end the vibrations. To end them, you and\
the target must be on the same plane of existence.\
When you end them, the target must make a Consti-\
tution saving throw, taking 10d12 Force damage on\
a failed save or half as much damage on a successful\
one.\
You can have only one creature under the effect\
of this feature at a time. You can end the vibrations\
harmlessly (no action required).\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/warrior_of_the_open_hand_level_17_quivering_palm ` |
| ` implementation ` | ` dnd2024:class_features_warrior_of_the_open_hand_level_17_quivering_palm ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` maximum_active_creatures ` | ` 1 ` |

#### Apply

<!-- splat:yaml path=splat_rule/parameters/apply -->

```yaml
trigger: unarmed_strike.hit_creature
focus_cost: 4
duration:
  days:
    stat: classes.monk.level
perceptible: false
passive_damage: 0
```

#### Detonate

<!-- splat:values path=splat_rule/parameters/detonate -->

| Field | Value |
| --- | --- |
| ` requires_same_plane ` | ` true ` |
| ` ends_vibrations ` | ` true ` |

##### Action options

<!-- splat:table
path: splat_rule/parameters/detonate/action_options
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Action:
    path: action
    type: symbol
  Requires action:
    path: requires_action
    type: symbol
    optional: true
  Turn:
    path: turn
    type: symbol
    optional: true
-->

| Key | Action | Requires action | Turn |
| --- | --- | --- | --- |
| ` row_1 ` | ` Action ` |  |  |
| ` row_2 ` | ` Replace one attack ` | ` Attack ` | ` Own ` |

##### Save

<!-- splat:yaml path=splat_rule/parameters/detonate/save -->

```yaml
ability: constitution
dc:
  op: add
  args:
  - 8
  - stat: abilities.wisdom.modifier
  - stat: proficiency_bonus
failure_damage:
  dice:
    count: 10
    sides: 12
success_damage_multiplier: 0.5
damage_type: force
```

#### Dismiss

<!-- splat:values path=splat_rule/parameters/dismiss -->

| Field | Value |
| --- | --- |
| ` action ` | ` none ` |
| ` harmless ` | ` true ` |
