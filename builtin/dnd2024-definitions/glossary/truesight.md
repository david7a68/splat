---
splat_schema: 2
splat_id: f4525cb7-2409-5126-bded-aea80329c9ed
splat_kind: rule
splat_key: dnd2024:glossary/truesight
splat_name: Truesight
splat_origin: dnd2024:reference/glossary/truesight
splat_sources:
- SRD 5.2.1, pp. 190, https://www.dndbeyond.com/srd
---

# Truesight

## Source text

<!-- SRD page 190 -->

If you have Truesight, your vision is enhanced within a specified range. Within that range, your vision pierces through the following:

Darkness. You can see in normal and magical Darkness.

Invisibility. You see creatures and objects that have the Invisible condition.

Visual Illusions. Visual illusions appear transparent to you, and you automatically succeed on saving throws against them.

Transformations. You discern the true form of any creature or object you see that has been transformed by magic.

Ethereal Plane. You see into the Ethereal Plane.

Original layout: [[../../srd-5.2.1/pages/190|page 190]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/truesight ` |
| ` implementation ` | ` dnd2024:glossary_truesight ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` range ` | ` feature_specified ` |
| ` see_in_darkness ` | ` ["normal", "magical"] ` |
| ` see_invisible ` | ` ["creatures", "objects"] ` |
| ` see_into_plane ` | ` ethereal ` |

#### Visual illusions

<!-- splat:values path=splat_rule/parameters/visual_illusions -->

| Field | Value |
| --- | --- |
| ` appearance ` | ` transparent ` |
| ` saves ` | ` automatic_success ` |

#### Magical transformations

<!-- splat:values path=splat_rule/parameters/magical_transformations -->

| Field | Value |
| --- | --- |
| ` discern_true_form ` | ` true ` |
| ` requires_seeing_target ` | ` true ` |
| ` target_types ` | ` ["creature", "object"] ` |
