---
splat_schema: 1
splat_id: b62a1a1e-7eec-54a5-83f8-3de063fb1e61
splat_kind: spell
splat_key: dnd2024:spells/see_invisibility
splat_name: See Invisibility
splat_origin:
  key: dnd2024:reference/spell/see_invisibility
splat_sources:
- title: SRD 5.2.1
  locator: pp. 160
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: divination
  classes:
  - bard
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: a pinch of talc
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/see_invisibility
  implementation: dnd2024:spell_see_invisibility
  parameters:
    target:
      kind: self
      count: 1
      visible_to_caster: false
    vision:
      invisible_creatures_and_objects: as_visible
      ethereal_plane: true
      ethereal_appearance: ghostly
splat_contracts:
- id: dnd2024:spell_see_invisibility
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
  - vision_effect
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - perception
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: 6dad19f23499d21d328c924379dce94b10fc2492e8f1fdf626bd1f897aae2c93
  source_path: builtin/srd-5.2.1/spell/see_invisibility.md
  pages:
  - 160
  engine_gaps: &id001
  - dnd2024:spell_see_invisibility
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: afaefa6997e8a82274e7f8424fef8d406a4241abdad915e6635b7d10d093e1da
splat_clauses:
- key: source_rules
  text: '# See Invisibility Level 2 Divination (Bard, Sorcerer, Wizard) Casting Time: Action Range: Self
    Components: V, S, M (a pinch of talc) Duration: 1 hour For the duration, you see creatures and objects
    that have the Invisible condition as if they were visible, and you can see into the Ethereal Plane.
    Creatures and objects there appear ghostly.'
  sha256: a5c727e9ad8b7f939173493282e61ac670388905a9b717eeded4b41d123a014c
  status: contract_specified
splat_requires_mechanics: *id001
---

# See Invisibility

<!-- SRD page 160 -->

Level 2 Divination (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a pinch of talc)\
Duration: 1 hour\
For the duration, you see creatures and objects that\
have the Invisible condition as if they were visible,\
and you can see into the Ethereal Plane. Creatures\
and objects there appear ghostly.\

Original layout: [[../../srd-5.2.1/pages/160|page 160]].
