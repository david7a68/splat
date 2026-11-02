---
splat_schema: 1
splat_id: bf9388da-b669-57ab-badc-bd78eb90219c
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_they_re_not_hit_points_or_healing
splat_name: They’re Not Hit Points or Healing
splat_origin:
  key: dnd2024:reference/pages/018
splat_sources:
- title: SRD 5.2.1
  locator: pp. 18
  url: https://www.dndbeyond.com/srd
---

# They’re Not Hit Points or Healing

## Source text

<!-- SRD page 18 -->

Temporary Hit Points can’t be added to your Hit Points, healing can’t restore them, and receiving Temporary Hit Points doesn’t count as healing. Because Temporary Hit Points aren’t Hit Points, a creature can be at full Hit Points and receive Temporary Hit Points.

If you have 0 Hit Points, receiving Temporary Hit Points doesn’t restore you to consciousness. Only true healing can save you.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_they_re_not_hit_points_or_healing ` |
| ` implementation ` | ` dnd2024:rules_damage_and_healing_they_re_not_hit_points_or_healing ` |

### Parameters

#### Temporary hp

<!-- splat:values path=splat_rule/parameters/temporary_hp -->

| Field | Value |
| --- | --- |
| ` add_to_hp ` | ` false ` |
| ` restored_by_healing ` | ` false ` |
| ` counts_as_healing ` | ` false ` |
| ` allowed_at_full_hp ` | ` true ` |
| ` restores_consciousness_at_zero_hp ` | ` false ` |
