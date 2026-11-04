---
splat_schema: 2
splat_id: 2a99d2e1-62c2-52c0-95f6-16a7715c9305
splat_kind: rule
splat_key: dnd2024:rules/weapons_ammunition
splat_name: Ammunition
splat_origin: dnd2024:reference/pages/089
splat_sources:
- SRD 5.2.1, pp. 89, https://www.dndbeyond.com/srd
---

# Ammunition

## Source text

<!-- SRD page 89 -->

You can use a weapon that has the Ammunition property to make a ranged attack only if you have ammunition to fire from it. The type of ammunition required is specified with the weapon’s range. Each attack expends one piece of ammunition. Drawing the ammunition is part of the attack (you need a free hand to load a one-handed weapon). After a fight, you can spend 1 minute to recover half the ammunition (round down) you used in the fight; the rest is lost.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_ammunition ` |
| ` implementation ` | ` dnd2024:rule_weapons_ammunition ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` ranged_attack_requires ` | ` compatible_ammunition ` |
| ` ammunition_consumed_per_attack ` | ` 1 ` |
| ` draw_ammunition_cost ` | ` part_of_attack ` |
| ` load_one_handed_weapon_requires_free_hand ` | ` true ` |

#### Recover after combat

<!-- splat:yaml path=splat_rule/parameters/recover_after_combat -->

```yaml
time_minutes: 1
quantity:
  op: floor
  args:
  - op: divide
    args:
    - variable: ammunition_used
    - 2
remaining_ammunition: lost
```
