---
splat_schema: 1
splat_id: 75adbfde-5148-5cb1-a47a-f8df1383980d
splat_kind: item
splat_key: dnd2024:equipment/spikes_iron_1_gp
splat_name: Spikes, Iron (1 GP)
splat_origin:
  key: dnd2024:reference/pages/100
splat_sources:
- title: SRD 5.2.1
  locator: pp. 100
  url: https://www.dndbeyond.com/srd
---

# Spikes, Iron (1 GP)

## Source text

<!-- SRD page 100 -->

Iron Spikes come in bundles of ten. As a Utilize\
action, you can use a blunt object, such as a Light\
Hammer, to hammer a spike into wood, earth, or a\
similar material. You can do so to jam a door shut or\
to then tie a Rope or Chain to the Spike.\

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

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/spikes_iron_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_spikes_iron_1_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` purchase_quantity ` | ` 10 ` |

#### Hammer

<!-- splat:values path=splat_rule/parameters/hammer -->

| Field | Value |
| --- | --- |
| ` action ` | ` utilize ` |
| ` requires ` | ` blunt_object ` |
| ` materials ` | ` ["wood", "earth", "similar_material"] ` |
| ` similar_material_authority ` | ` dm ` |
| ` operations ` | ` ["jam_door_shut", "provide_rope_anchor", "provide_chain_anchor"] ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5.0 ` |
| ` unit ` | ` lb ` |
