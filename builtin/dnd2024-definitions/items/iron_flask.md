---
splat_schema: 1
splat_id: 8ea683ac-bc3b-50bb-9d17-b1c4a8f8c7c0
splat_kind: item
splat_key: dnd2024:items/iron_flask
splat_name: Iron Flask
splat_origin:
  key: dnd2024:reference/magic_item/iron_flask
splat_sources:
- title: SRD 5.2.1
  locator: pp. 228
  url: https://www.dndbeyond.com/srd
---

# Iron Flask

## Source text

<!-- SRD page 228 -->

Wondrous Item, Legendary

While holding this brass-stoppered iron flask, you can take a Magic action to target a creature that you can see within 60 feet of yourself. If the flask is empty and the target is native to a plane of existence other than the one you’re on, the target must succeed on a DC 17 Wisdom saving throw or be trapped in the flask. If the target has been trapped by the flask before, it has Advantage on the save. Once trapped, a creature remains in the flask until released. The flask can hold only one creature at a time. A creature trapped in the flask doesn’t age and doesn’t need to breathe, eat, or drink.

You can take a Magic action to remove the flask’s stopper and release the creature in the flask. The creature then obeys your commands for 1 hour, understanding those commands even if it doesn’t know the language in which the commands are given. If you issue no commands or give the creature a command that is likely to result in its death or imprisonment, it defends itself but otherwise takes no actions. At the end of the duration, the creature acts in accordance with its normal disposition and alignment. An Identify spell reveals if the flask contains a creature, but the only way to determine the type of creature is to open the flask. A newly discovered Iron Flask might already contain a creature chosen by the GM.

Original layout: [[../../srd-5.2.1/pages/228|page 228]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` legendary ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/iron_flask ` |
| ` implementation ` | ` dnd2024:item_iron_flask ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` held ` |
| ` capacity_creatures ` | ` 1 ` |
| ` determine_type ` | ` open_flask ` |

#### Capture

<!-- splat:yaml path=splat_rule/parameters/capture -->

```yaml
action: magic
range_ft: 60
must_see: true
requires_flask_empty: true
target_native_plane_must_differ_from_current: true
save:
  ability: wisdom
  dc: 17
  failure:
    trapped_in_flask: true
  success:
    effect: none
save_advantage_if: previously_trapped_by_same_flask
```

#### Trapped

<!-- splat:values path=splat_rule/parameters/trapped -->

| Field | Value |
| --- | --- |
| ` duration ` | ` until_released ` |
| ` age ` | ` false ` |
| ` needs ` | ` ["no_breathing", "no_food", "no_drink"] ` |

#### Release

<!-- splat:yaml path=splat_rule/parameters/release -->

```yaml
action: magic
remove_stopper: true
obeys_user_commands_duration:
  amount: 1
  unit: hour
understands_commands_any_language: true
no_command_or_likely_death_or_imprisonment:
  self_defense: true
  other_actions: false
after_duration: normal_disposition_and_alignment
```

#### Identify

<!-- splat:values path=splat_rule/parameters/identify -->

| Field | Value |
| --- | --- |
| ` reveals_occupied ` | ` true ` |
| ` reveals_creature_type ` | ` false ` |

#### Initial occupant

<!-- splat:values path=splat_rule/parameters/initial_occupant -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` gm ` |
| ` optional ` | ` true ` |
| ` type ` | ` definition ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
