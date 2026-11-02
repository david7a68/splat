---
splat_schema: 1
splat_id: dfb16e24-c70c-5390-93cd-d4b54d07b50a
splat_kind: rule
splat_key: dnd2024:rules/magic_items_ending_attunement
splat_name: Ending Attunement
splat_origin:
  key: dnd2024:reference/pages/102
splat_sources:
- title: SRD 5.2.1
  locator: pp. 102, 103
  url: https://www.dndbeyond.com/srd
---

# Ending Attunement

## Source text

<!-- SRD page 102 -->

Your Attunement to an item ends if you no longer satisfy the prerequisites for Attunement, if the item has been more than 100 feet away for at least 24 hours, if you die, or if another creature attunes to the item. You can also voluntarily end Attunement by spending another Short Rest focused on the item unless the item is cursed.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_items_ending_attunement ` |
| ` implementation ` | ` dnd2024:rules_magic_items_ending_attunement ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attunement_ends_on ` | ` ["prerequisite_lost", "bearer_death", "other_creature_attunes"] ` |

#### Separation

<!-- splat:yaml path=splat_rule/parameters/separation -->

```yaml
distance_feet_strictly_greater_than: 100
continuous_duration:
  value: 24
  unit: hours
```

#### Voluntary end

<!-- splat:values path=splat_rule/parameters/voluntary_end -->

| Field | Value |
| --- | --- |
| ` focused_rest ` | ` short ` |
| ` cursed_item_allowed ` | ` false ` |
