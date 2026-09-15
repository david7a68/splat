---
splat_schema: 1
splat_id: 35365b15-7c04-50ae-ab40-9ce9e654b024
splat_kind: rule_group
splat_key: dnd2024:subclasses/fighter_fighter_subclass_champion
splat_name: 'Fighter Subclass: Champion'
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: subclass
  members:
  - key: dnd2024:class_features/champion_level_10_heroic_warrior
  - key: dnd2024:class_features/champion_level_15_superior_critical
  - key: dnd2024:class_features/champion_level_18_survivor
  - key: dnd2024:class_features/champion_level_3_improved_critical
  - key: dnd2024:class_features/champion_level_3_remarkable_athlete
  - key: dnd2024:class_features/champion_level_7_additional_fighting_style
splat_rule:
  mechanic: dnd2024:subclasses/fighter_fighter_subclass_champion
  implementation: dnd2024:subclasses_fighter_fighter_subclass_champion
  parameters:
    class: fighter
    subclass: champion
    feature_grants:
    - feature:
        key: dnd2024:class_features/champion_level_10_heroic_warrior
      minimum_class_level: 10
    - feature:
        key: dnd2024:class_features/champion_level_15_superior_critical
      minimum_class_level: 15
    - feature:
        key: dnd2024:class_features/champion_level_18_survivor
      minimum_class_level: 18
    - feature:
        key: dnd2024:class_features/champion_level_3_improved_critical
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/champion_level_3_remarkable_athlete
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/champion_level_7_additional_fighting_style
      minimum_class_level: 7
splat_contracts:
- id: dnd2024:subclasses_fighter_fighter_subclass_champion
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
  - 49
  engine_gaps: &id001
  - dnd2024:subclasses_fighter_fighter_subclass_champion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: ff7a7d40798961781233dad7eaaeeaf80f85a947ccd77163b35529bd7b67fab7
  parent_name: Fighter
  line_span:
    first: 4048
    count: 9
splat_clauses:
- key: source_rules
  text: '# Fighter Subclass: Champion Pursue Physical Excellence in Combat A Champion focuses on the development
    of martial prowess in a relentless pursuit of victory. Champions combine rigorous training with physical
    excellence to deal devastating blows, withstand peril, and garner glory. Whether in athletic contests
    or bloody battle, Champions strive for the crown of the victor.'
  sha256: 9ebee76fbc0fc34f96ee1b11e7ac2350d993ecedbedfa1ed6ba331b7cd445aeb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fighter Subclass: Champion

<!-- SRD page 49 -->

Pursue Physical Excellence in Combat\
A Champion focuses on the development of martial\
prowess in a relentless pursuit of victory. Champi-\
ons combine rigorous training with physical excel-\
lence to deal devastating blows, withstand peril,\
and garner glory. Whether in athletic contests or\
bloody battle, Champions strive for the crown of the\
victor.\
