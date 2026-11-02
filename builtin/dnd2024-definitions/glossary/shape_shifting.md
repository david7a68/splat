---
splat_schema: 1
splat_id: b42d9888-d40e-5013-a08a-742cb002b901
splat_kind: rule
splat_key: dnd2024:glossary/shape_shifting
splat_name: Shape-Shifting
splat_origin:
  key: dnd2024:reference/glossary/shape_shifting
splat_sources:
- title: SRD 5.2.1
  locator: pp. 187
  url: https://www.dndbeyond.com/srd
---

# Shape-Shifting

## Source text

<!-- SRD page 187 -->

If an effect, such as Wild Shape or the Polymorph spell, lets you shape-shift, its description specifies what happens to you. Unless that description says otherwise, any ongoing effects on you—conditions, spells, curses, and the like—carry over from one form to the other. You revert to your true form if you die.

Original layout: [[../../srd-5.2.1/pages/187|page 187]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/shape_shifting ` |
| ` implementation ` | ` dnd2024:glossary_shape_shifting ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` form_properties ` | ` source_effect_defined ` |
| ` ongoing_effects_carry_over_by_default ` | ` true ` |
| ` ongoing_effect_kinds ` | ` ["conditions", "spells", "curses", "other_effects"] ` |
| ` source_effect_override_allowed ` | ` true ` |
| ` death_reverts_to_true_form ` | ` true ` |
