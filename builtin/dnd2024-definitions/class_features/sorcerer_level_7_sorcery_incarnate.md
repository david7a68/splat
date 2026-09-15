---
splat_schema: 1
splat_id: 207aa813-5898-5064-9c61-a2b394a45820
splat_kind: rule
splat_key: dnd2024:class_features/sorcerer_level_7_sorcery_incarnate
splat_name: 'Level 7: Sorcery Incarnate'
splat_origin:
  key: dnd2024:reference/pages/066
splat_sources:
- title: SRD 5.2.1
  locator: pp. 66
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/sorcerer_level_7_sorcery_incarnate
  parameters:
    innate_sorcery_alternative_cost:
      requires_remaining_uses: 0
      sorcery_points: 2
      action: same_activation_bonus_action
    while_innate_sorcery_active:
      maximum_metamagic_options_per_cast: 2
  implementation: dnd2024:class_features_sorcerer_level_7_sorcery_incarnate
splat_contracts:
- id: dnd2024:class_features_sorcerer_level_7_sorcery_incarnate
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  - resource_changes
  events:
  - action.bonus
  - spell.cast_pending
  capabilities:
  - metamagic
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 66
  engine_gaps: &id001
  - dnd2024:class_features_sorcerer_level_7_sorcery_incarnate
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: e952865225dfc9866e0799a564d626ba4a22a7466936681ea1cba552af23e100
  parent_name: Sorcerer
  line_span:
    first: 5613
    count: 7
splat_clauses:
- key: source_rules
  text: '# Level 7: Sorcery Incarnate If you have no uses of Innate Sorcery left, you can use it if you
    spend 2 Sorcery Points when you take the Bonus Action to activate it. In addition, while your Innate
    Sorcery feature is active, you can use up to two of your Metamagic options on each spell you cast.'
  sha256: 8aae1bac9ba51932bf559e15d789d20947b90546064248e84a52ab26b592978a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 7: Sorcery Incarnate

<!-- SRD page 66 -->

If you have no uses of Innate Sorcery left, you can\
use it if you spend 2 Sorcery Points when you take\
the Bonus Action to activate it.\
In addition, while your Innate Sorcery feature is\
active, you can use up to two of your Metamagic op-\
tions on each spell you cast.\
