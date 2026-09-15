---
splat_schema: 1
splat_id: 95d7bee8-d399-51db-b328-2eb6cdc1d1c7
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_1_fighting_style
splat_name: 'Level 1: Fighting Style'
splat_origin:
  key: dnd2024:reference/pages/047
splat_sources:
- title: SRD 5.2.1
  locator: pp. 47
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fighter_level_1_fighting_style
  parameters:
    feat_choice:
      count: 1
      category: fighting_style
    replace:
      event: fighter.level_gained
      category: fighting_style
      maximum: 1
    class_scope:
      class: fighter
      minimum_level: 1
  implementation: dnd2024:class_features_fighter_level_1_fighting_style
splat_contracts:
- id: dnd2024:class_features_fighter_level_1_fighting_style
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - feat_grants
  events:
  - character.feature_acquired
  - character.level_gained
  capabilities:
  - character_features
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 47
  engine_gaps: &id001
  - dnd2024:class_features_fighter_level_1_fighting_style
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 2981b00e4ff7552eedcd5a8cdae5eed3775db99ce110864aca1782476932ace3
  parent_name: Fighter
  line_span:
    first: 3934
    count: 29
splat_clauses:
- key: source_rules
  text: '# Level 1: Fighting Style You have honed your martial prowess and gain a Fighting Style feat
    of your choice (see “Feats”). Defense is recommended. Whenever you gain a Fighter level, you can replace
    the feat you chose with a different Fighting Style feat. Second Weapon Wind Mastery 2 3 2 3 2 3 3
    4 3 4 3 4 3 4 3 4 3 4 4 5 4 5 4 5 4 5 4 5 4 5 4 6 4 6 4 6 4 6 4 6'
  sha256: d9bd0a678354f3e9e9ff0e8c27c2128f0a318e19fa86532b642967933fa7dcf0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Fighting Style

<!-- SRD page 47 -->

You have honed your martial prowess and gain a\
Fighting Style feat of your choice (see “Feats”). De-\
fense is recommended.\
Whenever you gain a Fighter level, you can replace\
the feat you chose with a different Fighting Style\
feat.\
Second Weapon\
Wind Mastery\
2 3\
2 3\
2 3\
3 4\
3 4\
3 4\
3 4\
3 4\
3 4\
4 5\
4 5\
4 5\
4 5\
4 5\
4 5\
4 6\
4 6\
4 6\
4 6\
4 6\
