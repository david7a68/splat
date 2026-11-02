---
splat_schema: 1
splat_id: 2c8c118c-af0e-5f3a-b75f-aab0d29ea5df
splat_kind: item
splat_key: dnd2024:items/wand_of_enemy_detection
splat_name: Wand of Enemy Detection
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_enemy_detection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 250
  url: https://www.dndbeyond.com/srd
---

# Wand of Enemy Detection

## Source text

<!-- SRD page 250 -->

Wand, Rare (Requires Attunement)

This wand has 7 charges. While holding it, you can take a Magic action to expend 1 charge. For 1 minute, you know the direction of the nearest creature Hostile to you within 60 feet, but not its distance from you. The wand can sense the presence of Hostile creatures that are Invisible, ethereal, disguised, or hidden, as well as those in plain sight. The effect ends if you stop holding the wand.

Regaining Charges. The wand regains 1d6 + 1 expended charges daily at dawn. If you expend the wand’s last charge, roll 1d20. On a 1, the wand crumbles into ashes and is destroyed.

Original layout: [[../../srd-5.2.1/pages/250|page 250]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wand ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/wand_of_enemy_detection ` |
| ` implementation ` | ` dnd2024:item_wand_of_enemy_detection ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` held ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 7
initial: 7
recovery:
  event: dawn
  amount:
    dice: 1
    sides: 6
    bonus: 1
```

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
action: magic
cost: 1
duration:
  value: 1
  unit: minute
end_on: stop_holding_wand
detection:
  select: nearest_hostile_creature
  range_ft: 60
  output:
  - direction
  distance_revealed: false
  detect_despite:
  - invisible
  - ethereal
  - disguised
  - hidden
```

#### Last charge

<!-- splat:yaml path=splat_rule/parameters/last_charge -->

```yaml
roll:
  dice: 1
  sides: 20
  bonus: 0
on_value: 1
effect: destroy_item
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
