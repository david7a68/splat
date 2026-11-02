---
splat_schema: 1
splat_id: dd9abbac-6dce-5d98-a8a6-66111abe138a
splat_kind: rule
splat_key: dnd2024:rules/armor_armor_training
splat_name: Armor Training
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
---

# Armor Training

## Source text

<!-- SRD page 92 -->

Anyone can don armor or hold a Shield, but only those with training can use them effectively, as explained below. A character’s class and other features determine the character’s armor training. A monster has training with any armor in its stat block.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/armor_armor_training ` |
| ` implementation ` | ` dnd2024:rules_armor_armor_training ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` don_without_training ` | ` true ` |
| ` training_source ` | ` ["class", "features"] ` |
| ` monster_training ` | ` all_armor_in_stat_block ` |
