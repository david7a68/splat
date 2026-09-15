---
splat_schema: 1
splat_id: d2f84f14-27ba-52fa-a151-bf2c2d7a2c31
splat_kind: rule
splat_key: dnd2024:class_features/oath_of_devotion_level_3_sacred_weapon
splat_name: 'Level 3: Sacred Weapon'
splat_origin:
  key: dnd2024:reference/pages/056
splat_sources:
- title: SRD 5.2.1
  locator: pp. 56
  url: https://www.dndbeyond.com/srd
---

# Level 3: Sacred Weapon

## Source text

<!-- SRD page 56 -->

When you take the Attack action, you can expend\
one use of your Channel Divinity to imbue one\
Melee weapon that you are holding with positive\
energy. For 10 minutes or until you use this feature\
again, you add your Charisma modifier to attack\
rolls you make with that weapon (minimum bonus\
of +1), and each time you hit with it, you cause it to\
deal its normal damage type or Radiant damage.\
The weapon also emits Bright Light in a 20-foot\
radius and Dim Light 20 feet beyond that.\
You can end this effect early (no action required).\
This effect also ends if you aren’t carrying the\
weapon.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/oath_of_devotion_level_3_sacred_weapon ` |
| ` implementation ` | ` dnd2024:class_features_oath_of_devotion_level_3_sacred_weapon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` actor.attack_action ` |
| ` optional ` | ` true ` |
| ` ends_on ` | ` ["feature.used_again", "weapon.not_carried_by_actor", "actor.voluntary_end_no_action"] ` |
| ` hit_damage_type_choice ` | ` ["normal", "radiant"] ` |

#### Resource cost

<!-- splat:values path=splat_rule/parameters/resource_cost -->

| Field | Value |
| --- | --- |
| ` id ` | ` paladin.channel_divinity ` |
| ` amount ` | ` 1 ` |

#### Target item

<!-- splat:values path=splat_rule/parameters/target_item -->

| Field | Value |
| --- | --- |
| ` type ` | ` melee_weapon ` |
| ` held_by_actor ` | ` true ` |
| ` count ` | ` 1 ` |

#### Duration

<!-- splat:values path=splat_rule/parameters/duration -->

| Field | Value |
| --- | --- |
| ` minutes ` | ` 10 ` |

#### Attack bonus

<!-- splat:yaml path=splat_rule/parameters/attack_bonus -->

```yaml
actor_only: true
weapon: selected
amount:
  op: max
  args:
  - 1
  - var: actor.abilities.charisma.modifier
```

#### Light

<!-- splat:values path=splat_rule/parameters/light -->

| Field | Value |
| --- | --- |
| ` bright_radius_feet ` | ` 20 ` |
| ` additional_dim_radius_feet ` | ` 20 ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` paladin ` |
| ` minimum_level ` | ` 3 ` |
