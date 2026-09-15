---
splat_schema: 1
splat_id: 04c58699-471e-574d-a658-a0587f19a46e
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_2_fighting_style
splat_name: 'Level 2: Fighting Style'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/ranger_level_2_fighting_style
  parameters:
    choice:
      count: 1
      options:
      - id: fighting_style
        feat_choice:
          category: fighting_style
          count: 1
      - id: druidic_warrior
        cantrip_choice:
          list: druid
          count: 2
        count_as_class_spells: ranger
        ability: wisdom
        replace:
          event: character.ranger_level_gained
          maximum: 1
          list: druid
          spell_level: 0
  implementation: dnd2024:class_features_ranger_level_2_fighting_style
splat_contracts:
- id: dnd2024:class_features_ranger_level_2_fighting_style
  inputs:
    actor:
      type: participant
      required: true
    selections:
      required: false
      type: choice
  outputs:
  - feature_state
  - choices
  - spell_grants
  events:
  - character.feature_gained
  - character.class_level_gained
  capabilities:
  - character_choices
  - feats
  - spellcasting
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 59
  engine_gaps: &id001
  - dnd2024:class_features_ranger_level_2_fighting_style
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 900fb9087f1ae936dd5c19f29699fb5fabe8a16149b7adc7a377ba58de340ca7
  parent_name: Ranger
  line_span:
    first: 4940
    count: 12
splat_clauses:
- key: source_rules
  text: '# Level 2: Fighting Style You gain a Fighting Style feat of your choice (see “Feats”). Instead
    of choosing one of those feats, you can choose the option below. Druidic Warrior. You learn two Druid
    cantrips of your choice (see the Druid class’s section for a list of Druid spells). Guidance and Starry
    Wisp are recommended. The chosen cantrips count as Ranger spells for you, and Wisdom is your spellcasting
    ability for them. Whenever you gain a Ranger level, you can replace one of these cantrips with another
    Druid cantrip.'
  sha256: 80218cf1b9a7c9591097bf3de2286588299baf899c25c947108cdff630ed834c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Fighting Style

<!-- SRD page 59 -->

You gain a Fighting Style feat of your choice (see\
“Feats”). Instead of choosing one of those feats, you\
can choose the option below.\
Druidic Warrior. You learn two Druid cantrips of\
your choice (see the Druid class’s section for a list of\
Druid spells). Guidance and Starry Wisp are recom-\
mended. The chosen cantrips count as Ranger spells\
for you, and Wisdom is your spellcasting ability for\
them. Whenever you gain a Ranger level, you can\
replace one of these cantrips with another Druid\
cantrip.\
