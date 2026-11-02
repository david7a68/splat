---
splat_schema: 1
splat_id: bd48fcf6-9d0b-52e1-8969-a7d7c0b411b0
splat_kind: rule
splat_key: dnd2024:class_features/thief_level_3_fast_hands
splat_name: 'Level 3: Fast Hands'
splat_origin:
  key: dnd2024:reference/pages/064
splat_sources:
- title: SRD 5.2.1
  locator: pp. 64
  url: https://www.dndbeyond.com/srd
---

# Level 3: Fast Hands

## Source text

<!-- SRD page 64 -->

As a Bonus Action, you can do one of the following.

Sleight of Hand. Make a Dexterity (Sleight of Hand) check to pick a lock or disarm a trap with Thieves’ Tools or to pick a pocket.

Use an Object. Take the Utilize action, or take the Magic action to use a magic item that requires that action.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/thief_level_3_fast_hands ` |
| ` implementation ` | ` dnd2024:class_features_thief_level_3_fast_hands ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |
| ` choose ` | ` 1 ` |

#### Options

<!-- splat:yaml path=splat_rule/parameters/options -->

```yaml
sleight_of_hand:
  ability: dexterity
  skill: sleight_of_hand
  tasks:
    pick_lock:
      tool: thieves_tools
    disarm_trap:
      tool: thieves_tools
    pick_pocket: {}
utilize:
  take_action: utilize
magic_item:
  take_action: magic
  scope: use_magic_item
  item_requires_action: magic
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` rogue ` |
| ` minimum_level ` | ` 3 ` |
