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

<!-- splat:rows path=splat_rule/parameters/variants -->

| Row | id | purchase_quantity |
| --- | --- | --- |
| ` 0 ` | ` arrows ` | ` 20 ` |
| ` 1 ` | ` bolts ` | ` 20 ` |
| ` 2 ` | ` firearm_bullets ` | ` 10 ` |
| ` 3 ` | ` sling_bullets ` | ` 20 ` |
| ` 4 ` | ` needles ` | ` 50 ` |

##### Storage

<!-- splat:values path=splat_rule/parameters/variants/0/storage -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/quiver_1_gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/0/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/0/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` gp ` |

##### Storage

<!-- splat:values path=splat_rule/parameters/variants/1/storage -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/case_crossbow_bolt_1_gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/1/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.5 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/1/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` gp ` |

##### Storage

<!-- splat:values path=splat_rule/parameters/variants/2/storage -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/pouch_5_sp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/2/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/2/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 3 ` |
| ` unit ` | ` gp ` |

##### Storage

<!-- splat:values path=splat_rule/parameters/variants/3/storage -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/pouch_5_sp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/3/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.5 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/3/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 4 ` |
| ` unit ` | ` cp ` |

##### Storage

<!-- splat:values path=splat_rule/parameters/variants/4/storage -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/pouch_5_sp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/4/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/4/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` gp ` |
