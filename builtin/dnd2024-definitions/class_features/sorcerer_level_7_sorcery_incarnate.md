---
splat_schema: 1
splat_id: 207aa813-5898-5064-9c61-a2b394a45820
splat_kind: rule
splat_key: dnd2024:class_features/sorcerer_level_7_sorcery_incarnate
splat_name: 'Level 7: Sorcery Incarnate'
splat_origin:
  key: dnd2024:reference/pages/066
splat_sources:
- title: SRD 5.2.1
  locator: pp. 66
  url: https://www.dndbeyond.com/srd
---

# Level 7: Sorcery Incarnate

## Source text

<!-- SRD page 66 -->

If you have no uses of Innate Sorcery left, you can use it if you spend 2 Sorcery Points when you take the Bonus Action to activate it.

In addition, while your Innate Sorcery feature is active, you can use up to two of your Metamagic options on each spell you cast.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/sorcerer_level_7_sorcery_incarnate ` |
| ` implementation ` | ` dnd2024:class_features_sorcerer_level_7_sorcery_incarnate ` |

### Parameters

#### Innate sorcery alternative cost

<!-- splat:values path=splat_rule/parameters/innate_sorcery_alternative_cost -->

| Field | Value |
| --- | --- |
| ` requires_remaining_uses ` | ` 0 ` |
| ` sorcery_points ` | ` 2 ` |
| ` action ` | ` same_activation_bonus_action ` |

#### While innate sorcery active

<!-- splat:values path=splat_rule/parameters/while_innate_sorcery_active -->

| Field | Value |
| --- | --- |
| ` maximum_metamagic_options_per_cast ` | ` 2 ` |
