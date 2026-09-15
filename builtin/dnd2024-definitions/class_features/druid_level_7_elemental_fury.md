---
splat_schema: 1
splat_id: 7007a918-dc31-52e9-99b1-d6aa4f2bf858
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_7_elemental_fury
splat_name: 'Level 7: Elemental Fury'
splat_origin:
  key: dnd2024:reference/pages/043
splat_sources:
- title: SRD 5.2.1
  locator: pp. 43
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/druid_level_7_elemental_fury
  parameters:
    choose: 1
    options:
      potent_spellcasting:
        spell_list: druid
        spell_level: 0
        damage_bonus:
          var: actor.abilities.wisdom.modifier
      primal_strike:
        trigger: attack.hits_creature
        attack_sources:
        - weapon
        - wild_shape.beast_attack
        limit:
          count: 1
          period: actor.turn
        extra_damage:
          dice:
            count: 1
            sides: 8
          type_choice:
          - cold
          - fire
          - lightning
          - thunder
          choice_time: attack.hit
    class_scope:
      class: druid
      minimum_level: 7
  implementation: dnd2024:class_features_druid_level_7_elemental_fury
splat_contracts:
- id: dnd2024:class_features_druid_level_7_elemental_fury
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
  - spell.damage
  - attack.hit
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
  - 43
  engine_gaps: &id001
  - dnd2024:class_features_druid_level_7_elemental_fury
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 527ea2e1ed8b814a640bceae5ab5dd7331932d2e7a284c865dba8d7622d49338
  parent_name: Druid
  line_span:
    first: 3591
    count: 11
splat_clauses:
- key: source_rules
  text: '# Level 7: Elemental Fury The might of the elements flows through you. You gain one of the following
    options of your choice. Potent Spellcasting. Add your Wisdom modifier to the damage you deal with
    any Druid cantrip. Primal Strike. Once on each of your turns when you hit a creature with an attack
    roll using a weapon or a Beast form’s attack in Wild Shape, you can cause the target to take an extra
    1d8 Cold, Fire, Lightning, or Thunder damage (choose when you hit).'
  sha256: 1be1e0aaa12cb45472ed77c0bf9ed483dafb0fbca72b3bb4971dd23c43127297
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 7: Elemental Fury

<!-- SRD page 43 -->

The might of the elements flows through you. You\
gain one of the following options of your choice.\
Potent Spellcasting. Add your Wisdom modifier\
to the damage you deal with any Druid cantrip.\
Primal Strike. Once on each of your turns when\
you hit a creature with an attack roll using a\
weapon or a Beast form’s attack in Wild Shape, you\
can cause the target to take an extra 1d8 Cold, Fire,\
Lightning, or Thunder damage (choose when you\
hit).\
