---
splat_schema: 1
splat_id: 8578a387-b95e-5caf-a050-54fdf7e0c833
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_2_fighting_style
splat_name: 'Level 2: Fighting Style'
splat_origin:
  key: dnd2024:reference/pages/054
splat_sources:
- title: SRD 5.2.1
  locator: pp. 54
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/paladin_level_2_fighting_style
  parameters:
    choose: 1
    options:
      fighting_style:
        feat_category: fighting_style
        count: 1
      blessed_warrior:
        cantrips:
          count: 2
          list: cleric
        counts_as_class: paladin
        ability: charisma
        replace:
          event: paladin.level_gained
          maximum: 1
          list: cleric
          spell_level: 0
    class_scope:
      class: paladin
      minimum_level: 2
  implementation: dnd2024:class_features_paladin_level_2_fighting_style
splat_contracts:
- id: dnd2024:class_features_paladin_level_2_fighting_style
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - feat_or_spell_grants
  events:
  - character.feature_acquired
  - character.level_gained
  capabilities:
  - character_features
  - spellcasting
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 54
  engine_gaps: &id001
  - dnd2024:class_features_paladin_level_2_fighting_style
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: c96dbebccb02f3b7d8c0e67942700f29f4b832b4f2c6f66e120b80235180cdce
  parent_name: Paladin
  line_span:
    first: 4540
    count: 12
splat_clauses:
- key: source_rules
  text: '# Level 2: Fighting Style You gain a Fighting Style feat of your choice (see “Feats” for feats).
    Instead of choosing one of those feats, you can choose the option below. Blessed Warrior. You learn
    two Cleric cantrips of your choice (see the Cleric class’s section for a list of Cleric spells). Guidance
    and Sacred Flame are recommended. The chosen cantrips count as Paladin spells for you, and Charisma
    is your spellcasting ability for them. Whenever you gain a Paladin level, you can replace one of these
    cantrips with another Cleric cantrip.'
  sha256: 6155bea7b351bf3795ba2dd6ba736fcc1c241a7f68235b53038e7ae7b1701720
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Fighting Style

<!-- SRD page 54 -->

You gain a Fighting Style feat of your choice (see\
“Feats” for feats). Instead of choosing one of those\
feats, you can choose the option below.\
Blessed Warrior. You learn two Cleric cantrips of\
your choice (see the Cleric class’s section for a list\
of Cleric spells). Guidance and Sacred Flame are rec-\
ommended. The chosen cantrips count as Paladin\
spells for you, and Charisma is your spellcasting\
ability for them. Whenever you gain a Paladin level,\
you can replace one of these cantrips with another\
Cleric cantrip.\
