---
splat_schema: 1
splat_id: 7b6124b3-cd72-59c6-a5f0-323f4dd69247
splat_kind: rule
splat_key: dnd2024:rules/combat_controlling_a_mount
splat_name: Controlling a Mount
splat_origin:
  key: dnd2024:reference/pages/016
splat_sources:
- title: SRD 5.2.1
  locator: pp. 16
  url: https://www.dndbeyond.com/srd
---

# Controlling a Mount

## Source text

<!-- SRD page 16 -->

You can control a mount only if it has been trained\
to accept a rider. Domesticated horses, mules, and\
similar creatures have such training.\
The Initiative of a controlled mount changes to\
match yours when you mount it. It moves on your\
turn as you direct it, and it has only three action op-\
tions during that turn: Dash, Disengage, and Dodge.\
A controlled mount can move and act even on the\
turn that you mount it.\
In contrast, an independent mount—one that lets\
you ride but ignores your control—retains its place\
in the Initiative order and moves and acts as it likes.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_controlling_a_mount ` |
| ` implementation ` | ` dnd2024:rule_combat_controlling_a_mount ` |

### Parameters

#### Controlled

<!-- splat:values path=splat_rule/parameters/controlled -->

| Field | Value |
| --- | --- |
| ` requires ` | ` trained_to_accept_rider ` |
| ` initiative ` | ` rider_initiative ` |
| ` movement_turn ` | ` rider_turn ` |
| ` controller ` | ` rider ` |
| ` actions ` | ` ["dash", "disengage", "dodge"] ` |
| ` may_act_on_mounting_turn ` | ` true ` |

#### Independent

<!-- splat:values path=splat_rule/parameters/independent -->

| Field | Value |
| --- | --- |
| ` initiative ` | ` unchanged ` |
| ` controller ` | ` mount ` |
