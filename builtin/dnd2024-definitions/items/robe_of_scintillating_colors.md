---
splat_schema: 1
splat_id: bfb6e27f-62a6-5c0a-a5d3-6a8fee8c2009
splat_kind: item
splat_key: dnd2024:items/robe_of_scintillating_colors
splat_name: Robe of Scintillating Colors
splat_origin:
  key: dnd2024:reference/magic_item/robe_of_scintillating_colors
splat_sources:
- title: SRD 5.2.1
  locator: pp. 240
  url: https://www.dndbeyond.com/srd
---

# Robe of Scintillating Colors

## Source text

<!-- SRD page 240 -->

Wondrous Item, Very Rare (Requires Attunement)

This robe has 3 charges, and it regains 1d3 expended charges daily at dawn. While you wear it, you can take a Magic action and expend 1 charge to cause the garment to display a shifting pattern of dazzling hues until the end of your next turn. During this time, the robe sheds Bright Light in a 30-foot radius and Dim Light for an additional 30 feet, and creatures that can see you have Disadvantage on attack rolls against you. Any creature in the Bright Light that can see you when the robe’s power is activated must succeed on a DC 15 Wisdom saving throw or have the Stunned condition until the effect ends.

Original layout: [[../../srd-5.2.1/pages/240|page 240]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/robe_of_scintillating_colors ` |
| ` implementation ` | ` dnd2024:item_robe_of_scintillating_colors ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 3
initial: 3
recovery:
  event: dawn
  amount:
    dice: 1
    sides: 3
    bonus: 0
```

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
action: magic
cost: 1
duration: until_user_next_turn_end
light:
  bright_radius_ft: 30
  additional_dim_ft: 30
attacks_against_user:
  disadvantage_if: attacker_can_see_user
activation_save:
  targets: creatures_in_bright_light_that_see_user
  ability: wisdom
  dc: 15
  failure:
    condition: stunned
    duration: until_effect_ends
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
