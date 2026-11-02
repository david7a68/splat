---
splat_schema: 1
splat_id: 49f5b590-7cb3-502f-87a2-1ea109302e29
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_14_improved_blessed_strikes
splat_name: 'Level 14: Improved Blessed Strikes'
splat_origin:
  key: dnd2024:reference/pages/038
splat_sources:
- title: SRD 5.2.1
  locator: pp. 38
  url: https://www.dndbeyond.com/srd
---

# Level 14: Improved Blessed Strikes

## Source text

<!-- SRD page 38 -->

The option you chose for Blessed Strikes grows more powerful.

Divine Strike. The extra damage of your Divine Strike increases to 2d8.

Potent Spellcasting. When you cast a Cleric cantrip and deal damage to a creature with it, you can give vitality to yourself or another creature within 60 feet of yourself, granting a number of Temporary Hit Points equal to twice your Wisdom modifier.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/cleric_level_14_improved_blessed_strikes ` |
| ` implementation ` | ` dnd2024:class_features_cleric_level_14_improved_blessed_strikes ` |

### Parameters

#### Modify selected blessed strikes

<!-- splat:yaml path=splat_rule/parameters/modify_selected_blessed_strikes -->

```yaml
divine_strike:
  damage_dice:
    count: 2
    sides: 8
potent_spellcasting:
  trigger: cleric.cantrip_cast_deals_damage_to_creature
  optional: true
  target:
    type: creature
    self_allowed: true
    range_feet: 60
    count: 1
  temporary_hit_points:
    op: multiply
    args:
    - 2
    - var: actor.abilities.wisdom.modifier
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` cleric ` |
| ` minimum_level ` | ` 14 ` |
