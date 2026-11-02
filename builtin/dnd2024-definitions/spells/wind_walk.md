---
splat_schema: 1
splat_id: 36dc5451-c203-5802-842e-2e7b01d30f57
splat_kind: spell
splat_key: dnd2024:spells/wind_walk
splat_name: Wind Walk
splat_origin:
  key: dnd2024:reference/spell/wind_walk
splat_sources:
- title: SRD 5.2.1
  locator: pp. 174
  url: https://www.dndbeyond.com/srd
---

# Wind Walk

## Source text

<!-- SRD page 174 -->

Level 6 Transmutation (Druid)\
Casting Time: 1 minute\
Range: 30 feet\
Components: V, S, M (a candle)\
Duration: 8 hours

You and up to ten willing creatures of your choice within range assume gaseous forms for the duration, appearing as wisps of cloud. While in this cloud form, a target has a Fly Speed of 300 feet and can hover; it has Immunity to the Prone condition; and it has Resistance to Bludgeoning, Piercing, and Slashing damage. The only actions a target can take in this form are the Dash action or a Magic action to begin reverting to its normal form. Reverting takes 1 minute, during which the target has the Stunned condition. Until the spell ends, the target can revert to cloud form, which also requires a Magic action followed by a 1-minute transformation.

If a target is in cloud form and flying when the effect ends, the target descends 60 feet per round for 1 minute until it lands, which it does safely. If it can’t land after 1 minute, it falls the remaining distance.

Original layout: [[../../srd-5.2.1/pages/174|page 174]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 6 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["druid"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 8 hours ` |
| ` casting_time ` | ` 1 minute ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a candle ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/wind_walk ` |
| ` implementation ` | ` dnd2024:spell_wind_walk ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 10 ` |
| ` within_spell_range ` | ` true ` |
| ` willing ` | ` true ` |
| ` additional_target ` | ` caster ` |

#### Cloud form

<!-- splat:values path=splat_rule/parameters/cloud_form -->

| Field | Value |
| --- | --- |
| ` duration ` | ` spell_duration ` |
| ` appearance ` | ` cloud_wisp ` |
| ` fly_speed_feet ` | ` 300 ` |
| ` hover ` | ` true ` |
| ` condition_immunity ` | ` ["prone"] ` |
| ` damage_resistance ` | ` ["bludgeoning", "piercing", "slashing"] ` |
| ` only_actions ` | ` ["dash", "magic_action_begin_revert"] ` |

#### Transformations

<!-- splat:yaml path=splat_rule/parameters/transformations -->

```yaml
to_normal:
  cost: magic_action
  duration_minutes: 1
  during_condition: stunned
to_cloud:
  requires: spell_not_ended
  cost: magic_action
  duration_minutes: 1
```

#### End while flying in cloud

<!-- splat:yaml path=splat_rule/parameters/end_while_flying_in_cloud -->

```yaml
descent_feet_per_round: 60
maximum_minutes: 1
landing: safe
if_not_landed_after_minutes:
  minutes: 1
  effect: fall_remaining_distance
```
