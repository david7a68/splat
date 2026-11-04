---
splat_schema: 2
splat_id: 6205a774-967b-5a3c-84ea-b1e5c1407d6b
splat_kind: rule
splat_key: dnd2024:rules/combat_mounting_and_dismounting
splat_name: Mounting and Dismounting
splat_origin: dnd2024:reference/pages/015
splat_sources:
- SRD 5.2.1, pp. 15, https://www.dndbeyond.com/srd
---

# Mounting and Dismounting

## Source text

<!-- SRD page 15 -->

During your move, you can mount a creature that is within 5 feet of you or dismount. Doing so costs an amount of movement equal to half your Speed (round down). For example, if your Speed is 30 feet, you spend 15 feet of movement to mount a horse.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_mounting_and_dismounting ` |
| ` implementation ` | ` dnd2024:rules_combat_mounting_and_dismounting ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` timing ` | ` during_movement ` |
| ` mount_distance_max_ft ` | ` 5 ` |
| ` choices ` | ` ["mount", "dismount"] ` |

#### Movement cost

<!-- splat:yaml path=splat_rule/parameters/movement_cost -->

```yaml
op: floor
args:
- op: divide
  args:
  - var: subject.speed
  - 2
```
