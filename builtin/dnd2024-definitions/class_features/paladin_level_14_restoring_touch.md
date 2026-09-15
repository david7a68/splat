---
splat_schema: 1
splat_id: 0e1fd6f2-67cc-5333-85c7-7e56d13a5f46
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_14_restoring_touch
splat_name: 'Level 14: Restoring Touch'
splat_origin:
  key: dnd2024:reference/pages/055
splat_sources:
- title: SRD 5.2.1
  locator: pp. 55
  url: https://www.dndbeyond.com/srd
---

# Level 14: Restoring Touch

## Source text

<!-- SRD page 55 -->

When you use Lay On Hands on a creature, you can\
also remove one or more of the following conditions\
from the creature: Blinded, Charmed, Deafened,\
Frightened, Paralyzed, or Stunned. You must ex-\
pend 5 Hit Points from the healing pool of Lay On\
Hands for each of these conditions you remove;\
those points don’t also restore Hit Points to the\
creature.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/paladin_level_14_restoring_touch ` |
| ` implementation ` | ` dnd2024:class_features_paladin_level_14_restoring_touch ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` during ` | ` lay_on_hands ` |
| ` pool_cost_per_condition ` | ` 5 ` |
| ` cost_points_also_heal ` | ` false ` |

#### Remove conditions choice

<!-- splat:values path=splat_rule/parameters/remove_conditions_choice -->

| Field | Value |
| --- | --- |
| ` options ` | ` ["blinded", "charmed", "deafened", "frightened", "paralyzed", "stunned"] ` |
| ` maximum ` | ` all_present ` |
