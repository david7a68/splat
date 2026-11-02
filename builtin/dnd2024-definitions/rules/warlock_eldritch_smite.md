---
splat_schema: 1
splat_id: 000658d6-364c-5983-a6e0-70543064f0f9
splat_kind: rule
splat_key: dnd2024:rules/warlock_eldritch_smite
splat_name: Eldritch Smite
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72, 73
  url: https://www.dndbeyond.com/srd
---

# Eldritch Smite

## Source text

<!-- SRD page 72 -->

Prerequisite: Level 5+ Warlock, Pact of the Blade Invocation

Once per turn when you hit a creature with your pact weapon, you can expend a Pact Magic spell slot to deal an extra 1d8 Force damage to the target, plus another 1d8 per level of the spell slot, and you can give the target the Prone condition if it is Huge or smaller.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_eldritch_smite ` |
| ` implementation ` | ` dnd2024:rules_warlock_eldritch_smite ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` pact_weapon.hit_creature ` |
| ` uses_per_turn ` | ` 1 ` |

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` warlock_level_minimum ` | ` 5 ` |
| ` invocation ` | ` pact_of_the_blade ` |

#### Cost

<!-- splat:values path=splat_rule/parameters/cost -->

| Field | Value |
| --- | --- |
| ` resource ` | ` pact_magic_slot ` |
| ` amount ` | ` 1 ` |

#### Extra damage

<!-- splat:yaml path=splat_rule/parameters/extra_damage -->

```yaml
type: force
dice:
  count:
    op: add
    left: 1
    right: expended_slot_level
  sides: 8
```

#### Optional prone

<!-- splat:values path=splat_rule/parameters/optional_prone -->

| Field | Value |
| --- | --- |
| ` target_size_maximum ` | ` huge ` |
