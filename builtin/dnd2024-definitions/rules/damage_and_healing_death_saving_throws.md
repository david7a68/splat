---
splat_schema: 1
splat_id: 27e4fbaf-5e7c-5c22-99b0-3bcd2ba57d77
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_death_saving_throws
splat_name: Death Saving Throws
splat_origin:
  key: dnd2024:reference/pages/017
splat_sources:
- title: SRD 5.2.1
  locator: pp. 17, 18
  url: https://www.dndbeyond.com/srd
---

# Death Saving Throws

## Source text

<!-- SRD page 17 -->

Whenever you start your turn with 0 Hit Points, you must make a Death Saving Throw to determine whether you creep closer to death or hang on to life. Unlike other saving throws, this one isn’t tied to an ability score. You’re in the hands of fate now.

Three Successes/Failures. Roll 1d20. If the roll is 10 or higher, you succeed. Otherwise, you fail. A success or failure has no effect by itself. On your third success, you become Stable (see “Stabilizing a Character” below). On your third failure, you die.

The successes and failures don’t need to be consecutive; keep track of both until you collect three of a kind. The number of both is reset to zero when you regain any Hit Points or become Stable.

<!-- SRD page 18 -->

Rolling a 1 or 20. When you roll a 1 on the d20 for a Death Saving Throw, you suffer two failures. If you roll a 20 on the d20, you regain 1 Hit Point.

Damage at 0 Hit Points. If you take any damage while you have 0 Hit Points, you suffer a Death Saving Throw failure. If the damage is from a Critical Hit, you suffer two failures instead. If the damage equals or exceeds your Hit Point maximum, you die.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_death_saving_throws ` |
| ` implementation ` | ` dnd2024:rules_damage_and_healing_death_saving_throws ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` success_threshold ` | ` 3 ` |
| ` failure_threshold ` | ` 3 ` |
| ` third_success ` | ` stable ` |
| ` third_failure ` | ` dead ` |
| ` consecutive_required ` | ` false ` |
| ` reset_both_counters_on ` | ` ["hp_regained", "become_stable"] ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` event ` | ` turn.started ` |
| ` hp ` | ` 0 ` |
| ` stable ` | ` false ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
die:
  count: 1
  sides: 20
  add: 0
ability_modifier: null
dc: 10
```

#### Natural die overrides

<!-- splat:yaml path=splat_rule/parameters/natural_die_overrides -->

```yaml
'1':
  failures: 2
'20':
  heal_hp: 1
```

#### Damage at zero

<!-- splat:values path=splat_rule/parameters/damage_at_zero -->

| Field | Value |
| --- | --- |
| ` failure_increment ` | ` 1 ` |
| ` critical_hit_failure_increment ` | ` 2 ` |
| ` instant_death_when_damage_gte ` | ` hp_maximum ` |
