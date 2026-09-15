---
splat_schema: 1
splat_id: ae96b156-df76-5e73-9bcb-fef056e1f8fe
splat_kind: rule
splat_key: dnd2024:rules/fear_and_mental_stress_prolonged_effects
splat_name: Prolonged Effects
splat_origin:
  key: dnd2024:reference/pages/196
splat_sources:
- title: SRD 5.2.1
  locator: pp. 196
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/fear_and_mental_stress_prolonged_effects
  parameters:
    application:
      authority: gm
      cause: mental_stress
      choice:
      - short_term
      - long_term
      - indefinite
    short_term:
      condition_choice:
      - frightened
      - incapacitated
      - stunned
      duration:
        dice:
          count: 1
          sides: 10
        unit: minute
      optional_narrative:
      - alarming_behavior
      - hallucinations
      suppressed_by:
        key: dnd2024:spells/calm_emotions
      removed_by:
        key: dnd2024:spells/lesser_restoration
    long_term:
      disadvantage: selected_ability_checks
      ability_scope:
        authority: gm
        choose_min: 1
        choose_max: 6
        options:
        - strength
        - dexterity
        - constitution
        - intelligence
        - wisdom
        - charisma
      duration:
        dice:
          count: 1
          sides: 10
        multiplier: 10
        unit: hour
      suppressed_by:
        key: dnd2024:spells/calm_emotions
      removed_by:
        key: dnd2024:spells/lesser_restoration
    indefinite:
      effect_template: long_term
      duration: until_removed
      suppressed_by:
        key: dnd2024:spells/calm_emotions
      removed_by:
        key: dnd2024:spells/greater_restoration
      lesser_restoration_removes: false
  implementation: dnd2024:rules_fear_and_mental_stress_prolonged_effects
splat_contracts:
- id: dnd2024:rules_fear_and_mental_stress_prolonged_effects
  inputs:
    subject:
      type: participant
      required: true
    effect_mode:
      type: choice
      required: true
    condition_choice:
      required: false
      type: choice
    affected_abilities:
      required: false
      type: choice
  outputs:
  - stress_effect
  events: []
  capabilities:
  - mental_stress
  - conditions
  - checks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 196
  engine_gaps: &id001
  - dnd2024:rules_fear_and_mental_stress_prolonged_effects
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: a9701889eccb9fc294a7d0b0f6a456164abb59f73192eede7ed540a6a8a517b7
  parent_name: Fear and Mental Stress
  line_span:
    first: 9748
    count: 23
splat_clauses:
- key: source_rules
  text: '# Prolonged Effects Exposure to mental stress can cause prolonged effects. Consider the following
    possibilities. Short-Term Effects. The character has the Frightened, Incapacitated, or Stunned condition
    for 1d10 minutes. This condition might be accompanied by alarming behavior or hallucinations. These
    effects can be suppressed with the Calm Emotions spell or removed by the Lesser Restoration spell.
    Long-Term Effects. The character has Disadvantage on some or all ability checks for 1d10 × 10 hours,
    stemming from an unwillingness or inability to exert a particular set of abilities. The character
    might feel enervated and unable to exert much Strength, for example, or become so suspicious of others
    that Charisma checks are more difficult. These effects can be suppressed with the Calm Emotions spell
    or removed by the Lesser Restoration spell. Indefinite Effects. An indefinite effect is a longterm
    effect (see above) that lasts until removed by a Greater Restoration spell. It can be suppressed by
    a Calm Emotions spell.'
  sha256: f6cccd927d3437583b92dcacab8d198c1e9de137fdf4b62d476f002daa0574fb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Prolonged Effects

<!-- SRD page 196 -->

Exposure to mental stress can cause prolonged ef-\
fects. Consider the following possibilities.\
Short-Term Effects. The character has the Fright-\
ened, Incapacitated, or Stunned condition for 1d10\
minutes. This condition might be accompanied by\
alarming behavior or hallucinations. These effects\
can be suppressed with the Calm Emotions spell or\
removed by the Lesser Restoration spell.\
Long-Term Effects. The character has Disad-\
vantage on some or all ability checks for 1d10 × 10\
hours, stemming from an unwillingness or inability\
to exert a particular set of abilities. The character\
might feel enervated and unable to exert much\
Strength, for example, or become so suspicious of\
others that Charisma checks are more difficult.\
These effects can be suppressed with the Calm\
Emotions spell or removed by the Lesser Restoration\
spell.\
Indefinite Effects. An indefinite effect is a long-\
term effect (see above) that lasts until removed by a\
Greater Restoration spell. It can be suppressed by a\
Calm Emotions spell.\
