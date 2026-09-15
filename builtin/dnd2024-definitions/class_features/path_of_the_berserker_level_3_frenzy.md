---
splat_schema: 1
splat_id: f2e4240c-f3bf-5302-9f6c-66c6657609e2
splat_kind: rule
splat_key: dnd2024:class_features/path_of_the_berserker_level_3_frenzy
splat_name: 'Level 3: Frenzy'
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
---

# Level 3: Frenzy

## Source text

<!-- SRD page 30 -->

If you use Reckless Attack while your Rage is active,\
you deal extra damage to the first target you hit on\
your turn with a Strength-based attack. To deter-\
mine the extra damage, roll a number of d6s equal\
to your Rage Damage bonus, and add them together.\
The damage has the same type as the weapon or\
Unarmed Strike used for the attack.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/path_of_the_berserker_level_3_frenzy ` |
| ` implementation ` | ` dnd2024:class_features_path_of_the_berserker_level_3_frenzy ` |

### Parameters

#### When

<!-- splat:values path=splat_rule/parameters/when -->

| Field | Value |
| --- | --- |
| ` rage_active ` | ` true ` |
| ` reckless_attack_used ` | ` true ` |
| ` turn ` | ` self ` |
| ` attack_ability ` | ` strength ` |
| ` first_target_hit_on_turn ` | ` true ` |

#### Extra damage

<!-- splat:yaml path=splat_rule/parameters/extra_damage -->

```yaml
roll:
  dice:
    count:
      stat: rage.damage_bonus
    sides: 6
type: triggering_weapon_or_unarmed_strike_damage_type
```
