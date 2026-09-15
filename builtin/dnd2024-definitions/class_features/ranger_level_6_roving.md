---
splat_schema: 1
splat_id: c615f96a-0485-54ce-9e1e-9686fefdd8cf
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_6_roving
splat_name: 'Level 6: Roving'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
---

# Level 6: Roving

## Source text

<!-- SRD page 59 -->

Your Speed increases by 10 feet while you aren’t\
wearing Heavy armor. You also have a Climb Speed\
and a Swim Speed equal to your Speed.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/ranger_level_6_roving ` |
| ` implementation ` | ` dnd2024:class_features_ranger_level_6_roving ` |

### Parameters

#### Speed bonus

<!-- splat:values path=splat_rule/parameters/speed_bonus -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 10 ` |
| ` unless_armor ` | ` heavy ` |

#### Movement speeds

<!-- splat:yaml path=splat_rule/parameters/movement_speeds -->

```yaml
climb: &id001
  stat: speed
swim: *id001
```
