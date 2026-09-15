---
splat_schema: 1
splat_id: 54abaa45-4ef6-5af9-a91a-0de7960bc28a
splat_kind: rule
splat_key: dnd2024:class_features/champion_level_3_remarkable_athlete
splat_name: 'Level 3: Remarkable Athlete'
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49
  url: https://www.dndbeyond.com/srd
---

# Level 3: Remarkable Athlete

## Source text

<!-- SRD page 49 -->

Thanks to your athleticism, you have Advantage on\
Initiative rolls and Strength (Athletics) checks.\
In addition, immediately after you score a Critical\
Hit, you can move up to half your Speed without\
provoking Opportunity Attacks.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/champion_level_3_remarkable_athlete ` |
| ` implementation ` | ` dnd2024:class_features_champion_level_3_remarkable_athlete ` |

### Parameters

#### Advantage on

<!-- splat:rows path=splat_rule/parameters/advantage_on -->

| Row | roll | ability | skill |
| --- | --- | --- | --- |
| ` 0 ` | ` initiative ` |  |  |
| ` 1 ` | ` ability_check ` | ` strength ` | ` athletics ` |

#### After critical hit

<!-- splat:yaml path=splat_rule/parameters/after_critical_hit -->

```yaml
optional_move_maximum_feet:
  op: divide
  args:
  - stat: speed
  - 2
provokes_opportunity_attacks: false
```
