---
splat_schema: 1
splat_id: 732c6263-2133-5754-a5da-900b6d33f302
splat_kind: spell
splat_key: dnd2024:spells/color_spray
splat_name: Color Spray
splat_origin:
  key: dnd2024:reference/spell/color_spray
splat_sources:
- title: SRD 5.2.1
  locator: pp. 116
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: illusion
  classes:
  - bard
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: a pinch of colorful sand
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/color_spray
  implementation: dnd2024:spell_color_spray
  parameters:
    spell_lifetime:
      concentration: false
      instantaneous: true
    area:
      shape: cone
      length_feet: 15
      origin: caster
    targets:
      kind: creature
      selection: all_in_area
    save:
      ability: constitution
      dc: spell_save_dc
      success: unaffected
    failure:
      condition: blinded
      until:
        event: turn.end
        subject: caster
        occurrence: next
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_color_spray
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
    origin:
      type: location
      required: true
    save_dc:
      type: integer
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  events:
  - spell.cast_completed
  - turn.end
  capabilities:
  - spellcasting
  - effects
  - areas
  - tests
splat_conversion:
  status: specified
  category: spell
  source_sha256: 3101fcbf046c6050f4d78e0f0c96bac177885b4ef8b4c9eb9f14ab03149ed0bf
  source_path: builtin/srd-5.2.1/spell/color_spray.md
  pages:
  - 116
  engine_gaps: &id001
  - dnd2024:spell_color_spray
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 91bf1689a1592ea183f9c61ac0992dae3a7a2b82bdc5fe82b0bb13b7a2f21df0
splat_clauses:
- key: source_rules
  text: '# Color Spray Level 1 Illusion (Bard, Sorcerer, Wizard) Casting Time: Action Range: Self Components:
    V, S, M (a pinch of colorful sand) Duration: Instantaneous You launch a dazzling array of flashing,
    colorful light. Each creature in a 15-foot Cone originating from you must succeed on a Constitution
    saving throw or have the Blinded condition until the end of your next turn.'
  sha256: 9dfdcf471ca5637bbfe3e56968b2382fe13a13c5c6825ec40b3ea7f19a390d32
  status: contract_specified
splat_requires_mechanics: *id001
---

# Color Spray

<!-- SRD page 116 -->

Level 1 Illusion (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a pinch of colorful sand)\
Duration: Instantaneous\
You launch a dazzling array of flashing, colorful\
light. Each creature in a 15-foot Cone originating\
from you must succeed on a Constitution saving\
throw or have the Blinded condition until the end of\
your next turn.\

Original layout: [[../../srd-5.2.1/pages/116|page 116]].
