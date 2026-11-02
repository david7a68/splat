---
splat_schema: 1
splat_id: 1ce5ee88-00e0-5c6b-ac9c-36b851c8227b
splat_kind: spell
splat_key: dnd2024:spells/slow
splat_name: Slow
splat_origin:
  key: dnd2024:reference/spell/slow
splat_sources:
- title: SRD 5.2.1
  locator: pp. 163
  url: https://www.dndbeyond.com/srd
---

# Slow

## Source text

<!-- SRD page 163 -->

Level 3 Transmutation (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a drop of molasses)\
Duration: Concentration, up to 1 minute

You alter time around up to six creatures of your choice in a 40-foot Cube within range. Each target must succeed on a Wisdom saving throw or be affected by this spell for the duration.

An affected target’s Speed is halved, it takes a −2 penalty to AC and Dexterity saving throws, and it can’t take Reactions. On its turns, it can take either an action or a Bonus Action, not both, and it can make only one attack if it takes the Attack action. If it casts a spell with a Somatic component, there is a 25 percent chance the spell fails as a result of the target making the spell’s gestures too slowly.

An affected target repeats the save at the end of each of its turns, ending the spell on itself on a success.

Original layout: [[../../srd-5.2.1/pages/163|page 163]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["bard", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 120 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a drop of molasses ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/slow ` |
| ` implementation ` | ` dnd2024:spell_slow ` |

### Parameters

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cube ` |
| ` side_feet ` | ` 40 ` |

#### Target

<!-- splat:yaml path=splat_rule/parameters/target -->

```yaml
kind: creature
count:
  maximum: 6
visible_to_caster: false
```

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc:
  binding: save_dc
failure:
  affected: true
success:
  effect: none
```

#### Affected

<!-- splat:yaml path=splat_rule/parameters/affected -->

```yaml
speed_fraction:
  numerator: 1
  denominator: 2
ac_delta: -2
dexterity_save_delta: -2
reactions: false
on_turn_action_choice:
- action
- bonus_action
both_action_and_bonus_action: false
attack_action_maximum_attacks: 1
somatic_spell_failure_percent: 25
```

#### Repeat save

<!-- splat:values path=splat_rule/parameters/repeat_save -->

| Field | Value |
| --- | --- |
| ` event ` | ` turn.end ` |
| ` ability ` | ` wisdom ` |
| ` success ` | ` end_spell_on_self ` |
