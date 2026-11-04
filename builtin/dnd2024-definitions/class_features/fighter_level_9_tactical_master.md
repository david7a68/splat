---
splat_schema: 2
splat_id: 3e9d4faf-6804-5e6d-95cb-833bd7ede923
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_9_tactical_master
splat_name: 'Level 9: Tactical Master'
splat_origin: dnd2024:reference/pages/048
splat_sources:
- SRD 5.2.1, pp. 48, https://www.dndbeyond.com/srd
---

# Level 9: Tactical Master

## Source text

<!-- SRD page 48 -->

When you attack with a weapon whose mastery property you can use, you can replace that property with the Push, Sap, or Slow property for that attack.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/fighter_level_9_tactical_master ` |
| ` implementation ` | ` dnd2024:class_features_fighter_level_9_tactical_master ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` weapon.attack ` |

#### Requires

<!-- splat:values path=splat_rule/parameters/requires -->

| Field | Value |
| --- | --- |
| ` weapon_mastery_available_to_actor ` | ` true ` |

#### Replace mastery for this attack

<!-- splat:values path=splat_rule/parameters/replace_mastery_for_this_attack -->

| Field | Value |
| --- | --- |
| ` optional ` | ` true ` |
| ` choices ` | ` ["push", "sap", "slow"] ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` fighter ` |
| ` minimum_level ` | ` 9 ` |
