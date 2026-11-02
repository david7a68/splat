---
splat_schema: 1
splat_id: 195d81fd-98a0-57e1-b2b3-134a814d0214
splat_kind: item
splat_key: dnd2024:equipment/ammunition_varies
splat_name: Ammunition (Varies)
splat_origin:
  key: dnd2024:reference/pages/094
splat_sources:
- title: SRD 5.2.1
  locator: pp. 94, 96
  url: https://www.dndbeyond.com/srd
---

# Ammunition (Varies)

## Source text

<!-- SRD page 94 -->

Ammunition is required by a weapon that has the Ammunition property. A weapon’s description specifies the type of ammunition used by the weapon. The Ammunition table lists the different types and the amount you get when you buy them. The table also lists the item that is typically used to store each type; storage must be bought separately.

<!-- SRD page 96 -->

| Type | Amount | Storage | Weight | Cost |
| --- | --- | --- | --- | --- |
| Arrows | 20 | Quiver | 1 lb. | 1 GP |
| Bolts | 20 | Case | 1½ lb. | 1 GP |
| Bullets, Firearm | 10 | Pouch | 2 lb. | 3 GP |
| Bullets, Sling | 20 | Pouch | 1½ lb. | 4 CP |
| Needles | 50 | Pouch | 1 lb. | 1 GP |

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
| ` mechanic ` | ` dnd2024:equipment/ammunition_varies ` |
| ` implementation ` | ` dnd2024:equipment_ammunition_varies ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` required_by_property ` | ` ammunition ` |
| ` weapon_selects_type ` | ` true ` |
| ` storage_included ` | ` false ` |

#### Variants

<!-- splat:table
path: splat_rule/parameters/variants
key: Variant
order:
- arrows
- bolts
- firearm_bullets
- sling_bullets
- needles
columns:
  Variant:
    path: id
    type: symbol
  Quantity:
    path: purchase_quantity
    type: integer
  Storage:
    path: storage
    type: reference
  Weight:
    path: mass
    type: quantity
  Cost:
    path: price
    type: quantity
-->

| Variant | Quantity | Storage | Weight | Cost |
| --- | --- | --- | --- | --- |
| ` Arrows ` | ` 20 ` | [Quiver (1 GP)](<quiver_1_gp.md> "dnd2024:equipment/quiver_1_gp") | ` 1 lb ` | ` 1 GP ` |
| ` Bolts ` | ` 20 ` | [Case, Crossbow Bolt (1 GP)](<case_crossbow_bolt_1_gp.md> "dnd2024:equipment/case_crossbow_bolt_1_gp") | ` 1½ lb ` | ` 1 GP ` |
| ` Firearm bullets ` | ` 10 ` | [Pouch (5 SP)](<pouch_5_sp.md> "dnd2024:equipment/pouch_5_sp") | ` 2 lb ` | ` 3 GP ` |
| ` Sling bullets ` | ` 20 ` | [Pouch (5 SP)](<pouch_5_sp.md> "dnd2024:equipment/pouch_5_sp") | ` 1½ lb ` | ` 4 CP ` |
| ` Needles ` | ` 50 ` | [Pouch (5 SP)](<pouch_5_sp.md> "dnd2024:equipment/pouch_5_sp") | ` 1 lb ` | ` 1 GP ` |
