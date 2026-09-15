---
splat_schema: 1
splat_id: e102040a-fbb9-5eb3-9f47-0dae6fe81a30
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_14_devious_strikes
splat_name: 'Level 14: Devious Strikes'
splat_origin:
  key: dnd2024:reference/pages/063
splat_sources:
- title: SRD 5.2.1
  locator: pp. 63
  url: https://www.dndbeyond.com/srd
---

# Level 14: Devious Strikes

## Source text

<!-- SRD page 63 -->

You’ve practiced new ways to use your Sneak Attack\
deviously. The following effects are now among\
your Cunning Strike options.\
Daze (Cost: 2d6). The target must succeed on a\
Constitution saving throw, or on its next turn, it can\
do only one of the following: move or take an action\
or a Bonus Action.\
Knock Out (Cost: 6d6). The target must succeed\
on a Constitution saving throw, or it has the Uncon-\
scious condition for 1 minute or until it takes any\
damage. The Unconscious target repeats the save\
at the end of each of its turns, ending the effect on\
itself on a success.\
Obscure (Cost: 3d6). The target must succeed on\
a Dexterity saving throw, or it has the Blinded con-\
dition until the end of its next turn.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/rogue_level_14_devious_strikes ` |
| ` implementation ` | ` dnd2024:class_features_rogue_level_14_devious_strikes ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` extends ` | ` rogue.cunning_strike ` |
| ` cost_resource ` | ` sneak_attack.d6_dice ` |

#### Save dc

<!-- splat:yaml path=splat_rule/parameters/save_dc -->

```yaml
op: add
args:
- 8
- var: actor.abilities.dexterity.modifier
- var: actor.proficiency_bonus
```

#### Options

<!-- splat:yaml path=splat_rule/parameters/options -->

```yaml
daze:
  cost: 2
  save: constitution
  failure:
    during: target.next_turn
    activity_limit:
      choose_only_one:
      - move
      - action
      - bonus_action
knock_out:
  cost: 6
  save: constitution
  failure:
    condition: unconscious
    duration:
      minutes: 1
    ends_on:
    - target.damage_taken
    repeat_save:
      event: target.turn.end
      success: end_effect
obscure:
  cost: 3
  save: dexterity
  failure:
    condition: blinded
    end: target.next_turn.end
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` rogue ` |
| ` minimum_level ` | ` 14 ` |
