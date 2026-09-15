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
Druidic Focuses\
Focus Weight Cost\
Sprig of mistletoe — 1 GP\
Wooden staff (also a Quarterstaff) 4 lb. 5 GP\
Yew wand 1 lb. 10 GP\

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

<!-- splat:rows path=splat_rule/parameters/variants -->

| Row | id | valid_use_states |
| --- | --- | --- |
| ` 0 ` | ` sprig_of_mistletoe ` | ` ["held"] ` |
| ` 1 ` | ` wooden_staff ` | ` ["held"] ` |
| ` 2 ` | ` yew_wand ` | ` ["held"] ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/0/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/0/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/1/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 4 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/1/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5 ` |
| ` unit ` | ` gp ` |

##### Also weapon

<!-- splat:values path=splat_rule/parameters/variants/1/also_weapon -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/quarterstaff ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/2/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/2/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 10 ` |
| ` unit ` | ` gp ` |
