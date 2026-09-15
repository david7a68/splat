---
splat_schema: 1
splat_id: c5f74151-5da5-58ca-b7b9-b79d45c81d35
splat_kind: rule
splat_key: dnd2024:feats/origin_feats_savage_attacker
splat_name: Savage Attacker
splat_origin:
  key: dnd2024:reference/pages/087
splat_sources:
- title: SRD 5.2.1
  locator: pp. 87
  url: https://www.dndbeyond.com/srd
---

# Savage Attacker

## Source text

<!-- SRD page 87 -->

Origin Feat\
You’ve trained to deal particularly damaging\
strikes. Once per turn when you hit a target with\
a weapon, you can roll the weapon’s damage dice\
twice and use either roll against the target.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:feats/origin_feats_savage_attacker ` |
| ` implementation ` | ` dnd2024:feats_origin_feats_savage_attacker ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` feat_category ` | ` origin ` |
| ` trigger ` | ` actor.weapon_hits_target ` |
| ` optional ` | ` true ` |

#### Limit

<!-- splat:values path=splat_rule/parameters/limit -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` period ` | ` turn ` |

#### Damage dice

<!-- splat:values path=splat_rule/parameters/damage_dice -->

| Field | Value |
| --- | --- |
| ` scope ` | ` weapon_damage_dice ` |
| ` roll_count ` | ` 2 ` |
| ` choose_one_roll_total ` | ` true ` |
