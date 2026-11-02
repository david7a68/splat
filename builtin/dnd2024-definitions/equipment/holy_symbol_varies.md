---
splat_schema: 1
splat_id: 210679f4-23a6-5183-80e9-87002c3038d3
splat_kind: item
splat_key: dnd2024:equipment/holy_symbol_varies
splat_name: Holy Symbol (Varies)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
---

# Holy Symbol (Varies)

## Source text

<!-- SRD page 97 -->

A Holy Symbol takes one of the forms in the Holy Symbol table and is bejeweled or painted to channel divine magic. A Cleric or Paladin can use a Holy Symbol as a Spellcasting Focus.

The table indicates whether a Holy Symbol needs to be held, worn, or borne on fabric (such as a tabard or banner) or a Shield.

**Holy Symbols**

| Symbol | Weight | Cost |
| --- | --- | --- |
| Amulet (worn or held) | 1 lb. | 5 GP |
| Emblem (borne on fabric or a Shield) | — | 5 GP |
| Reliquary (held) | 2 lb. | 5 GP |

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
| ` mechanic ` | ` dnd2024:equipment/holy_symbol_varies ` |
| ` implementation ` | ` dnd2024:equipment_holy_symbol_varies ` |

### Parameters

#### Spellcasting focus

<!-- splat:yaml path=splat_rule/parameters/spellcasting_focus -->

```yaml
eligible_classes:
- key: dnd2024:classes/cleric
- key: dnd2024:classes/paladin
rule:
  key: dnd2024:glossary/spellcasting_focus
```

#### Variants

<!-- splat:table
path: splat_rule/parameters/variants
key: Variant
order:
- amulet
- emblem
- reliquary
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
-->

| Variant | Weight | Cost | Valid use states |
| --- | --- | --- | --- |
| ` Amulet ` | ` 1 lb ` | ` 5 GP ` | ` ["worn", "held"] ` |
| ` Emblem ` | ` 0 lb ` | ` 5 GP ` | ` ["borne_on_fabric", "borne_on_shield"] ` |
| ` Reliquary ` | ` 2 lb ` | ` 5 GP ` | ` ["held"] ` |
