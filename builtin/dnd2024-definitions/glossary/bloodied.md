---
splat_schema: 1
splat_id: 3798113b-dd23-5a0c-bdf2-4995ce62efc7
splat_kind: rule
splat_key: dnd2024:glossary/bloodied
splat_name: Bloodied
splat_origin:
  key: dnd2024:reference/glossary/bloodied
splat_sources:
- title: SRD 5.2.1
  locator: pp. 177
  url: https://www.dndbeyond.com/srd
---

# Bloodied

## Source text

<!-- SRD page 177 -->

A creature is Bloodied while it has half its Hit Points or fewer remaining.

Original layout: [[../../srd-5.2.1/pages/177|page 177]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/bloodied ` |
| ` implementation ` | ` dnd2024:glossary_bloodied ` |

### Parameters

#### Predicate

<!-- splat:yaml path=splat_rule/parameters/predicate -->

```yaml
op: less_or_equal
args:
- var: current_hit_points
- op: divide
  args:
  - var: hit_point_maximum
  - 2
```
