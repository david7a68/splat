---
splat_schema: 2
splat_id: e5100277-58ae-5c6d-a845-6e7d3f1ae8b8
splat_kind: rule
splat_key: dnd2024:glossary/suffocation_hazard
splat_name: Suffocation [Hazard]
splat_origin: dnd2024:reference/glossary/suffocation_hazard
splat_sources:
- SRD 5.2.1, pp. 189, https://www.dndbeyond.com/srd
---

# Suffocation [Hazard]

## Source text

<!-- SRD page 189 -->

A creature can hold its breath for a number of minutes equal to 1 plus its Constitution modifier (minimum of 30 seconds) before suffocation begins. When a creature runs out of breath or is choking, it gains 1 Exhaustion level at the end of each of its turns. When a creature can breathe again, it removes all levels of Exhaustion it gained from suffocating.

Original layout: [[../../srd-5.2.1/pages/189|page 189]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/suffocation_hazard ` |
| ` implementation ` | ` dnd2024:glossary_suffocation_hazard ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` suffocating_when_any ` | ` ["out_of_breath", "choking"] ` |

#### Hold breath seconds

<!-- splat:yaml path=splat_rule/parameters/hold_breath_seconds -->

```yaml
op: max
args:
- 30
- op: multiply
  args:
  - 60
  - op: add
    args:
    - 1
    - var: constitution_modifier
```

#### Exhaustion

<!-- splat:values path=splat_rule/parameters/exhaustion -->

| Field | Value |
| --- | --- |
| ` timing ` | ` own_turn_end ` |
| ` levels ` | ` 1 ` |
| ` cause ` | ` suffocation ` |

#### Breathing resumes

<!-- splat:values path=splat_rule/parameters/breathing_resumes -->

| Field | Value |
| --- | --- |
| ` remove_exhaustion_levels ` | ` all_caused_by_suffocation ` |
