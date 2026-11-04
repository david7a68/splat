---
splat_schema: 2
splat_id: 470f050d-30bd-597d-ad11-c8d557a4463a
splat_kind: rule
splat_key: dnd2024:glossary/grappling
splat_name: Grappling
splat_origin: dnd2024:reference/glossary/grappling
splat_sources:
- SRD 5.2.1, pp. 182, https://www.dndbeyond.com/srd
---

# Grappling

## Source text

<!-- SRD page 182 -->

A creature can grapple another creature. Characters typically grapple by using an Unarmed Strike. Many monsters have special attacks that allow them to quickly grapple prey. However a grapple is initiated, it follows these rules. See also “Unarmed Strike” and “Grappled.”

Grappled Condition. Successfully grappling a creature gives it the Grappled condition.

One Grapple per Hand. A creature must have a hand free to grapple another creature. Some stat blocks and game effects allow a creature to grapple using a tentacle, a maw, or another body part. Whatever part a grappler uses, it can grapple only one creature at a time with that part, and the grappler can’t use that part to target another creature unless it ends the grapple.

Ending a Grapple. A Grappled creature can use its action to make a Strength (Athletics) or Dexterity (Acrobatics) check against the grapple’s escape DC, ending the condition on itself on a success. The condition also ends if the grappler has the Incapacitated condition or if the distance between the Grappled target and the grappler exceeds the grapple’s range. In addition, the grappler can release the target at any time (no action required).

Original layout: [[../../srd-5.2.1/pages/182|page 182]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/grappling ` |
| ` implementation ` | ` dnd2024:glossary_grappling ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` successful_grapple_condition ` | ` grappled ` |
| ` also_ends_when_any ` | ` ["grappler_incapacitated", "distance_exceeds_grapple_range"] ` |

#### Body part

<!-- splat:values path=splat_rule/parameters/body_part -->

| Field | Value |
| --- | --- |
| ` default ` | ` free_hand ` |
| ` alternatives_require_explicit_rule ` | ` true ` |
| ` targets_per_part ` | ` 1 ` |
| ` cannot_target_other_creatures_while_grappling ` | ` true ` |

#### Escape

<!-- splat:values path=splat_rule/parameters/escape -->

| Field | Value |
| --- | --- |
| ` cost ` | ` action ` |
| ` dc ` | ` grapple_escape_dc ` |
| ` success ` | ` end_grappled ` |

##### Check choice

<!-- splat:values path=splat_rule/parameters/escape/check_choice -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` subject ` |

###### Options

<!-- splat:table
path: splat_rule/parameters/escape/check_choice/options
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Ability:
    path: ability
    type: symbol
  Skill:
    path: skill
    type: symbol
-->

| Key | Ability | Skill |
| --- | --- | --- |
| ` row_1 ` | ` Strength ` | ` Athletics ` |
| ` row_2 ` | ` Dexterity ` | ` Acrobatics ` |

#### Release

<!-- splat:values path=splat_rule/parameters/release -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` grappler ` |
| ` timing ` | ` any_time ` |
| ` cost ` | ` none ` |
