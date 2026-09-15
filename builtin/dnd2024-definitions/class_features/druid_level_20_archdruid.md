---
splat_schema: 1
splat_id: f0c0deaa-e5fa-524d-9813-d6bffca0b9cc
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_20_archdruid
splat_name: 'Level 20: Archdruid'
splat_origin:
  key: dnd2024:reference/pages/043
splat_sources:
- title: SRD 5.2.1
  locator: pp. 43
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/druid_level_20_archdruid
  parameters:
    evergreen_wild_shape:
      trigger: initiative.rolled
      requires:
        wild_shape_uses: 0
      restore_uses: 1
    nature_magician:
      action: none
      resource:
        id: archdruid.nature_magician
        maximum: 1
        recovery:
        - event: rest.long_completed
          amount: all
      cost_uses: 1
      convert:
        resource: druid.wild_shape
        choose_unexpended_count: true
        minimum_use_count: 1
        create_spell_slot_count: 1
        slot_level:
          op: multiply
          args:
          - 2
          - var: chosen_use_count
    aging:
      body_years_per_elapsed_year: 0.1
    class_scope:
      class: druid
      minimum_level: 20
  implementation: dnd2024:class_features_druid_level_20_archdruid
splat_contracts:
- id: dnd2024:class_features_druid_level_20_archdruid
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
  - initiative.rolled
  - resource.convert
  - rest.long_completed
  - time.elapsed
  capabilities:
  - character_features
  - resources
  - spellcasting
  - initiative
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 43
  engine_gaps: &id001
  - dnd2024:class_features_druid_level_20_archdruid
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 3a5fbd2ccc916a81041293cc678619dbbb62d499957222e658ed0dc491b2fd68
  parent_name: Druid
  line_span:
    first: 3619
    count: 17
splat_clauses:
- key: source_rules
  text: '# Level 20: Archdruid The vitality of nature constantly blooms within you, granting you the following
    benefits. Evergreen Wild Shape. Whenever you roll Initiative and have no uses of Wild Shape left,
    you regain one expended use of it. Nature Magician. You can convert uses of Wild Shape into a spell
    slot (no action required). Choose a number of your unexpended uses of Wild Shape and convert them
    into a single spell slot, with each use contributing 2 spell levels. For example, if you convert two
    uses of Wild Shape, you produce a level 4 spell slot. Once you use this benefit, you can’t do so again
    until you finish a Long Rest. Longevity. The primal magic that you wield causes you to age more slowly.
    For every ten years that pass, your body ages only one year.'
  sha256: ebb662b313d593cbc91db0f94e17eaa38773a4acf18e0dd3e1c8fa1f590f795b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 20: Archdruid

<!-- SRD page 43 -->

The vitality of nature constantly blooms within you,\
granting you the following benefits.\
Evergreen Wild Shape. Whenever you roll Initia-\
tive and have no uses of Wild Shape left, you regain\
one expended use of it.\
Nature Magician. You can convert uses of Wild\
Shape into a spell slot (no action required). Choose\
a number of your unexpended uses of Wild Shape\
and convert them into a single spell slot, with each\
use contributing 2 spell levels. For example, if you\
convert two uses of Wild Shape, you produce a level\
4 spell slot. Once you use this benefit, you can’t do\
so again until you finish a Long Rest.\
Longevity. The primal magic that you wield\
causes you to age more slowly. For every ten years\
that pass, your body ages only one year.\
