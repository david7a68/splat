---
splat_schema: 1
splat_id: 2f057967-273c-532a-9bac-bb9ac8602109
splat_kind: item
splat_key: dnd2024:items/wand_of_paralysis
splat_name: Wand of Paralysis
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_paralysis
splat_sources:
- title: SRD 5.2.1
  locator: pp. 251
  url: https://www.dndbeyond.com/srd
---

# Wand of Paralysis

## Source text

<!-- SRD page 251 -->

Wand, Rare (Requires Attunement by a Spellcaster)\
This wand has 7 charges. While holding it, you can\
take a Magic action to expend 1 charge to cause a\
thin blue ray to streak from the tip toward a crea-\
ture you can see within 60 feet of yourself. The\
target must succeed on a DC 15 Constitution saving\
throw or have the Paralyzed condition for 1 minute.\
At the end of each of the target’s turns, it repeats\
the save, ending the effect on itself on a success.\
Regaining Charges. The wand regains 1d6 +\
1 expended charges daily at dawn. If you expend\
the wand’s last charge, roll 1d20. On a 1, the wand\
crumbles into ashes and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/251|page 251]].

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
| ` attunement_requirement_text ` | ` a Spellcaster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/wand_of_paralysis ` |
| ` implementation ` | ` dnd2024:item_wand_of_paralysis ` |

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
target:
  kind: creature
  seen: true
  range_ft: 60
save:
  ability: constitution
  dc: 15
failure:
  condition: paralyzed
  duration:
    value: 1
    unit: minute
repeat_save: target_turn_end
success: end_condition
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

<!-- splat:yaml path=splat_rule/parameters/attunement -->

```yaml
required: true
eligibility:
  spellcaster: true
```
