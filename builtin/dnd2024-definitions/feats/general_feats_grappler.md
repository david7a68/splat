---
splat_schema: 2
splat_id: 847a6cdf-014e-50fd-a776-79ec3e16b7f4
splat_kind: rule
splat_key: dnd2024:feats/general_feats_grappler
splat_name: Grappler
splat_origin: dnd2024:reference/pages/087
splat_sources:
- SRD 5.2.1, pp. 87, https://www.dndbeyond.com/srd
---

# Grappler

## Source text

<!-- SRD page 87 -->

General Feat (Prerequisite: Level 4+, Strength or Dexterity 13+)

You gain the following benefits.

Ability Score Increase. Increase your Strength or Dexterity score by 1, to a maximum of 20.

Punch and Grab. When you hit a creature with an Unarmed Strike as part of the Attack action on your turn, you can use both the Damage and the Grapple option. You can use this benefit only once per turn.

Attack Advantage. You have Advantage on attack rolls against a creature Grappled by you.

Fast Wrestler. You don’t have to spend extra movement to move a creature Grappled by you if the creature is your size or smaller.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:feats/general_feats_grappler ` |
| ` implementation ` | ` dnd2024:feats_general_feats_grappler ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` feat_category ` | ` general ` |

#### Prerequisites

<!-- splat:yaml path=splat_rule/parameters/prerequisites -->

```yaml
minimum_character_level: 4
any_ability_score_minimum:
  abilities:
  - strength
  - dexterity
  minimum: 13
```

#### Ability score increase

<!-- splat:values path=splat_rule/parameters/ability_score_increase -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["strength", "dexterity"] ` |
| ` amount ` | ` 1 ` |
| ` maximum ` | ` 20 ` |

#### Punch and grab

<!-- splat:yaml path=splat_rule/parameters/punch_and_grab -->

```yaml
trigger:
  unarmed_strike_hit_creature: true
  part_of_attack_action: true
  turn: own
allow_both_options:
- damage
- grapple
limit:
  count: 1
  period: turn
```

#### Attack advantage

<!-- splat:values path=splat_rule/parameters/attack_advantage -->

| Field | Value |
| --- | --- |
| ` target_grappled_by ` | ` self ` |

#### Fast wrestler

<!-- splat:values path=splat_rule/parameters/fast_wrestler -->

| Field | Value |
| --- | --- |
| ` target_grappled_by ` | ` self ` |
| ` maximum_target_size ` | ` own_size ` |
| ` extra_movement_cost ` | ` 0 ` |
