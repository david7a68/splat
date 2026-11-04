---
splat_schema: 2
splat_id: 017c8afb-915f-514b-a8b7-592487ac13aa
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles
splat_name: Mounts and Vehicles
splat_origin: dnd2024:reference/pages/100
splat_sources:
- SRD 5.2.1, pp. 100, https://www.dndbeyond.com/srd
---

# Mounts and Vehicles

## Source text

<!-- SRD page 100 -->

A mount can help you move more quickly through the wilderness, but its primary purpose is to carry gear that would otherwise slow you down. The Mounts and Other Animals table shows each animal’s carrying capacity. See “Monsters” for the animals’ stat blocks.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/mounts_and_vehicles ` |
| ` implementation ` | ` dnd2024:rules_mounts_and_vehicles ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` animal_statistics_source ` | ` monster_stat_block ` |
| ` carrying_capacity_source ` | ` mounts_and_other_animals_table ` |

#### Capacity table rule

<!-- splat:values path=splat_rule/parameters/capacity_table_rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/mounts_and_vehicles_saddles ` |
