---
splat_schema: 2
splat_id: 63c848ff-ddfe-5e35-9e68-d069146287ac
splat_kind: rule
splat_key: dnd2024:rules/combat_impeded_weapons
splat_name: Impeded Weapons
splat_origin: dnd2024:reference/pages/016
splat_sources:
- SRD 5.2.1, pp. 16, https://www.dndbeyond.com/srd
---

# Impeded Weapons

## Source text

<!-- SRD page 16 -->

When making a melee attack roll with a weapon underwater, a creature that lacks a Swim Speed has Disadvantage on the attack roll unless the weapon deals Piercing damage.

A ranged attack roll with a weapon underwater automatically misses a target beyond the weapon’s normal range, and the attack roll has Disadvantage against a target within normal range.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_impeded_weapons ` |
| ` implementation ` | ` dnd2024:rule_combat_impeded_weapons ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` environment ` | ` underwater ` |

#### Melee weapon attack

<!-- splat:yaml path=splat_rule/parameters/melee_weapon_attack -->

```yaml
disadvantage_when:
  swim_speed: false
  damage_type_not: piercing
```

#### Ranged weapon attack

<!-- splat:values path=splat_rule/parameters/ranged_weapon_attack -->

| Field | Value |
| --- | --- |
| ` beyond_normal_range ` | ` automatic_miss ` |
| ` within_normal_range ` | ` disadvantage ` |
