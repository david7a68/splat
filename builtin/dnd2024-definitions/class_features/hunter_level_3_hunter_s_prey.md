---
splat_schema: 1
splat_id: ecc4959e-d0cc-55eb-a878-b0b56181e2e9
splat_kind: rule
splat_key: dnd2024:class_features/hunter_level_3_hunter_s_prey
splat_name: 'Level 3: Hunter’s Prey'
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
---

# Level 3: Hunter’s Prey

## Source text

<!-- SRD page 61 -->

You gain one of the following feature options of your choice. Whenever you finish a Short or Long Rest, you can replace the chosen option with the other one.

Colossus Slayer. Your tenacity can wear down even the most resilient foes. When you hit a creature with a weapon, the weapon deals an extra 1d8 damage to the target if it’s missing any of its Hit Points. You can deal this extra damage only once per turn.

Horde Breaker. Once on each of your turns when you make an attack with a weapon, you can make another attack with the same weapon against a different creature that is within 5 feet of the original target, that is within the weapon’s range, and that you haven’t attacked this turn.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/hunter_level_3_hunter_s_prey ` |
| ` implementation ` | ` dnd2024:class_features_hunter_level_3_hunter_s_prey ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` choose ` | ` 1 ` |

#### Replacement

<!-- splat:values path=splat_rule/parameters/replacement -->

| Field | Value |
| --- | --- |
| ` events ` | ` ["rest.short_completed", "rest.long_completed"] ` |
| ` choose_other_option ` | ` true ` |

#### Options

<!-- splat:yaml path=splat_rule/parameters/options -->

```yaml
colossus_slayer:
  trigger: actor.weapon_hits_creature
  requires:
    target_missing_hit_points: true
  extra_damage:
    dice:
      count: 1
      sides: 8
    type: weapon.damage_type
  limit:
    count: 1
    period: turn
horde_breaker:
  trigger: actor.weapon_attack
  limit:
    count: 1
    period: actor.turn
  extra_attack:
    weapon: same
    target:
      type: creature
      different_from_original: true
      within_original_target_feet: 5
      within_weapon_range: true
      not_attacked_by_actor_this_turn: true
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` ranger ` |
| ` minimum_level ` | ` 3 ` |
