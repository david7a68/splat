---
splat_schema: 1
splat_id: 82e6f19c-c1c8-59a2-821c-3cfc697e68a7
splat_kind: rule
splat_key: dnd2024:class_features/life_domain_level_17_supreme_healing
splat_name: 'Level 17: Supreme Healing'
splat_origin:
  key: dnd2024:reference/pages/040
splat_sources:
- title: SRD 5.2.1
  locator: pp. 40
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/life_domain_level_17_supreme_healing
  parameters:
    trigger: healing.dice_pending
    sources:
    - actor.spell
    - actor.channel_divinity
    replace_each_die: maximum_face
    class_scope:
      class: cleric
      minimum_level: 17
  implementation: dnd2024:class_features_life_domain_level_17_supreme_healing
splat_contracts:
- id: dnd2024:class_features_life_domain_level_17_supreme_healing
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - healing.dice_pending
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
  - dnd2024:class_features_life_domain_level_17_supreme_healing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 8b4c9f4a9facf4ee364a4ae4a4ac4c97e4594ca5d186663a788fab256521a5de
  parent_name: Life Domain
  line_span:
    first: 3333
    count: 7
splat_clauses:
- key: source_rules
  text: '# Level 17: Supreme Healing When you would normally roll one or more dice to restore Hit Points
    to a creature with a spell or Channel Divinity, don’t roll those dice for the healing; instead use
    the highest number possible for each die. For example, instead of restoring 2d6 Hit Points to a creature
    with a spell, you restore 12.'
  sha256: 93a1e2f467fbc1ff3bbb4c5d35a7dd54d94145836f66700201a9741eb81a8e56
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 17: Supreme Healing

<!-- SRD page 40 -->

When you would normally roll one or more dice to\
restore Hit Points to a creature with a spell or Chan-\
nel Divinity, don’t roll those dice for the healing; in-\
stead use the highest number possible for each die.\
For example, instead of restoring 2d6 Hit Points to a\
creature with a spell, you restore 12.\
