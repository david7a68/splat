---
splat_schema: 1
splat_id: d3008830-4514-5e18-a003-b735f2ac7932
splat_kind: rule_group
splat_key: dnd2024:subclasses/barbarian_path_of_the_berserker
splat_name: Path of the Berserker
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: subclass
splat_rule:
  mechanic: dnd2024:subclasses/barbarian_path_of_the_berserker
  implementation: dnd2024:subclasses_barbarian_path_of_the_berserker
  parameters:
    class: barbarian
    minimum_class_level: 3
    feature_grants:
    - class_level: 10
      definition:
        key: dnd2024:class_features/path_of_the_berserker_level_10_retaliation
    - class_level: 14
      definition:
        key: dnd2024:class_features/path_of_the_berserker_level_14_intimidating_presence
    - class_level: 3
      definition:
        key: dnd2024:class_features/path_of_the_berserker_level_3_frenzy
    - class_level: 6
      definition:
        key: dnd2024:class_features/path_of_the_berserker_level_6_mindless_rage
    grant_eligible_features:
      'on':
      - subclass.selected
      - character.level_gained
      comparison: class_level_at_least_threshold
      persistent: true
splat_contracts:
- id: dnd2024:subclasses_barbarian_path_of_the_berserker
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - feature_grants
  events:
  - subclass.selected
  - character.level_gained
  capabilities:
  - character_features
splat_conversion:
  status: specified
  category: subclass
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 30
  engine_gaps: &id001
  - dnd2024:subclasses_barbarian_path_of_the_berserker
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 73bdd68e9ddc215c871b6144c1bc59ee8f031907e2376ff787e2103e59a11721
  parent_name: Barbarian
  line_span:
    first: 2452
    count: 7
splat_clauses:
- key: source_rules
  text: '# Path of the Berserker Channel Rage into Violent Fury Barbarians who walk the Path of the Berserker
    direct their Rage primarily toward violence. Their path is one of untrammeled fury, and they thrill
    in the chaos of battle as they allow their Rage to seize and empower them.'
  sha256: fcc88940d212a97cb6be61b00f8dc485584fc901520fd70920ff242be831a4bb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Path of the Berserker

<!-- SRD page 30 -->

Channel Rage into Violent Fury\
Barbarians who walk the Path of the Berserker\
direct their Rage primarily toward violence. Their\
path is one of untrammeled fury, and they thrill in\
the chaos of battle as they allow their Rage to seize\
and empower them.\
