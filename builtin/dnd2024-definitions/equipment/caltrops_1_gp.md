---
splat_schema: 1
splat_id: a4cb4197-cac1-5c91-874d-53516761b2f1
splat_kind: item
splat_key: dnd2024:equipment/caltrops_1_gp
splat_name: Caltrops (1 GP)
splat_origin:
  key: dnd2024:reference/pages/096
splat_sources:
- title: SRD 5.2.1
  locator: pp. 96
  url: https://www.dndbeyond.com/srd
---

# Caltrops (1 GP)

## Source text

<!-- SRD page 96 -->

As a Utilize action, you can spread Caltrops from\
their bag to cover a 5-foot-square area within 5 feet\
of yourself. A creature that enters this area for the\
first time on a turn must succeed on a DC 15 Dex-\
terity saving throw or take 1 Piercing damage and\
have its Speed reduced to 0 until the start of its next\
turn. It takes 10 minutes to recover the Caltrops.\

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
| ` value ` | ` 2.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/caltrops_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_caltrops_1_gp ` |

### Parameters

#### Deploy

<!-- splat:yaml path=splat_rule/parameters/deploy -->

```yaml
action: utilize
area:
  shape: square
  side_ft: 5
  range_ft: 5
trigger: creature_enters_first_time_on_turn
save:
  ability: dexterity
  dc: 15
  failure:
    damage:
      amount: 1
      type: piercing
    speed_set: 0
    speed_expiry: target_next_turn_start
  success: no_effect
```

#### Recover

<!-- splat:yaml path=splat_rule/parameters/recover -->

```yaml
duration:
  value: 10
  unit: minute
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2.0 ` |
| ` unit ` | ` lb ` |
