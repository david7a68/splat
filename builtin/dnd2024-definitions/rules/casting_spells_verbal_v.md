---
splat_schema: 1
splat_id: 0e3b269c-567c-55f8-81a7-715243cc0c86
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_verbal_v
splat_name: Verbal (V)
splat_origin:
  key: dnd2024:reference/pages/105
splat_sources:
- title: SRD 5.2.1
  locator: pp. 105
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells_verbal_v
  parameters:
    verbal:
      requires_normal_speaking_voice: true
      blocked_by:
      - gagged
      - magical_silence
  implementation: dnd2024:rules_casting_spells_verbal_v
splat_contracts:
- id: dnd2024:rules_casting_spells_verbal_v
  inputs:
    caster:
      type: participant
      required: true
    spell:
      type: definition
      required: true
    event:
      type: event
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - spell_state
  - casting_requirements
  events: []
  capabilities:
  - spellcasting
  - concentration
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 105
  engine_gaps: &id001
  - dnd2024:rules_casting_spells_verbal_v
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 4a08743db82c479430c098fb882b374ac21e20dd19ae1088868fcbc7fb509ca0
  parent_name: Casting Spells
  line_span:
    first: 9197
    count: 10
splat_clauses:
- key: source_rules
  text: '# Verbal (V) A Verbal component is the chanting of esoteric words that sound like nonsense to
    the uninitiated. The words must be uttered in a normal speaking voice. The words themselves aren’t
    the source of the spell’s power; rather, the particular combination of sounds, with specific pitch
    and resonance, sets the threads of magic in motion. Thus, a creature who is gagged or in an area of
    magical silence can’t cast a spell with a Verbal component.'
  sha256: 27a156906641ce5695c9b5f8d6f5e3150a44ef2e8bebeafb7ef1cf399ee8465c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Verbal (V)

<!-- SRD page 105 -->

A Verbal component is the chanting of esoteric\
words that sound like nonsense to the uninitiated.\
The words must be uttered in a normal speaking\
voice. The words themselves aren’t the source of the\
spell’s power; rather, the particular combination of\
sounds, with specific pitch and resonance, sets the\
threads of magic in motion. Thus, a creature who is\
gagged or in an area of magical silence can’t cast a\
spell with a Verbal component.\
