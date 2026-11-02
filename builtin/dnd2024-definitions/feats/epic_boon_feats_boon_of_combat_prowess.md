---
splat_schema: 1
splat_id: ad50542a-870b-5aa1-ae1f-4092d8750fd7
splat_kind: rule
splat_key: dnd2024:feats/epic_boon_feats_boon_of_combat_prowess
splat_name: Boon of Combat Prowess
splat_origin:
  key: dnd2024:reference/pages/088
splat_sources:
- title: SRD 5.2.1
  locator: pp. 88
  url: https://www.dndbeyond.com/srd
---

# Boon of Combat Prowess

## Source text

<!-- SRD page 88 -->

Epic Boon Feat (Prerequisite: Level 19+)

You gain the following benefits.

Ability Score Increase. Increase one ability score of your choice by 1, to a maximum of 30.

Peerless Aim. When you miss with an attack roll, you can hit instead. Once you use this benefit, you can’t use it again until the start of your next turn.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:feats/epic_boon_feats_boon_of_combat_prowess ` |
| ` implementation ` | ` dnd2024:feats_epic_boon_feats_boon_of_combat_prowess ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` feat_category ` | ` epic_boon ` |

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` minimum_character_level ` | ` 19 ` |

#### Ability increase

<!-- splat:values path=splat_rule/parameters/ability_increase -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` amount ` | ` 1 ` |
| ` choices ` | ` all ` |
| ` maximum_score ` | ` 30 ` |

#### Peerless aim

<!-- splat:yaml path=splat_rule/parameters/peerless_aim -->

```yaml
trigger: actor.attack_missed
optional: true
replace_result: hit
resource:
  id: boon.combat_prowess
  maximum: 1
  recovery:
  - event: actor.turn.start
    amount: all
cost: 1
```
