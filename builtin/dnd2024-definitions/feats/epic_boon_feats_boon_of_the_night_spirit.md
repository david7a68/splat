---
splat_schema: 1
splat_id: 1fa8b1b5-a6b8-58bc-976d-d30efb2a87f0
splat_kind: rule
splat_key: dnd2024:feats/epic_boon_feats_boon_of_the_night_spirit
splat_name: Boon of the Night Spirit
splat_origin:
  key: dnd2024:reference/pages/088
splat_sources:
- title: SRD 5.2.1
  locator: pp. 88
  url: https://www.dndbeyond.com/srd
---

# Boon of the Night Spirit

## Source text

<!-- SRD page 88 -->

Epic Boon Feat (Prerequisite: Level 19+)

You gain the following benefits.

Ability Score Increase. Increase one ability score of your choice by 1, to a maximum of 30.

Merge with Shadows. While within Dim Light or Darkness, you can give yourself the Invisible condition as a Bonus Action. The condition ends on you immediately after you take an action, a Bonus Action, or a Reaction.

Shadowy Form. While within Dim Light or Darkness, you have Resistance to all damage except Psychic and Radiant.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:feats/epic_boon_feats_boon_of_the_night_spirit ` |
| ` implementation ` | ` dnd2024:feats_epic_boon_feats_boon_of_the_night_spirit ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` feat_category ` | ` epic_boon ` |
| ` minimum_character_level ` | ` 19 ` |

#### Ability score increase

<!-- splat:values path=splat_rule/parameters/ability_score_increase -->

| Field | Value |
| --- | --- |
| ` choice ` | ` any_ability ` |
| ` amount ` | ` 1 ` |
| ` maximum ` | ` 30 ` |

#### Merge with shadows

<!-- splat:values path=splat_rule/parameters/merge_with_shadows -->

| Field | Value |
| --- | --- |
| ` requires_light ` | ` ["dim", "darkness"] ` |
| ` action ` | ` bonus_action ` |
| ` condition ` | ` invisible ` |
| ` ends_immediately_after_subsequent ` | ` ["action", "bonus_action", "reaction"] ` |

#### Shadowy form

<!-- splat:yaml path=splat_rule/parameters/shadowy_form -->

```yaml
requires_light:
- dim
- darkness
damage_resistance:
  all_except:
  - psychic
  - radiant
```
