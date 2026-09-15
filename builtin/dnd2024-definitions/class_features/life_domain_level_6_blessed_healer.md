---
splat_schema: 1
splat_id: 233537fe-fc1f-54cb-8f1c-a0eb4d5f49ed
splat_kind: rule
splat_key: dnd2024:class_features/life_domain_level_6_blessed_healer
splat_name: 'Level 6: Blessed Healer'
splat_origin:
  key: dnd2024:reference/pages/040
splat_sources:
- title: SRD 5.2.1
  locator: pp. 40
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/life_domain_level_6_blessed_healer
  parameters:
    trigger:
      spell_cast: true
      expended_spell_slot: true
      restores_hit_points_to_other_creature: true
    heal_self:
      op: add
      args:
      - 2
      - stat: spell.slot_level
    timing: immediately_after_cast
  implementation: dnd2024:class_features_life_domain_level_6_blessed_healer
splat_contracts:
- id: dnd2024:class_features_life_domain_level_6_blessed_healer
  inputs:
    actor:
      type: participant
      required: true
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  - healing
  events:
  - spell.cast_completed
  capabilities:
  - healing
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 40
  engine_gaps: &id001
  - dnd2024:class_features_life_domain_level_6_blessed_healer
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: fc93b6655d865145b9f46f24a47d6afc1676e6108cb3a7bbbf784bd2ef7ded94
  parent_name: Life Domain
  line_span:
    first: 3327
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 6: Blessed Healer The healing spells you cast on others heal you as well. Immediately
    after you cast a spell with a spell slot that restores Hit Points to one or more creatures other than
    yourself, you regain Hit Points equal to 2 plus the spell slot’s level.'
  sha256: d795984a616b12ae81225c8fc5ceff1a119f5df43a87df383b4355fe615c6f8d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 6: Blessed Healer

<!-- SRD page 40 -->

The healing spells you cast on others heal you as\
well. Immediately after you cast a spell with a spell\
slot that restores Hit Points to one or more crea-\
tures other than yourself, you regain Hit Points\
equal to 2 plus the spell slot’s level.\
