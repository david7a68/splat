---
splat_schema: 2
splat_id: 5ef5564c-4227-52a8-b5ae-9e2113ffe8f1
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_5_cunning_strike
splat_name: 'Level 5: Cunning Strike'
splat_origin: dnd2024:reference/pages/063
splat_sources:
- SRD 5.2.1, pp. 63, https://www.dndbeyond.com/srd
---

# Level 5: Cunning Strike

## Source text

<!-- SRD page 63 -->

You’ve developed cunning ways to use your Sneak Attack. When you deal Sneak Attack damage, you can add one of the following Cunning Strike effects. Each effect has a die cost, which is the number of Sneak Attack damage dice you must forgo to add the effect. You remove the die before rolling, and the effect occurs immediately after the attack’s damage is dealt. For example, if you add the Poison effect, remove 1d6 from the Sneak Attack’s damage before rolling.

If a Cunning Strike effect requires a saving throw, the DC equals 8 plus your Dexterity modifier and Proficiency Bonus.

Poison (Cost: 1d6). You add a toxin to your strike, forcing the target to make a Constitution saving throw. On a failed save, the target has the Poisoned condition for 1 minute. At the end of each of its turns, the Poisoned target repeats the save, ending the effect on itself on a success.

To use this effect, you must have a Poisoner’s Kit on your person.

Trip (Cost: 1d6). If the target is Large or smaller, it must succeed on a Dexterity saving throw or have the Prone condition.

Withdraw (Cost: 1d6). Immediately after the attack, you move up to half your Speed without provoking Opportunity Attacks.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/rogue_level_5_cunning_strike ` |
| ` implementation ` | ` dnd2024:class_features_rogue_level_5_cunning_strike ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` sneak_attack.damage_pending ` |
| ` maximum_effect_choices ` | ` 1 ` |
| ` effect_timing ` | ` immediately_after_attack_damage ` |

#### Cost

<!-- splat:values path=splat_rule/parameters/cost -->

| Field | Value |
| --- | --- |
| ` resource ` | ` sneak_attack_damage_dice ` |
| ` remove_before_roll ` | ` true ` |

#### Save dc

<!-- splat:yaml path=splat_rule/parameters/save_dc -->

```yaml
op: add
args:
- 8
- stat: abilities.dexterity.modifier
- stat: proficiency_bonus
```

#### Options

<!-- splat:rows path=splat_rule/parameters/options -->

| Row | id | cost_d6 | requires_carried_item | maximum_target_size |
| --- | --- | --- | --- | --- |
| ` 0 ` | ` poison ` | ` 1 ` | ` poisoners_kit ` |  |
| ` 1 ` | ` trip ` | ` 1 ` |  | ` large ` |
| ` 2 ` | ` withdraw ` | ` 1 ` |  |  |

##### Save

<!-- splat:yaml path=splat_rule/parameters/options/0/save -->

```yaml
ability: constitution
failure_condition: poisoned
duration:
  minutes: 1
repeat:
  event: target.turn_ended
  success_ends: true
```

##### Save

<!-- splat:values path=splat_rule/parameters/options/1/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |
| ` failure_condition ` | ` prone ` |

##### Move

<!-- splat:yaml path=splat_rule/parameters/options/2/move -->

```yaml
maximum_feet:
  op: divide
  args:
  - stat: speed
  - 2
provokes_opportunity_attacks: false
```
