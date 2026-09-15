---
splat_schema: 1
splat_id: e4a7c15f-aadc-550c-bb18-c86442166399
splat_kind: item
splat_key: dnd2024:items/dragon_scale_mail
splat_name: Dragon Scale Mail
splat_origin:
  key: dnd2024:reference/magic_item/dragon_scale_mail
splat_sources:
- title: SRD 5.2.1
  locator: pp. 218, 219
  url: https://www.dndbeyond.com/srd
---

# Dragon Scale Mail

## Source text

<!-- SRD page 218 -->

Armor (Scale Mail), Very Rare (Requires Attunement)\
Dragon Scale Mail is made of the scales of one kind\
of dragon. Sometimes dragons collect their cast-off\

<!-- SRD page 219 -->

scales and gift them. Other times, hunters carefully\
preserve the hide of a dead dragon. In either case,\
Dragon Scale Mail is highly valued.\
While wearing this armor, you gain a +1 bonus to\
Armor Class, you have Advantage on saving throws\
against the breath weapons of Dragons, and you\
have Resistance to one damage type determined by\
the kind of dragon that provided the scales (see the\
accompanying table).\
Additionally, you can focus your senses as a Magic\
action to discern the distance and direction to the\
closest dragon within 30 miles of yourself that is\
of the same type as the armor. This action can’t be\
used again until the next dawn.\
Dragon Resistance Dragon Resistance\
Black Acid Gold Fire\
Blue Lightning Green Poison\
Brass Fire Red Fire\
Bronze Lightning Silver Cold\
Copper Acid White Cold\

Original layout: [[../../srd-5.2.1/pages/218|page 218]], [[../../srd-5.2.1/pages/219|page 219]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` armor ` |
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
| ` mechanic ` | ` dnd2024:items/dragon_scale_mail ` |
| ` implementation ` | ` dnd2024:item_dragon_scale_mail ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` armor_class_bonus ` | ` 1 ` |

#### Saving throws

<!-- splat:values path=splat_rule/parameters/saving_throws -->

| Field | Value |
| --- | --- |
| ` advantage_against ` | ` dragon_breath_weapons ` |

#### Variants

<!-- splat:yaml path=splat_rule/parameters/variants -->

```yaml
black:
  resistance: acid
blue:
  resistance: lightning
brass:
  resistance: fire
bronze:
  resistance: lightning
copper:
  resistance: acid
gold:
  resistance: fire
green:
  resistance: poison
red:
  resistance: fire
silver:
  resistance: cold
white:
  resistance: cold
```

#### Detection

<!-- splat:yaml path=splat_rule/parameters/detection -->

```yaml
action: magic
resource:
  uses: 1
  recovery: dawn
target: closest_dragon_matching_armor_type
range_miles: 30
outputs:
- distance
- direction
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
