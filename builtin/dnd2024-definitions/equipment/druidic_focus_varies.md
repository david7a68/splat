---
splat_schema: 1
splat_id: 834bcd37-0663-53ce-aac3-a2718dacb30f
splat_kind: item
splat_key: dnd2024:equipment/druidic_focus_varies
splat_name: Druidic Focus (Varies)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
---

# Druidic Focus (Varies)

## Source text

<!-- SRD page 97 -->

A Druidic Focus takes one of the forms in the Dru-\
idic Focuses table and is carved, tied with ribbon, or\
painted to channel primal magic. A Druid or Ranger\
can use such an object as a Spellcasting Focus.\

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` adventuring_gear ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/druidic_focus_varies ` |
| ` implementation ` | ` dnd2024:equipment_druidic_focus_varies ` |

### Parameters

#### Spellcasting focus

<!-- splat:yaml path=splat_rule/parameters/spellcasting_focus -->

```yaml
eligible_classes:
- key: dnd2024:classes/druid
- key: dnd2024:classes/ranger
rule:
  key: dnd2024:glossary/spellcasting_focus
```

#### Variants

<!-- splat:table
path: splat_rule/parameters/variants
key: Variant
order:
- sprig_of_mistletoe
- wooden_staff
- yew_wand
columns:
  Variant:
    path: id
    type: symbol
  Weight:
    path: mass
    type: quantity
  Cost:
    path: price
    type: quantity
  Valid use states:
    path: valid_use_states
    type: yaml
  Also weapon:
    path: also_weapon
    type: reference
    optional: true
-->

| Variant | Weight | Cost | Valid use states | Also weapon |
| --- | --- | --- | --- | --- |
| ` Sprig of mistletoe ` | ` 0 lb ` | ` 1 GP ` | ` ["held"] ` |  |
| ` Wooden staff ` | ` 4 lb ` | ` 5 GP ` | ` ["held"] ` | [Quarterstaff](<quarterstaff.md> "dnd2024:equipment/quarterstaff") |
| ` Yew wand ` | ` 1 lb ` | ` 10 GP ` | ` ["held"] ` |  |
