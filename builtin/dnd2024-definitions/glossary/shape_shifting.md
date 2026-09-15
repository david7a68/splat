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
splat_rule:
  mechanic: dnd2024:glossary/shape_shifting
  parameters:
    form_properties: source_effect_defined
    ongoing_effects_carry_over_by_default: true
    ongoing_effect_kinds:
    - conditions
    - spells
    - curses
    - other_effects
    source_effect_override_allowed: true
    death_reverts_to_true_form: true
  implementation: dnd2024:glossary_shape_shifting
splat_contracts:
- id: dnd2024:glossary_shape_shifting
  inputs:
    subject:
      type: participant
      required: true
    transform_effect:
      type: effect
      required: true
  outputs:
  - form
  - retained_effects
  events:
  - shape.changed
  - creature.died
  capabilities:
  - transformation
  - effects
splat_conversion:
  status: specified
  category: glossary
  source_sha256: aaaa8f9c8ab359dc09dfa8261743b92c71c5c7fc7ba961f183e08cc04b989d7b
  source_path: builtin/srd-5.2.1/glossary/shape_shifting.md
  pages:
  - 187
  engine_gaps: &id001
  - dnd2024:glossary_shape_shifting
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 19975f876e1b48c26d7896c53f6b32831aa2ca525bea1bde60768c8558837115
splat_clauses:
- key: source_rules
  text: '# Shape-Shifting If an effect, such as Wild Shape or the Polymorph spell, lets you shape-shift,
    its description specifies what happens to you. Unless that description says otherwise, any ongoing
    effects on you—conditions, spells, curses, and the like—carry over from one form to the other. You
    revert to your true form if you die.'
  sha256: f36ba44cd428aa947316c7218ed083a96ccd9c0655d8f8ca1643da80a49aff78
  status: contract_specified
splat_requires_mechanics: *id001
---

# Shape-Shifting

<!-- SRD page 187 -->

If an effect, such as Wild Shape or the Polymorph\
spell, lets you shape-shift, its description specifies\
what happens to you. Unless that description says\
otherwise, any ongoing effects on you—conditions,\
spells, curses, and the like—carry over from one\
form to the other. You revert to your true form if\
you die.\

Original layout: [[../../srd-5.2.1/pages/187|page 187]].
