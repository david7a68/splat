---
splat_schema: 1
splat_id: 7f2acf2e-b970-5d4d-a706-297e8c2e55a6
splat_kind: spell
splat_key: dnd2024:spells/detect_evil_and_good
splat_name: Detect Evil and Good
splat_origin:
  key: dnd2024:reference/spell/detect_evil_and_good
splat_sources:
- title: SRD 5.2.1
  locator: pp. 123
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: divination
  classes:
  - cleric
  - paladin
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/detect_evil_and_good
  implementation: dnd2024:spell_detect_evil_and_good
  parameters:
    detection:
      radius_feet: 30
      origin: caster
      creature_types:
      - aberration
      - celestial
      - elemental
      - fey
      - fiend
      - undead
      outputs:
      - locations
      - hallow_presence
      - hallow_locations
    barriers:
    - materials:
      - stone
      - dirt
      - wood
      thickness_feet: 1
    - materials:
      - metal
      thickness_inches: 1
    - materials:
      - lead
      thickness: thin_sheet
splat_contracts:
- id: dnd2024:spell_detect_evil_and_good
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - detected_locations
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: fff520b6a1c16dec20552ab26e1687bfe7026f1c233e3614e8dcb4cb1a85d496
  source_path: builtin/srd-5.2.1/spell/detect_evil_and_good.md
  pages:
  - 123
  engine_gaps: &id001
  - dnd2024:spell_detect_evil_and_good
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: f60ff9fe4a1f9aae61cb851d4a5a16627ebd8a968744c8021c3d0ecb36b2e210
splat_clauses:
- key: source_rules
  text: '# Detect Evil and Good Level 1 Divination (Cleric, Paladin) Casting Time: Action Range: Self
    Components: V, S Duration: Concentration, up to 10 minutes For the duration, you sense the location
    of any Aberration, Celestial, Elemental, Fey, Fiend, or Undead within 30 feet of yourself. You also
    sense whether the Hallow spell is active there and, if so, where. The spell is blocked by 1 foot of
    stone, dirt, or wood; 1 inch of metal; or a thin sheet of lead.'
  sha256: 6f5270404fc8b29f2f9452c9dc56f0273326df98a2bb525aa12aa883df2b2cbc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Detect Evil and Good

<!-- SRD page 123 -->

Level 1 Divination (Cleric, Paladin)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 10 minutes\
For the duration, you sense the location of any Ab-\
erration, Celestial, Elemental, Fey, Fiend, or Undead\
within 30 feet of yourself. You also sense whether\
the Hallow spell is active there and, if so, where.\
The spell is blocked by 1 foot of stone, dirt, or\
wood; 1 inch of metal; or a thin sheet of lead.\

Original layout: [[../../srd-5.2.1/pages/123|page 123]].
