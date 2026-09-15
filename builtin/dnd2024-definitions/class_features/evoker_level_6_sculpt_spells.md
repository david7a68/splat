---
splat_schema: 1
splat_id: 7691b3f2-db16-5bb6-a466-04cc54464ebe
splat_kind: rule
splat_key: dnd2024:class_features/evoker_level_6_sculpt_spells
splat_name: 'Level 6: Sculpt Spells'
splat_origin:
  key: dnd2024:reference/pages/082
splat_sources:
- title: SRD 5.2.1
  locator: pp. 82
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/evoker_level_6_sculpt_spells
  parameters:
    trigger: actor.casts_evocation_spell
    targets:
      not_actor: true
      affected_by_spell: true
      visible_to_actor: true
      maximum:
        op: add
        args:
        - 1
        - var: spell.level
      selection: actor_choice
    saving_throws_against_spell: automatic_success
    damage_on_success:
      if_normally_half: 0
    class_scope:
      class: wizard
      minimum_level: 6
  implementation: dnd2024:class_features_evoker_level_6_sculpt_spells
splat_contracts:
- id: dnd2024:class_features_evoker_level_6_sculpt_spells
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
  - spell.cast
  - saving_throw
  - damage.pending
  capabilities:
  - character_features
  - spellcasting
  - damage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 82
  engine_gaps: &id001
  - dnd2024:class_features_evoker_level_6_sculpt_spells
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 5bf10bc284d8e9f023b20dd1c77b9319301e125f214bda36a3f02da033e875b5
  parent_name: Evoker
  line_span:
    first: 7033
    count: 9
splat_clauses:
- key: source_rules
  text: '# Level 6: Sculpt Spells You can create pockets of relative safety within the effects of your
    evocations. When you cast an Evocation spell that affects other creatures that you can see, you can
    choose a number of them equal to 1 plus the spell’s level. The chosen creatures automatically succeed
    on their saving throws against the spell, and they take no damage if they would normally take half
    damage on a successful save.'
  sha256: a7abbfe2666d9eb4309632f42c0d88ccdca5bf88966a6df64b76b94332e2ddef
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 6: Sculpt Spells

<!-- SRD page 82 -->

You can create pockets of relative safety within the\
effects of your evocations. When you cast an Evoca-\
tion spell that affects other creatures that you can\
see, you can choose a number of them equal to 1\
plus the spell’s level. The chosen creatures automat-\
ically succeed on their saving throws against the\
spell, and they take no damage if they would nor-\
mally take half damage on a successful save.\
