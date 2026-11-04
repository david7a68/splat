---
splat_schema: 2
splat_id: 90f2f9db-29eb-5ade-a417-c59abc81984c
splat_kind: rule_group
splat_key: dnd2024:species/human
splat_name: Human
splat_origin: dnd2024:reference/pages/086
splat_sources:
- SRD 5.2.1, pp. 86, https://www.dndbeyond.com/srd
---

# Human

## Source text

<!-- SRD page 86 -->

Creature Type: Humanoid\
Size: Medium (about 4–7 feet tall) or Small (about 2–4 feet tall), chosen when you select this species\
Speed: 30 feet

As a Human, you have these special traits.

Resourceful. You gain Heroic Inspiration whenever you finish a Long Rest.

Skillful. You gain proficiency in one skill of your choice.

Versatile. You gain an Origin feat of your choice (see “Feats”). Skilled is recommended.

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` species ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:species/human ` |
| ` implementation ` | ` dnd2024:species_human ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` creature_type ` | ` humanoid ` |
| ` speed_feet ` | ` 30 ` |

#### Size choice

<!-- splat:values path=splat_rule/parameters/size_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |

##### Options

<!-- splat:table
path: splat_rule/parameters/size_choice/options
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Size:
    path: size
    type: symbol
  Height feet approximate minimum:
    path: height_feet_approximate/minimum
    type: integer
  Height feet approximate maximum:
    path: height_feet_approximate/maximum
    type: integer
-->

| Key | Size | Height feet approximate minimum | Height feet approximate maximum |
| --- | --- | --- | --- |
| ` row_1 ` | ` Medium ` | ` 4 ` | ` 7 ` |
| ` row_2 ` | ` Small ` | ` 2 ` | ` 4 ` |

#### Resourceful

<!-- splat:values path=splat_rule/parameters/resourceful -->

| Field | Value |
| --- | --- |
| ` event ` | ` rest.long_rest_completed ` |
| ` grant ` | ` heroic_inspiration ` |

#### Skillful

<!-- splat:yaml path=splat_rule/parameters/skillful -->

```yaml
proficiency_choice:
  count: 1
  category: any_skill
```

#### Versatile

<!-- splat:yaml path=splat_rule/parameters/versatile -->

```yaml
feat_choice:
  count: 1
  category: origin
```
