---
splat_schema: 1
splat_id: 07d470ab-d68e-5abe-b224-d36b2ba35380
splat_kind: item
splat_key: dnd2024:equipment/healer_s_kit_5_gp
splat_name: Healer’s Kit (5 GP)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
---

# Healer’s Kit (5 GP)

## Source text

<!-- SRD page 97 -->

A Healer’s Kit has ten uses. As a Utilize action, you\
can expend one of its uses to stabilize an Uncon-\
scious creature that has 0 Hit Points without need-\
ing to make a Wisdom (Medicine) check.\

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
| ` value ` | ` 5 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 3.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/healer_s_kit_5_gp ` |
| ` implementation ` | ` dnd2024:equipment_healer_s_kit_5_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` uses ` | ` 10 ` |

#### Use

<!-- splat:yaml path=splat_rule/parameters/use -->

```yaml
action: utilize
expend_uses: 1
target:
  condition: unconscious
  hit_points: 0
effect: stabilize
medicine_check_required: false
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 3.0 ` |
| ` unit ` | ` lb ` |
