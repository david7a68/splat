---
splat_schema: 1
splat_id: d61bdb61-79ee-54bc-a351-233ed15ee7ae
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_1_favored_enemy
splat_name: 'Level 1: Favored Enemy'
splat_origin:
  key: dnd2024:reference/pages/058
splat_sources:
- title: SRD 5.2.1
  locator: pp. 58
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/ranger_level_1_favored_enemy
  parameters:
    always_prepared:
    - key: dnd2024:spells/hunter_s_mark
    counts_against_prepared_limit: false
    free_cast:
      resource:
        id: ranger.favored_enemy
        maximum:
          table:
            key: dnd2024:tables/ranger_progression
          row:
            var: actor.class_levels.ranger
          column: favored_enemy
        recovery:
        - event: rest.long_completed
          amount: all
      cost: 1
      expend_slot: false
    class_scope:
      class: ranger
      minimum_level: 1
  implementation: dnd2024:class_features_ranger_level_1_favored_enemy
splat_contracts:
- id: dnd2024:class_features_ranger_level_1_favored_enemy
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
  - prepared_spells
  - resources
  events:
  - spell.cast
  - rest.long_completed
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
  - 58
  engine_gaps: &id001
  - dnd2024:class_features_ranger_level_1_favored_enemy
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 01cac4217b362242e8075d1bdb0a9386185fb086c61e3d71d1bd4c4ccfaf9224
  parent_name: Ranger
  line_span:
    first: 4913
    count: 9
splat_clauses:
- key: source_rules
  text: '# Level 1: Favored Enemy You always have the Hunter’s Mark spell prepared. You can cast it twice
    without expending a spell slot, and you regain all expended uses of this ability when you finish a
    Long Rest. The number of times you can cast the spell without a spell slot increases when you reach
    certain Ranger levels, as shown in the Favored Enemy column of the Ranger Features table.'
  sha256: ba5390fde2c89961e9f34492f4f38fa31b0557cf0e07c98d4a0e478f8bd15b37
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Favored Enemy

<!-- SRD page 58 -->

You always have the Hunter’s Mark spell prepared.\
You can cast it twice without expending a spell slot,\
and you regain all expended uses of this ability\
when you finish a Long Rest.\
The number of times you can cast the spell with-\
out a spell slot increases when you reach certain\
Ranger levels, as shown in the Favored Enemy col-\
umn of the Ranger Features table.\
