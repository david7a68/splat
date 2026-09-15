---
splat_schema: 1
splat_id: afb1b1fd-06d5-5f96-9c0e-2abbb7260a86
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_9_brutal_strike
splat_name: 'Level 9: Brutal Strike'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29, 30
  url: https://www.dndbeyond.com/srd
---

# Level 9: Brutal Strike

## Source text

<!-- SRD page 29 -->

If you use Reckless Attack, you can forgo any Ad-\
vantage on one Strength-based attack roll of your\
choice on your turn. The chosen attack roll mustn’t\
have Disadvantage. If the chosen attack roll hits, the\
target takes an extra 1d10 damage of the same type\
dealt by the weapon or Unarmed Strike, and you can\
cause one Brutal Strike effect of your choice. You\
have the following effect options.\
Forceful Blow. The target is pushed 15 feet\
straight away from you. You can then move up to\
half your Speed straight toward the target without\
provoking Opportunity Attacks.\

<!-- SRD page 30 -->

Hamstring Blow. The target’s Speed is reduced by\
15 feet until the start of your next turn. A target can\
be affected by only one Hamstring Blow at a time—\
the most recent one.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/barbarian_level_9_brutal_strike ` |
| ` implementation ` | ` dnd2024:class_features_barbarian_level_9_brutal_strike ` |

### Parameters

#### Requires

<!-- splat:values path=splat_rule/parameters/requires -->

| Field | Value |
| --- | --- |
| ` reckless_attack_used ` | ` true ` |
| ` attack_ability ` | ` strength ` |
| ` attack_disadvantage ` | ` false ` |
| ` turn ` | ` actor ` |

#### Limit

<!-- splat:values path=splat_rule/parameters/limit -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` period ` | ` actor.turn ` |

#### Cost

<!-- splat:values path=splat_rule/parameters/cost -->

| Field | Value |
| --- | --- |
| ` forgo_all_advantage ` | ` true ` |

#### On hit

<!-- splat:yaml path=splat_rule/parameters/on_hit -->

```yaml
extra_damage:
  dice:
    count: 1
    sides: 10
  type: attack.weapon_or_unarmed_damage_type
choose_effects: 1
options:
  forceful_blow:
    push:
      feet: 15
      direction: straight_away_from_actor
    follow_movement:
      maximum_feet:
        op: multiply
        args:
        - var: actor.speed
        - 0.5
      direction: straight_toward_target
      opportunity_attacks: false
  hamstring_blow:
    speed_bonus_feet: -15
    end: actor.next_turn.start
    stacking: latest_only
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` barbarian ` |
| ` minimum_level ` | ` 9 ` |
