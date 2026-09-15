---
splat_schema: 1
splat_id: 0a4aac78-c87d-5f56-8366-6ab785b04fc5
splat_kind: item
splat_key: dnd2024:equipment/torch_1_cp
splat_name: Torch (1 CP)
splat_origin:
  key: dnd2024:reference/pages/100
splat_sources:
- title: SRD 5.2.1
  locator: pp. 100
  url: https://www.dndbeyond.com/srd
---

# Torch (1 CP)

## Source text

<!-- SRD page 100 -->

A Torch burns for 1 hour, casting Bright Light in\
a 20-foot radius and Dim Light for an additional\
20 feet. When you take the Attack action, you can\
attack with the Torch, using it as a Simple Melee\
weapon. On a hit, the target takes 1 Fire damage.\

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
| ` unit ` | ` cp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/torch_1_cp ` |
| ` implementation ` | ` dnd2024:equipment_torch_1_cp ` |

### Parameters

#### Light

<!-- splat:values path=splat_rule/parameters/light -->

| Field | Value |
| --- | --- |
| ` requires_lit ` | ` true ` |
| ` shape ` | ` radius ` |
| ` bright_ft ` | ` 20 ` |
| ` additional_dim_ft ` | ` 20 ` |

#### Burn duration

<!-- splat:values path=splat_rule/parameters/burn_duration -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` hour ` |

#### Attack

<!-- splat:yaml path=splat_rule/parameters/attack -->

```yaml
action: attack
weapon_category: simple_melee
damage:
  amount: 1
  type: fire
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.0 ` |
| ` unit ` | ` lb ` |
