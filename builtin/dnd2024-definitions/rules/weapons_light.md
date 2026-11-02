---
splat_schema: 1
splat_id: 20d2b5d7-9345-53e5-8d54-8a688f803bc6
splat_kind: rule
splat_key: dnd2024:rules/weapons_light
splat_name: Light
splat_origin:
  key: dnd2024:reference/pages/089
splat_sources:
- title: SRD 5.2.1
  locator: pp. 89
  url: https://www.dndbeyond.com/srd
---

# Light

## Source text

<!-- SRD page 89 -->

When you take the Attack action on your turn and attack with a Light weapon, you can make one extra attack as a Bonus Action later on the same turn. That extra attack must be made with a different Light weapon, and you don’t add your ability modifier to the extra attack’s damage unless that modifier is negative. For example, you can attack with a Shortsword in one hand and a Dagger in the other using the Attack action and a Bonus Action, but you don’t add your Strength or Dexterity modifier to the damage roll of the Bonus Action unless that modifier is negative.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_light ` |
| ` implementation ` | ` dnd2024:rule_weapons_light ` |

### Parameters

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` action ` | ` attack ` |
| ` own_turn ` | ` true ` |
| ` weapon_property ` | ` light ` |

#### Extra attack

<!-- splat:yaml path=splat_rule/parameters/extra_attack -->

```yaml
count: 1
cost: bonus_action
timing: later_same_turn
different_weapon: true
weapon_property: light
damage_ability_modifier:
  op: min
  args:
  - 0
  - variable: attack_ability_modifier
```
