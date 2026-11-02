---
splat_schema: 1
splat_id: 55664569-b7c6-5c29-a6be-a0f8a1be023f
splat_kind: item
splat_key: dnd2024:equipment/ball_bearings_1_gp
splat_name: Ball Bearings (1 GP)
splat_origin:
  key: dnd2024:reference/pages/096
splat_sources:
- title: SRD 5.2.1
  locator: pp. 96
  url: https://www.dndbeyond.com/srd
---

# Ball Bearings (1 GP)

## Source text

<!-- SRD page 96 -->

As a Utilize action, you can spill Ball Bearings from their pouch. They spread to cover a level, 10-foot-square area within 10 feet of yourself. A creature that enters this area for the first time on a turn must succeed on a DC 10 Dexterity saving throw or have the Prone condition. It takes 10 minutes to recover the Ball Bearings.

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
| ` mechanic ` | ` dnd2024:equipment/ball_bearings_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_ball_bearings_1_gp ` |

### Parameters

#### Deploy

<!-- splat:yaml path=splat_rule/parameters/deploy -->

```yaml
action: utilize
area:
  shape: square
  side_ft: 10
  range_ft: 10
  level_ground: true
trigger: creature_enters_first_time_on_turn
save:
  ability: dexterity
  dc: 10
  failure:
    condition: prone
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
