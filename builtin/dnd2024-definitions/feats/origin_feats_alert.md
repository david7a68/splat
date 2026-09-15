---
splat_schema: 1
splat_id: 42bc1a6d-829d-5d92-8f01-3d2fd68d5155
splat_kind: rule
splat_key: dnd2024:feats/origin_feats_alert
splat_name: Alert
splat_origin:
  key: dnd2024:reference/pages/087
splat_sources:
- title: SRD 5.2.1
  locator: pp. 87
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:feats/origin_feats_alert
  parameters:
    feat_category: origin
    initiative_bonus:
      optional: true
      amount:
        var: actor.proficiency_bonus
    initiative_swap:
      timing: immediately_after_actor_initiative_roll
      optional: true
      target:
        willing: true
        ally: true
        same_combat: true
        incapacitated: false
      requires:
        actor_incapacitated: false
      swap_values:
      - actor.initiative
      - target.initiative
  implementation: dnd2024:feats_origin_feats_alert
splat_contracts:
- id: dnd2024:feats_origin_feats_alert
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
  capabilities:
  - character_features
  - proficiencies
  - initiative
splat_conversion:
  status: specified
  category: feat
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 87
  engine_gaps: &id001
  - dnd2024:feats_origin_feats_alert
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 2c36c92d011eda58257ba3a237a2102a24362553aa5cecd75b19ebda61a9141b
  parent_name: Origin Feats
  line_span:
    first: 7477
    count: 10
splat_clauses:
- key: source_rules
  text: '# Alert Origin Feat You gain the following benefits. Initiative Proficiency. When you roll Initiative,
    you can add your Proficiency Bonus to the roll. Initiative Swap. Immediately after you roll Initiative,
    you can swap your Initiative with the Initiative of one willing ally in the same combat. You can’t
    make this swap if you or the ally has the Incapacitated condition.'
  sha256: 84c07f37fe9c9fbdc70090ca314c5a390fe88041a183a95d66ebcd248bf83782
  status: contract_specified
splat_requires_mechanics: *id001
---

# Alert

<!-- SRD page 87 -->

Origin Feat\
You gain the following benefits.\
Initiative Proficiency. When you roll Initiative,\
you can add your Proficiency Bonus to the roll.\
Initiative Swap. Immediately after you roll Initia-\
tive, you can swap your Initiative with the Initiative\
of one willing ally in the same combat. You can’t\
make this swap if you or the ally has the Incapaci-\
tated condition.\
