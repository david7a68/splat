---
splat_schema: 1
splat_id: d2c2ad7c-5657-584a-839d-b729a774bf16
splat_kind: rule_group
splat_key: dnd2024:subclasses/druid_circle_of_the_land
splat_name: Circle of the Land
splat_origin:
  key: dnd2024:reference/pages/046
splat_sources:
- title: SRD 5.2.1
  locator: pp. 46
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: subclass
  members:
  - key: dnd2024:class_features/circle_of_the_land_level_10_nature_s_ward
  - key: dnd2024:class_features/circle_of_the_land_level_14_nature_s_sanctuary
  - key: dnd2024:class_features/circle_of_the_land_level_3_circle_of_the_land_spells
  - key: dnd2024:class_features/circle_of_the_land_level_3_land_s_aid
  - key: dnd2024:class_features/circle_of_the_land_level_6_natural_recovery
splat_rule:
  mechanic: dnd2024:subclasses/druid_circle_of_the_land
  implementation: dnd2024:subclasses_druid_circle_of_the_land
  parameters:
    class: druid
    subclass: circle_of_the_land
    feature_grants:
    - feature:
        key: dnd2024:class_features/circle_of_the_land_level_10_nature_s_ward
      minimum_class_level: 10
    - feature:
        key: dnd2024:class_features/circle_of_the_land_level_14_nature_s_sanctuary
      minimum_class_level: 14
    - feature:
        key: dnd2024:class_features/circle_of_the_land_level_3_circle_of_the_land_spells
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/circle_of_the_land_level_3_land_s_aid
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/circle_of_the_land_level_6_natural_recovery
      minimum_class_level: 6
splat_contracts:
- id: dnd2024:subclasses_druid_circle_of_the_land
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
  - 46
  engine_gaps: &id001
  - dnd2024:subclasses_druid_circle_of_the_land
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: bf965318908021918afe7ee1117c387cef921005802ef59f380ca78c2ccdbce5
  parent_name: Druid
  line_span:
    first: 3788
    count: 8
splat_clauses:
- key: source_rules
  text: '# Circle of the Land Celebrate Connection to the Natural World The Circle of the Land comprises
    mystics and sages who safeguard ancient knowledge and rites. These Druids meet within sacred circles
    of trees or standing stones to whisper primal secrets in Druidic. The circle’s wisest members preside
    as the chief priests of their communities.'
  sha256: 6ade6f9cba6c87731dd499321f5419ddf22734eacad2912357d31073e928ef73
  status: contract_specified
splat_requires_mechanics: *id001
---

# Circle of the Land

<!-- SRD page 46 -->

Celebrate Connection to the Natural World\
The Circle of the Land comprises mystics and sages\
who safeguard ancient knowledge and rites. These\
Druids meet within sacred circles of trees or stand-\
ing stones to whisper primal secrets in Druidic. The\
circle’s wisest members preside as the chief priests\
of their communities.\
