---
splat_schema: 1
splat_id: cd6350c7-7f49-5f31-a86f-2de36a75a108
splat_kind: rule
splat_key: dnd2024:glossary/damage_threshold
splat_name: Damage Threshold
splat_origin:
  key: dnd2024:reference/glossary/damage_threshold
splat_sources:
- title: SRD 5.2.1
  locator: pp. 180
  url: https://www.dndbeyond.com/srd
---

# Damage Threshold

## Source text

<!-- SRD page 180 -->

A creature or an object that has a damage threshold has Immunity to all damage unless it takes an amount of damage from a single attack or effect equal to or greater than its damage threshold, in which case it takes that entire instance of damage. Any damage that fails to meet or exceed the damage threshold is superficial and doesn’t reduce Hit Points. For example, if an object has a damage threshold of 10, the object takes no damage if 9 damage is dealt to it, since that damage fails to exceed the threshold. If the same object is dealt 11 damage, it takes all of that damage.

Original layout: [[../../srd-5.2.1/pages/180|page 180]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/damage_threshold ` |
| ` implementation ` | ` dnd2024:glossary_damage_threshold ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` scope ` | ` single_attack_or_effect_instance ` |
| ` fail_damage ` | ` 0 ` |
| ` fail_immunity ` | ` true ` |
| ` threshold_is_not_subtracted ` | ` true ` |

#### Pass predicate

<!-- splat:values path=splat_rule/parameters/pass_predicate -->

| Field | Value |
| --- | --- |
| ` op ` | ` greater_or_equal ` |

##### Args

<!-- splat:table
path: splat_rule/parameters/pass_predicate/args
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Var:
    path: var
    type: symbol
-->

| Key | Var |
| --- | --- |
| ` row_1 ` | ` Damage amount ` |
| ` row_2 ` | ` Threshold ` |

#### Pass damage

<!-- splat:values path=splat_rule/parameters/pass_damage -->

| Field | Value |
| --- | --- |
| ` var ` | ` damage_amount ` |
