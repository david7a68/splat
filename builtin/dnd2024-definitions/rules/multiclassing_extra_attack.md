---
splat_schema: 2
splat_id: 47bb1c5b-e5cc-5360-914e-4c91ed046089
splat_kind: rule
splat_key: dnd2024:rules/multiclassing_extra_attack
splat_name: Extra Attack
splat_origin: dnd2024:reference/pages/025
splat_sources:
- SRD 5.2.1, pp. 25, https://www.dndbeyond.com/srd
---

# Extra Attack

## Source text

<!-- SRD page 25 -->

If you gain the Extra Attack feature from more than one class, the features don’t stack. You can’t make more than two attacks with this feature unless you have a feature that says you can (such as the Fighter’s Two Extra Attacks feature).

Similarly, the Warlock’s Thirsting Blade invocation, which grants you the Extra Attack feature with your pact weapon, doesn’t give you additional attacks if you also have Extra Attack.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/multiclassing_extra_attack ` |
| ` implementation ` | ` dnd2024:rules_multiclassing_extra_attack ` |

### Parameters

#### Extra attack

<!-- splat:values path=splat_rule/parameters/extra_attack -->

| Field | Value |
| --- | --- |
| ` stacks_across_classes ` | ` false ` |
| ` default_attacks ` | ` 2 ` |
| ` higher_count_requires_explicit_feature ` | ` true ` |

#### Thirsting blade

<!-- splat:values path=splat_rule/parameters/thirsting_blade -->

| Field | Value |
| --- | --- |
| ` stacks_with_extra_attack ` | ` false ` |
| ` weapon_scope ` | ` pact_weapon ` |
