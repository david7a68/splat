---
splat_schema: 1
splat_id: 09a6667d-e827-5ada-98db-6fea9ecb97de
splat_kind: rule_group
splat_key: dnd2024:subclasses/ranger_ranger_subclass_hunter
splat_name: 'Ranger Subclass: Hunter'
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: subclass
splat_rule:
  mechanic: dnd2024:subclasses/ranger_ranger_subclass_hunter
  implementation: dnd2024:subclasses_ranger_ranger_subclass_hunter
  parameters:
    class: ranger
    minimum_class_level: 3
    feature_grants:
    - class_level: 11
      definition:
        key: dnd2024:class_features/hunter_level_11_superior_hunter_s_prey
    - class_level: 15
      definition:
        key: dnd2024:class_features/hunter_level_15_superior_hunter_s_defense
    - class_level: 3
      definition:
        key: dnd2024:class_features/hunter_level_3_hunter_s_lore
    - class_level: 3
      definition:
        key: dnd2024:class_features/hunter_level_3_hunter_s_prey
    - class_level: 7
      definition:
        key: dnd2024:class_features/hunter_level_7_defensive_tactics
    grant_eligible_features:
      'on':
      - subclass.selected
      - character.level_gained
      comparison: class_level_at_least_threshold
      persistent: true
splat_contracts:
- id: dnd2024:subclasses_ranger_ranger_subclass_hunter
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
  - 61
  engine_gaps: &id001
  - dnd2024:subclasses_ranger_ranger_subclass_hunter
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: a474aedce580e6d3553f67bf1de243dc341447cff9036e18a605e0b95445a939
  parent_name: Ranger
  line_span:
    first: 5077
    count: 6
splat_clauses:
- key: source_rules
  text: '# Ranger Subclass: Hunter Protect Nature and People from Destruction You stalk prey in the wilds
    and elsewhere, using your abilities as a Hunter to protect nature and people everywhere from forces
    that would destroy them.'
  sha256: 89e12a6b5265317822b4fdaea0e581d08cf9a488f9152515676076099b64102e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ranger Subclass: Hunter

<!-- SRD page 61 -->

Protect Nature and People from Destruction\
You stalk prey in the wilds and elsewhere, using\
your abilities as a Hunter to protect nature and\
people everywhere from forces that would destroy\
them.\
