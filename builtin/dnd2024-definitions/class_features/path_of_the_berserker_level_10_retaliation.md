---
splat_schema: 2
splat_id: 764f6967-da91-5443-adc4-a7824accc3ff
splat_kind: rule
splat_key: dnd2024:class_features/path_of_the_berserker_level_10_retaliation
splat_name: 'Level 10: Retaliation'
splat_origin: dnd2024:reference/pages/030
splat_sources:
- SRD 5.2.1, pp. 30, https://www.dndbeyond.com/srd
---

# Level 10: Retaliation

## Source text

<!-- SRD page 30 -->

When you take damage from a creature that is within 5 feet of you, you can take a Reaction to make one melee attack against that creature, using a weapon or an Unarmed Strike.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/path_of_the_berserker_level_10_retaliation ` |
| ` implementation ` | ` dnd2024:class_features_path_of_the_berserker_level_10_retaliation ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` reaction ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` event ` | ` damage.taken ` |
| ` source_type ` | ` creature ` |
| ` source_distance_max_feet ` | ` 5 ` |

#### Attack

<!-- splat:values path=splat_rule/parameters/attack -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` target ` | ` damage_source ` |
| ` range ` | ` melee ` |
| ` allowed ` | ` ["weapon", "unarmed_strike"] ` |
