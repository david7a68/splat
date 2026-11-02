---
splat_schema: 1
splat_id: 06788234-addb-5feb-a49c-755ef501e2d4
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_half_damage
splat_name: Half Damage
splat_origin:
  key: dnd2024:reference/pages/016
splat_sources:
- title: SRD 5.2.1
  locator: pp. 16
  url: https://www.dndbeyond.com/srd
---

# Half Damage

## Source text

<!-- SRD page 16 -->

Many saving throw effects deal half damage (round down) to a target when the target succeeds on the saving throw. The halved damage is equal to half the damage that would be dealt on a failed save.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_half_damage ` |
| ` implementation ` | ` dnd2024:rules_damage_and_healing_half_damage ` |

### Parameters

#### Saving throw success damage

<!-- splat:yaml path=splat_rule/parameters/saving_throw_success_damage -->

```yaml
op: floor
args:
- op: divide
  args:
  - var: failed_save_damage
  - 2
```
