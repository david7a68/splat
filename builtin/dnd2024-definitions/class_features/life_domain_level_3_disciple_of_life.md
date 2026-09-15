---
splat_schema: 1
splat_id: 15d20d39-9010-5ff8-bb78-716c6e07bef5
splat_kind: rule
splat_key: dnd2024:class_features/life_domain_level_3_disciple_of_life
splat_name: 'Level 3: Disciple of Life'
splat_origin:
  key: dnd2024:reference/pages/040
splat_sources:
- title: SRD 5.2.1
  locator: pp. 40
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/life_domain_level_3_disciple_of_life
  parameters:
    trigger: spell.healing
    requires:
      caster: actor
      expended_spell_slot: true
      turn: spell.cast_turn
    additional_healing:
      op: add
      args:
      - 2
      - var: spell.slot_level
    class_scope:
      class: cleric
      minimum_level: 3
  implementation: dnd2024:class_features_life_domain_level_3_disciple_of_life
splat_contracts:
- id: dnd2024:class_features_life_domain_level_3_disciple_of_life
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
  - spell.healing
  capabilities:
  - character_features
  - spellcasting
  - healing
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 40
  engine_gaps: &id001
  - dnd2024:class_features_life_domain_level_3_disciple_of_life
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: f328d73b8fa7aba11b9de4d6785fb2193c739fc883482b792ee072622a057739
  parent_name: Life Domain
  line_span:
    first: 3298
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 3: Disciple of Life When a spell you cast with a spell slot restores Hit Points to a
    creature, that creature regains additional Hit Points on the turn you cast the spell. The additional
    Hit Points equal 2 plus the spell slot’s level.'
  sha256: 1b492c24f187cc13b46c8f29e3dc52451acd8b9b8e8df15b26e8f9737adb6e51
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Disciple of Life

<!-- SRD page 40 -->

When a spell you cast with a spell slot restores Hit\
Points to a creature, that creature regains addi-\
tional Hit Points on the turn you cast the spell. The\
additional Hit Points equal 2 plus the spell slot’s\
level.\
