---
splat_schema: 1
splat_id: 8910f23e-77e5-525c-a122-2ab5a264c305
splat_kind: rule
splat_key: dnd2024:glossary/swimming
splat_name: Swimming
splat_origin:
  key: dnd2024:reference/glossary/swimming
splat_sources:
- title: SRD 5.2.1
  locator: pp. 189
  url: https://www.dndbeyond.com/srd
---

# Swimming

## Source text

<!-- SRD page 189 -->

While you’re swimming, each foot of movement\
costs 1 extra foot (2 extra feet in Difficult Terrain).\
You ignore this extra cost if you have a Swim Speed\
and use it to swim. At the GM’s option, moving any\
distance in rough water might require a successful\
DC 15 Strength (Athletics) check.\

Original layout: [[../../srd-5.2.1/pages/189|page 189]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/swimming ` |
| ` implementation ` | ` dnd2024:glossary_swimming ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` movement_mode ` | ` swim ` |

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
| ` has_special_speed ` | ` swim ` |
| ` uses_special_speed ` | ` true ` |

#### Gm optional check

<!-- splat:yaml path=splat_rule/parameters/gm_optional_check -->

```yaml
terrain_any:
- rough_water
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
