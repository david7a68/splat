---
splat_schema: 1
splat_id: 6cf285ec-2c46-540a-b012-b95e9af50229
splat_kind: rule
splat_key: dnd2024:feats/epic_boon_feats_boon_of_fate
splat_name: Boon of Fate
splat_origin:
  key: dnd2024:reference/pages/088
splat_sources:
- title: SRD 5.2.1
  locator: pp. 88
  url: https://www.dndbeyond.com/srd
---

# Boon of Fate

## Source text

<!-- SRD page 88 -->

Epic Boon Feat (Prerequisite: Level 19+)\
You gain the following benefits.\
Ability Score Increase. Increase one ability score\
of your choice by 1, to a maximum of 30.\
Improve Fate. When you or another creature\
within 60 feet of you succeeds on or fails a D20\
Test, you can roll 2d4 and apply the total rolled as a\
bonus or penalty to the d20 roll. Once you use this\
benefit, you can’t use it again until you roll Initiative\
or finish a Short or Long Rest.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:feats/epic_boon_feats_boon_of_fate ` |
| ` implementation ` | ` dnd2024:feats_epic_boon_feats_boon_of_fate ` |

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

#### Improve fate

<!-- splat:values path=splat_rule/parameters/improve_fate -->

| Field | Value |
| --- | --- |
| ` trigger_any ` | ` ["creature.d20_test_succeeded", "creature.d20_test_failed"] ` |
| ` optional ` | ` true ` |
| ` modify_choice ` | ` ["add_to_d20", "subtract_from_d20"] ` |
| ` recompute_result ` | ` true ` |
| ` cost ` | ` 1 ` |

##### Target

<!-- splat:values path=splat_rule/parameters/improve_fate/target -->

| Field | Value |
| --- | --- |
| ` self_allowed ` | ` true ` |
| ` range_feet ` | ` 60 ` |

##### Roll

<!-- splat:values path=splat_rule/parameters/improve_fate/roll -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` sides ` | ` 4 ` |

##### Resource

<!-- splat:values path=splat_rule/parameters/improve_fate/resource -->

| Field | Value |
| --- | --- |
| ` id ` | ` boon.fate ` |
| ` maximum ` | ` 1 ` |

###### Recovery

<!-- splat:table
path: splat_rule/parameters/improve_fate/resource/recovery
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Event:
    path: event
    type: string
  Amount:
    path: amount
    type: symbol
-->

| Key | Event | Amount |
| --- | --- | --- |
| ` row_1 ` | ` actor.initiative.rolled ` | ` All ` |
| ` row_2 ` | ` rest.short_completed ` | ` All ` |
| ` row_3 ` | ` rest.long_completed ` | ` All ` |
