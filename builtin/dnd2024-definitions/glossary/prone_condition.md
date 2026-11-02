---
splat_schema: 1
splat_id: 4a852418-1cfd-599d-8511-812b3997433c
splat_kind: rule
splat_key: dnd2024:glossary/prone_condition
splat_name: Prone [Condition]
splat_origin:
  key: dnd2024:reference/glossary/prone_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 186
  url: https://www.dndbeyond.com/srd
---

# Prone [Condition]

## Source text

<!-- SRD page 186 -->

While you have the Prone condition, you experience the following effects.

Restricted Movement. Your only movement options are to crawl or to spend an amount of movement equal to half your Speed (round down) to right yourself and thereby end the condition. If your Speed is 0, you can’t right yourself.

Attacks Affected. You have Disadvantage on attack rolls. An attack roll against you has Advantage if the attacker is within 5 feet of you. Otherwise, that attack roll has Disadvantage.

Original layout: [[../../srd-5.2.1/pages/186|page 186]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/prone_condition ` |
| ` implementation ` | ` dnd2024:glossary_prone_condition ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition ` | ` prone ` |
| ` movement_options ` | ` ["crawl", "right_self"] ` |
| ` own_attack_rolls ` | ` disadvantage ` |

#### Right self

<!-- splat:yaml path=splat_rule/parameters/right_self -->

```yaml
movement_cost:
  op: floor
  args:
  - op: divide
    args:
    - var: speed
    - 2
requires_speed_greater_than: 0
ends_condition: true
```

#### Incoming attacks

<!-- splat:values path=splat_rule/parameters/incoming_attacks -->

| Field | Value |
| --- | --- |
| ` attacker_distance_feet_max_5 ` | ` advantage ` |
| ` attacker_distance_feet_greater_5 ` | ` disadvantage ` |
