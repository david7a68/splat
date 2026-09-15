---
splat_schema: 1
splat_id: 92d39316-b0ec-5655-a7c1-c5df64ea7786
splat_kind: rule
splat_key: dnd2024:rules/combat_cover
splat_name: Cover
splat_origin:
  key: dnd2024:reference/pages/015
splat_sources:
- title: SRD 5.2.1
  locator: pp. 15
  url: https://www.dndbeyond.com/srd
---

# Cover

## Source text

<!-- SRD page 15 -->

Walls, trees, creatures, and other obstacles can pro-\
vide cover, making a target more difficult to harm.\
As detailed in the Cover table, there are three de-\
grees of cover, each of which gives a different bene-\
fit to a target.\
A target can benefit from cover only when an at-\
tack or other effect originates on the opposite side\
of the cover. If a target is behind multiple sources\
of cover, only the most protective degree of cover\
applies; the degrees aren’t added together. For ex-\
ample, if a target is behind a creature that gives Half\
Cover and a tree trunk that gives Three-Quarters\
Cover, the target has Three-Quarters Cover.\
Cover\
Degree Benefit to Target Offered By …\
Half +2 bonus to AC Another creature or\
and Dexterity an object that covers at\
saving throws least half of the target\
Three- +5 bonus to AC An object that covers at\
Quarters and Dexterity least three-quarters of\
saving throws the target\
Total Can’t be targeted An object that covers\
directly the whole target\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_cover ` |
| ` implementation ` | ` dnd2024:rule_combat_cover ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` requires ` | ` effect_origin_opposite_side_of_cover ` |
| ` stacking ` | ` highest_degree_only ` |

#### Degrees

<!-- splat:rows path=splat_rule/parameters/degrees -->

| Row | degree | minimum_covered_fraction | ac_bonus | dexterity_save_bonus | creature_can_supply | direct_targeting |
| --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` half ` | ` 0.5 ` | ` 2 ` | ` 2 ` | ` true ` |  |
| ` 1 ` | ` three_quarters ` | ` 0.75 ` | ` 5 ` | ` 5 ` |  |  |
| ` 2 ` | ` total ` | ` 1 ` |  |  |  | ` false ` |
