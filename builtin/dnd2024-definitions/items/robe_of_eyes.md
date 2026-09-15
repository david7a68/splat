---
splat_schema: 1
splat_id: 1c8ebf98-c2c6-5e3d-acaf-be0f0be519a4
splat_kind: item
splat_key: dnd2024:items/robe_of_eyes
splat_name: Robe of Eyes
splat_origin:
  key: dnd2024:reference/magic_item/robe_of_eyes
splat_sources:
- title: SRD 5.2.1
  locator: pp. 240
  url: https://www.dndbeyond.com/srd
---

# Robe of Eyes

## Source text

<!-- SRD page 240 -->

Wondrous Item, Rare (Requires Attunement)\
This robe is adorned with eyelike patterns. While\
you wear the robe, you gain the following benefits:\
All-Around Vision. The robe gives you Advantage\
on Wisdom (Perception) checks that rely on sight.\
Special Senses. You have Darkvision and Truesight,\
both with a range of 120 feet.\
Drawbacks. A Light spell cast on the robe or a\
Daylight spell cast within 5 feet of the robe gives\
you the Blinded condition for 1 minute. At the end of\
each of your turns, you make a Constitution saving\
throw (DC 11 for Light or DC 15 for Daylight), ending\
the condition on yourself on a success.\

Original layout: [[../../srd-5.2.1/pages/240|page 240]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
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
| ` mechanic ` | ` dnd2024:items/robe_of_eyes ` |
| ` implementation ` | ` dnd2024:item_robe_of_eyes ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_required ` | ` true ` |

#### Checks

<!-- splat:rows path=splat_rule/parameters/checks -->

| Row | ability | skill | requires | mode |
| --- | --- | --- | --- | --- |
| ` 0 ` | ` wisdom ` | ` perception ` | ` sight ` | ` advantage ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision_ft ` | ` 120 ` |
| ` truesight_ft ` | ` 120 ` |

#### Drawbacks

<!-- splat:yaml path=splat_rule/parameters/drawbacks -->

```yaml
- trigger:
    spell:
      key: dnd2024:spells/light
    target: robe
  effect:
    condition: blinded
    duration:
      amount: 1
      unit: minutes
  repeat_save:
    ability: constitution
    dc: 11
    failure:
      continue: true
    success:
      end_effect: true
    repeat: target.turn_end
- trigger:
    spell:
      key: dnd2024:spells/daylight
    distance_from_robe_ft_max: 5
  effect:
    condition: blinded
    duration:
      amount: 1
      unit: minutes
  repeat_save:
    ability: constitution
    dc: 15
    failure:
      continue: true
    success:
      end_effect: true
    repeat: target.turn_end
```
