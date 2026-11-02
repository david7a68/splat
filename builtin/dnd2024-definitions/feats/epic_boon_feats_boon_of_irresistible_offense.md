---
splat_schema: 1
splat_id: ca7e9da1-ce27-5267-8c28-e409ea166f46
splat_kind: rule
splat_key: dnd2024:feats/epic_boon_feats_boon_of_irresistible_offense
splat_name: Boon of Irresistible Offense
splat_origin:
  key: dnd2024:reference/pages/088
splat_sources:
- title: SRD 5.2.1
  locator: pp. 88
  url: https://www.dndbeyond.com/srd
---

# Boon of Irresistible Offense

## Source text

<!-- SRD page 88 -->

Epic Boon Feat (Prerequisite: Level 19+)

You gain the following benefits.

Ability Score Increase. Increase your Strength or Dexterity score by 1, to a maximum of 30.

Overcome Defenses. The Bludgeoning, Piercing, and Slashing damage you deal always ignores Resistance.

Overwhelming Strike. When you roll a 20 on the d20 for an attack roll, you can deal extra damage to the target equal to the ability score increased by this feat. The extra damage’s type is the same as the attack’s type.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:feats/epic_boon_feats_boon_of_irresistible_offense ` |
| ` implementation ` | ` dnd2024:feats_epic_boon_feats_boon_of_irresistible_offense ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` feat_category ` | ` epic_boon ` |
| ` minimum_character_level ` | ` 19 ` |
| ` ignore_damage_resistance ` | ` ["bludgeoning", "piercing", "slashing"] ` |

#### Ability score increase

<!-- splat:values path=splat_rule/parameters/ability_score_increase -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["strength", "dexterity"] ` |
| ` amount ` | ` 1 ` |
| ` maximum ` | ` 30 ` |

#### Overwhelming strike

<!-- splat:yaml path=splat_rule/parameters/overwhelming_strike -->

```yaml
trigger:
  attack_d20_natural_result: 20
optional_extra_damage:
  amount:
    stat: ability_chosen_for_this_feat.score
  type: triggering_attack_damage_type
```
