---
splat_schema: 1
splat_id: 82e6f19c-c1c8-59a2-821c-3cfc697e68a7
splat_kind: rule
splat_key: dnd2024:class_features/life_domain_level_17_supreme_healing
splat_name: 'Level 17: Supreme Healing'
splat_origin:
  key: dnd2024:reference/pages/040
splat_sources:
- title: SRD 5.2.1
  locator: pp. 40
  url: https://www.dndbeyond.com/srd
---

# Level 17: Supreme Healing

## Source text

<!-- SRD page 40 -->

When you would normally roll one or more dice to restore Hit Points to a creature with a spell or Channel Divinity, don’t roll those dice for the healing; instead use the highest number possible for each die. For example, instead of restoring 2d6 Hit Points to a creature with a spell, you restore 12.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/life_domain_level_17_supreme_healing ` |
| ` implementation ` | ` dnd2024:class_features_life_domain_level_17_supreme_healing ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` healing.dice_pending ` |
| ` sources ` | ` ["actor.spell", "actor.channel_divinity"] ` |
| ` replace_each_die ` | ` maximum_face ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` cleric ` |
| ` minimum_level ` | ` 17 ` |
