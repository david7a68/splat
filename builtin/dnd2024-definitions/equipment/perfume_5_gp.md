---
splat_schema: 1
splat_id: 28e2a10a-5f6e-510d-a6d4-e7b3287c5c95
splat_kind: item
splat_key: dnd2024:equipment/perfume_5_gp
splat_name: Perfume (5 GP)
splat_origin:
  key: dnd2024:reference/pages/099
splat_sources:
- title: SRD 5.2.1
  locator: pp. 99
  url: https://www.dndbeyond.com/srd
---

# Perfume (5 GP)

## Source text

<!-- SRD page 99 -->

Perfume comes in a 4-ounce vial. For 1 hour after applying Perfume to yourself, you have Advantage on Charisma (Persuasion) checks made to influence an Indifferent Humanoid within 5 feet of yourself.

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
| ` value ` | ` 0.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/perfume_5_gp ` |
| ` implementation ` | ` dnd2024:equipment_perfume_5_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` container ` | ` vial ` |
| ` volume_ounces ` | ` 4 ` |

#### Apply

<!-- splat:yaml path=splat_rule/parameters/apply -->

```yaml
target: self
duration:
  value: 1
  unit: hour
effect:
  roll: ability_check
  mode: advantage
  ability: charisma
  skill: persuasion
  target_type: humanoid
  target_attitude: indifferent
  target_max_distance_ft: 5
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0.0 ` |
| ` unit ` | ` lb ` |
| ` negligible ` | ` true ` |
