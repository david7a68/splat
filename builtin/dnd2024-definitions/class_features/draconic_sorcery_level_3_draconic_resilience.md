---
splat_schema: 1
splat_id: 86d68e1c-4938-51ba-b368-a497f0c1432c
splat_kind: rule
splat_key: dnd2024:class_features/draconic_sorcery_level_3_draconic_resilience
splat_name: 'Level 3: Draconic Resilience'
splat_origin:
  key: dnd2024:reference/pages/069
splat_sources:
- title: SRD 5.2.1
  locator: pp. 69
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/draconic_sorcery_level_3_draconic_resilience
  parameters:
    hit_point_maximum_bonus:
      at_sorcerer_level_3: 3
      per_subsequent_sorcerer_level: 1
    base_ac:
      when_armor_worn: false
      value:
        op: add
        args:
        - 10
        - stat: abilities.dexterity.modifier
        - stat: abilities.charisma.modifier
  implementation: dnd2024:class_features_draconic_sorcery_level_3_draconic_resilience
splat_contracts:
- id: dnd2024:class_features_draconic_sorcery_level_3_draconic_resilience
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - character.class_level_gained
  - armor_class.calculated
  capabilities:
  - hit_points
  - armor_class
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 69
  engine_gaps: &id001
  - dnd2024:class_features_draconic_sorcery_level_3_draconic_resilience
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: d1a7467d9e6d397f56d03ac1d3df8cd7b285ce6c6918af09f8ca7fd37be2e7a8
  parent_name: Draconic Sorcery
  line_span:
    first: 5886
    count: 9
splat_clauses:
- key: source_rules
  text: '# Level 3: Draconic Resilience The magic in your body manifests physical traits of your draconic
    gift. Your Hit Point maximum increases by 3, and it increases by 1 whenever you gain another Sorcerer
    level. Parts of you are also covered by dragon-like scales. While you aren’t wearing armor, your base
    Armor Class equals 10 plus your Dexterity and Charisma modifiers.'
  sha256: 82fed63bdc8ab068f0546873736908d77d747a0db532c570363f52fe70e7e099
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Draconic Resilience

<!-- SRD page 69 -->

The magic in your body manifests physical traits\
of your draconic gift. Your Hit Point maximum in-\
creases by 3, and it increases by 1 whenever you\
gain another Sorcerer level.\
Parts of you are also covered by dragon-like\
scales. While you aren’t wearing armor, your base\
Armor Class equals 10 plus your Dexterity and Cha-\
risma modifiers.\
