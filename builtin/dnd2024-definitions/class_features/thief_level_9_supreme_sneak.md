---
splat_schema: 2
splat_id: b936ddb1-08bf-5e5c-bfda-8ca0af67f130
splat_kind: rule
splat_key: dnd2024:class_features/thief_level_9_supreme_sneak
splat_name: 'Level 9: Supreme Sneak'
splat_origin: dnd2024:reference/pages/064
splat_sources:
- SRD 5.2.1, pp. 64, https://www.dndbeyond.com/srd
---

# Level 9: Supreme Sneak

## Source text

<!-- SRD page 64 -->

You gain the following Cunning Strike option.

Stealth Attack (Cost: 1d6). If you have the Hide action’s Invisible condition, this attack doesn’t end that condition on you if you end the turn behind Three-Quarters Cover or Total Cover.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/thief_level_9_supreme_sneak ` |
| ` implementation ` | ` dnd2024:class_features_thief_level_9_supreme_sneak ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` extends ` | ` rogue.cunning_strike ` |
| ` option ` | ` stealth_attack ` |
| ` attack_ends_invisible ` | ` false ` |

#### Cost

<!-- splat:values path=splat_rule/parameters/cost -->

| Field | Value |
| --- | --- |
| ` resource ` | ` sneak_attack.d6_dice ` |
| ` amount ` | ` 1 ` |

#### Requires

<!-- splat:values path=splat_rule/parameters/requires -->

| Field | Value |
| --- | --- |
| ` invisible_source ` | ` hide_action ` |

#### Conditional retention

<!-- splat:values path=splat_rule/parameters/conditional_retention -->

| Field | Value |
| --- | --- |
| ` check ` | ` actor.turn.end ` |
| ` requires_cover_any ` | ` ["three_quarters", "total"] ` |
| ` on_requirement_failure ` | ` end_hide_invisible ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` rogue ` |
| ` minimum_level ` | ` 9 ` |
