---
splat_schema: 1
splat_id: d3277324-ecf6-55cf-9a82-c019bc9e9372
splat_kind: rule
splat_key: dnd2024:class_features/wizard_level_20_signature_spells
splat_name: 'Level 20: Signature Spells'
splat_origin:
  key: dnd2024:reference/pages/079
splat_sources:
- title: SRD 5.2.1
  locator: pp. 79
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/wizard_level_20_signature_spells
  parameters:
    choose_spells:
      count: 2
      level: 3
      in_spellbook: true
    always_prepared: true
    counts_against_prepared_limit: false
    free_cast:
      per_spell_resource:
        id: wizard.signature_spell
        maximum: 1
        recovery:
        - event: rest.short_completed
          amount: all
        - event: rest.long_completed
          amount: all
      cost: 1
      cast_level: 3
      expend_slot: false
    higher_level_casting:
      expend_slot: true
    class_scope:
      class: wizard
      minimum_level: 20
  implementation: dnd2024:class_features_wizard_level_20_signature_spells
splat_contracts:
- id: dnd2024:class_features_wizard_level_20_signature_spells
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - character.feature_acquired
  - rest.short_completed
  - rest.long_completed
  - spell.cast
  capabilities:
  - character_features
  - resources
  - spellcasting
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 79
  engine_gaps: &id001
  - dnd2024:class_features_wizard_level_20_signature_spells
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: da26c1970208b395336461852b708bc25be59418fc1d114b9cbfcf0109589131
  parent_name: Wizard
  line_span:
    first: 6752
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 20: Signature Spells Choose two level 3 spells in your spellbook as your signature spells.
    You always have these spells prepared, and you can cast each of them once at level 3 without expending
    a spell slot. When you do so, you can’t cast them in this way again until you finish a Short or Long
    Rest. To cast either spell at a higher level, you must expend a spell slot.'
  sha256: 96b04e6179028e8e432199aab7883b2eec6a00c635f13130fcd3ed841976f17c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 20: Signature Spells

<!-- SRD page 79 -->

Choose two level 3 spells in your spellbook as your\
signature spells. You always have these spells pre-\
pared, and you can cast each of them once at level 3\
without expending a spell slot. When you do so, you\
can’t cast them in this way again until you finish a\
Short or Long Rest. To cast either spell at a higher\
level, you must expend a spell slot.\
