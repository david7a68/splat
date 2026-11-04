---
splat_schema: 2
splat_id: 7556bf31-64d6-5732-89a5-292ea63b46a7
splat_kind: rule
splat_key: dnd2024:glossary/climbing
splat_name: Climbing
splat_origin: dnd2024:reference/glossary/climbing
splat_sources:
- SRD 5.2.1, pp. 178, 179, https://www.dndbeyond.com/srd
---

# Climbing

## Source text

<!-- SRD page 178 -->

While you’re climbing, each foot of movement costs 1 extra foot (2 extra feet in Difficult Terrain). You ignore this extra cost if you have a Climb Speed and use it to climb.

<!-- SRD page 179 -->

At the GM’s option, climbing a slippery surface or one with few handholds might require a successful DC 15 Strength (Athletics) check.

Original layout: [[../../srd-5.2.1/pages/178|page 178]], [[../../srd-5.2.1/pages/179|page 179]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/climbing ` |
| ` implementation ` | ` dnd2024:glossary_climbing ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` movement_mode ` | ` climb ` |

#### Per foot cost

<!-- splat:values path=splat_rule/parameters/per_foot_cost -->

| Field | Value |
| --- | --- |
| ` normal ` | ` 2 ` |
| ` difficult_terrain ` | ` 3 ` |

#### Mode extra cost ignored if

<!-- splat:values path=splat_rule/parameters/mode_extra_cost_ignored_if -->

| Field | Value |
| --- | --- |
| ` has_special_speed ` | ` climb ` |
| ` uses_special_speed ` | ` true ` |

#### Gm optional check

<!-- splat:yaml path=splat_rule/parameters/gm_optional_check -->

```yaml
terrain_any:
- slippery_surface
- few_handholds
required:
  chooser: gm
  options:
  - true
  - false
ability: strength
skill: athletics
dc: 15
success_required_to_move: true
```
