---
splat_schema: 1
splat_id: 83b18bd8-7cf2-5876-b671-37a806ea35b1
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_5_font_of_inspiration
splat_name: 'Level 5: Font of Inspiration'
splat_origin:
  key: dnd2024:reference/pages/032
splat_sources:
- title: SRD 5.2.1
  locator: pp. 32
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/bard_level_5_font_of_inspiration
  parameters:
    resource_recovery_override:
      id: bard.bardic_inspiration
      maximum:
        op: max
        args:
        - 1
        - var: actor.abilities.charisma.modifier
      recovery:
      - event: rest.short_completed
        amount: all
      - event: rest.long_completed
        amount: all
    conversion:
      action: none
      consume:
        resource: spell_slot
        count: 1
        minimum_level: 1
      restore:
        resource: bard.bardic_inspiration
        amount: 1
        expended_only: true
    class_scope:
      class: bard
      minimum_level: 5
  implementation: dnd2024:class_features_bard_level_5_font_of_inspiration
splat_contracts:
- id: dnd2024:class_features_bard_level_5_font_of_inspiration
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
  - rest.short_completed
  - rest.long_completed
  - resource.convert
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
  - 32
  engine_gaps: &id001
  - dnd2024:class_features_bard_level_5_font_of_inspiration
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: f76650b3af57d506387f80fa4cab7c2c36a1be6497149c708b028f0ae1f8b34b
  parent_name: Bard
  line_span:
    first: 2674
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 5: Font of Inspiration You now regain all your expended uses of Bardic Inspiration when
    you finish a Short or Long Rest. In addition, you can expend a spell slot (no action required) to
    regain one expended use of Bardic Inspiration.'
  sha256: b86763043182aa1bc538e55ef65a19132dd895d7657dc1b7c30f3c0cedc60ea2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 5: Font of Inspiration

<!-- SRD page 32 -->

You now regain all your expended uses of Bardic In-\
spiration when you finish a Short or Long Rest.\
In addition, you can expend a spell slot (no action\
required) to regain one expended use of Bardic\
Inspiration.\
