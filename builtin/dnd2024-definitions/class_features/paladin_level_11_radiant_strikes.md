---
splat_schema: 2
splat_id: 59dbb391-bf1e-5619-ae77-a55b240910d8
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_11_radiant_strikes
splat_name: 'Level 11: Radiant Strikes'
splat_origin: dnd2024:reference/pages/055
splat_sources:
- SRD 5.2.1, pp. 55, https://www.dndbeyond.com/srd
---

# Level 11: Radiant Strikes

## Source text

<!-- SRD page 55 -->

Your strikes now carry supernatural power. When you hit a target with an attack roll using a Melee weapon or an Unarmed Strike, the target takes an extra 1d8 Radiant damage.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/paladin_level_11_radiant_strikes ` |
| ` implementation ` | ` dnd2024:class_features_paladin_level_11_radiant_strikes ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` actor.attack_hits_target ` |
| ` attack_sources ` | ` ["melee_weapon", "unarmed_strike"] ` |

#### Extra damage

<!-- splat:yaml path=splat_rule/parameters/extra_damage -->

```yaml
dice:
  count: 1
  sides: 8
type: radiant
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` paladin ` |
| ` minimum_level ` | ` 11 ` |
