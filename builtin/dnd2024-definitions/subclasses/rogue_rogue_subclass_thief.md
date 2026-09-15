---
splat_schema: 1
splat_id: a00d37c5-3843-50df-8b89-64582a44a633
splat_kind: rule_group
splat_key: dnd2024:subclasses/rogue_rogue_subclass_thief
splat_name: 'Rogue Subclass: Thief'
splat_origin:
  key: dnd2024:reference/pages/064
splat_sources:
- title: SRD 5.2.1
  locator: pp. 64
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: subclass
  members:
  - key: dnd2024:class_features/thief_level_13_use_magic_device
  - key: dnd2024:class_features/thief_level_17_thief_s_reflexes
  - key: dnd2024:class_features/thief_level_3_fast_hands
  - key: dnd2024:class_features/thief_level_3_second_story_work
  - key: dnd2024:class_features/thief_level_9_supreme_sneak
splat_rule:
  mechanic: dnd2024:subclasses/rogue_rogue_subclass_thief
  implementation: dnd2024:subclasses_rogue_rogue_subclass_thief
  parameters:
    class: rogue
    subclass: thief
    feature_grants:
    - feature:
        key: dnd2024:class_features/thief_level_13_use_magic_device
      minimum_class_level: 13
    - feature:
        key: dnd2024:class_features/thief_level_17_thief_s_reflexes
      minimum_class_level: 17
    - feature:
        key: dnd2024:class_features/thief_level_3_fast_hands
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/thief_level_3_second_story_work
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/thief_level_9_supreme_sneak
      minimum_class_level: 9
splat_contracts:
- id: dnd2024:subclasses_rogue_rogue_subclass_thief
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - character.subclass_selected
  - character.class_level_gained
  capabilities:
  - class_progression
splat_conversion:
  status: specified
  category: subclass
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 64
  engine_gaps: &id001
  - dnd2024:subclasses_rogue_rogue_subclass_thief
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 6dbc41a923b97a00f8c8c1c371d1c6d6247228dc169cfe16affb84de2ac1ba76
  parent_name: Rogue
  line_span:
    first: 5351
    count: 7
splat_clauses:
- key: source_rules
  text: '# Rogue Subclass: Thief Hunt for Treasure as a Classic Adventurer A mix of burglar, treasure
    hunter, and explorer, you are the epitome of an adventurer. In addition to improving your agility
    and stealth, you gain abilities useful for delving into ruins and getting maximum benefit from the
    magic items you find there.'
  sha256: 944d7b5d7b03d0f759614ed12b9536c1ec603114ccf3fed46d74c95c8ff045bb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rogue Subclass: Thief

<!-- SRD page 64 -->

Hunt for Treasure as a Classic Adventurer\
A mix of burglar, treasure hunter, and explorer, you\
are the epitome of an adventurer. In addition to im-\
proving your agility and stealth, you gain abilities\
useful for delving into ruins and getting maximum\
benefit from the magic items you find there.\
