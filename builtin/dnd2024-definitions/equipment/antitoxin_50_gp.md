---
splat_schema: 1
splat_id: 647722ff-0ca2-5b87-958c-c34ce116c765
splat_kind: item
splat_key: dnd2024:equipment/antitoxin_50_gp
splat_name: Antitoxin (50 GP)
splat_origin:
  key: dnd2024:reference/pages/096
splat_sources:
- title: SRD 5.2.1
  locator: pp. 96
  url: https://www.dndbeyond.com/srd
---

# Antitoxin (50 GP)

## Source text

<!-- SRD page 96 -->

As a Bonus Action, you can drink a vial of Antitoxin\
to gain Advantage on saving throws to avoid or end\
the Poisoned condition for 1 hour.\

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
| ` value ` | ` 50 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/antitoxin_50_gp ` |
| ` implementation ` | ` dnd2024:equipment_antitoxin_50_gp ` |

### Parameters

#### Use

<!-- splat:yaml path=splat_rule/parameters/use -->

```yaml
economy: bonus_action
consume: 1
target: self
operation: drink
effect:
  roll: saving_throw
  mode: advantage
  scope: avoid_or_end_poisoned
  duration:
    value: 1
    unit: hour
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0.0 ` |
| ` unit ` | ` lb ` |
| ` negligible ` | ` true ` |
