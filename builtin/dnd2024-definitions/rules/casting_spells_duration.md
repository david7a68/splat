---
splat_schema: 1
splat_id: c17989ab-620a-5a65-9901-b4f1dd984243
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_duration
splat_name: Duration
splat_origin:
  key: dnd2024:reference/pages/106
splat_sources:
- title: SRD 5.2.1
  locator: pp. 106
  url: https://www.dndbeyond.com/srd
---

# Duration

## Source text

<!-- SRD page 106 -->

A spell’s duration is the length of time the spell persists after it is cast. A duration typically takes one of the following forms:

Concentration. A duration that requires Concentration follows the Concentration rules (see “Rules Glossary”).

Instantaneous. An instantaneous duration means the spell’s magic appears only for a moment and then disappears.

Time Span. A duration that provides a time span specifies how long the spell lasts in rounds, minutes, hours, or the like. For example, a Duration entry might say “1 minute,” meaning the spell ends after 1 minute has passed. While a time-span spell that you cast is ongoing, you can dismiss it (no action required) if you don’t have the Incapacitated condition.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/casting_spells_duration ` |
| ` implementation ` | ` dnd2024:rules_casting_spells_duration ` |

### Parameters

#### Duration forms

<!-- splat:yaml path=splat_rule/parameters/duration_forms -->

```yaml
concentration:
  key: dnd2024:glossary/concentration
instantaneous:
  persists: false
time_span:
  ends_at: elapsed_duration
  caster_dismissal:
    economy: none
    blocked_by: incapacitated
```
