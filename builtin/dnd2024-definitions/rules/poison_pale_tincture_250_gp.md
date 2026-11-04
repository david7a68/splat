---
splat_schema: 2
splat_id: ace7edef-f701-5a08-8e58-48f203fe472e
splat_kind: rule
splat_key: dnd2024:rules/poison_pale_tincture_250_gp
splat_name: Pale Tincture (250 GP)
splat_origin: dnd2024:reference/pages/198
splat_sources:
- SRD 5.2.1, pp. 198, https://www.dndbeyond.com/srd
---

# Pale Tincture (250 GP)

## Source text

<!-- SRD page 198 -->

Ingested Poison

A creature subjected to Pale Tincture must succeed on a DC 16 Constitution saving throw or take 3 (1d6) Poison damage and have the Poisoned condition. The Poisoned creature repeats the save every 24 hours, taking 3 (1d6) Poison damage on a failed save. The damage the poison deals can’t be healed by any means while the creature remains Poisoned. After seven successful saves against the poison, the creature is no longer Poisoned.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/poison_pale_tincture_250_gp ` |
| ` implementation ` | ` dnd2024:rules_poison_pale_tincture_250_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost_gp ` | ` 250 ` |
| ` exposure_route ` | ` ingested ` |

#### Initial save

<!-- splat:values path=splat_rule/parameters/initial_save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 16 ` |

#### Initial failure

<!-- splat:yaml path=splat_rule/parameters/initial_failure -->

```yaml
damage:
  dice:
    count: 1
    sides: 6
  average: 3
  type: poison
condition: poisoned
```

#### Repeat while poisoned

<!-- splat:yaml path=splat_rule/parameters/repeat_while_poisoned -->

```yaml
interval:
  value: 24
  unit: hour
ability: constitution
dc: 16
failure_damage:
  dice:
    count: 1
    sides: 6
  average: 3
  type: poison
```

#### While poisoned

<!-- splat:values path=splat_rule/parameters/while_poisoned -->

| Field | Value |
| --- | --- |
| ` healing_prohibited_for ` | ` damage_from_this_poison ` |
| ` prohibition_scope ` | ` all_means ` |

#### Ends

<!-- splat:values path=splat_rule/parameters/ends -->

| Field | Value |
| --- | --- |
| ` successful_saves ` | ` 7 ` |
| ` consecutive_required ` | ` false ` |
