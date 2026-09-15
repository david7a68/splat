---
splat_schema: 1
splat_id: 160877bd-5c87-5b3a-b2f2-867c566a400e
splat_kind: spell
splat_key: dnd2024:spells/fabricate
splat_name: Fabricate
splat_origin:
  key: dnd2024:reference/spell/fabricate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 129
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: transmutation
  classes:
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: 10 minutes
  ritual: false
  range:
    feet: 120
splat_rule:
  mechanic: dnd2024:spells/fabricate
  implementation: dnd2024:spell_fabricate
  parameters:
    spell_lifetime:
      concentration: false
      instantaneous: true
    inputs:
      kind: raw_materials
      visible: true
      range_feet: 120
      sufficient_quantity_required: true
    output:
      kind: object
      material: same_as_inputs
      quality: based_on_input_quality
      maximum_size: large
      geometry_choice:
      - cube_side_feet: 10
        count: 1
      - cube_side_feet: 5
        count: 8
        connected: true
    mineral_exception:
      materials:
      - metal
      - stone
      - other_mineral
      maximum_size: medium
      maximum_cube_side_feet: 5
    forbidden_output:
    - creature
    - magic_item
    skilled_craft:
      examples:
      - weapon
      - armor
      requires_proficiency: corresponding_artisans_tools
    supplied_facts:
      material_sufficiency: boolean
      input_quality: choice
      requires_high_skill: boolean
      corresponding_tools: definition
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_fabricate
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    materials:
      type: object_list
      required: true
    product_design:
      type: choice
      required: true
    material_facts:
      type: resolution
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - objects
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - world_objects
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: 65ad08770764f2573dde932711900515b21bb8f8706c49bceaa1af0cbcb49ceb
  source_path: builtin/srd-5.2.1/spell/fabricate.md
  pages:
  - 129
  engine_gaps: &id001
  - dnd2024:spell_fabricate
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 5c38f37bb21e8dd2540ea2e4eb27cf53a48271e0dbefec0cb4c68726bd194187
splat_clauses:
- key: source_rules
  text: '# Fabricate Level 4 Transmutation (Wizard) Casting Time: 10 minutes Range: 120 feet Components:
    V, S Duration: Instantaneous You convert raw materials into products of the same material. For example,
    you can fabricate a wooden bridge from a clump of trees, a rope from a patch of hemp, or clothes from
    flax or wool. Choose raw materials that you can see within range. You can fabricate a Large or smaller
    object (contained within a 10-foot Cube or eight connected 5-foot Cubes) given a sufficient quantity
    of material. If you’re working with metal, stone, or another mineral substance, however, the fabricated
    object can be no larger than Medium (contained within a 5-foot Cube). The quality of any fabricated
    objects is based on the quality of the raw materials. Creatures and magic items can’t be created by
    this spell. You also can’t use it to create items that require a high degree of skill—such as weapons
    and armor—unless you have proficiency with the type of Artisan’s Tools used to craft such objects.'
  sha256: acb968dbab828039480a54748426dbac4429ed21ac77a54c7f3a3a3ef7e850db
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fabricate

<!-- SRD page 129 -->

Level 4 Transmutation (Wizard)\
Casting Time: 10 minutes\
Range: 120 feet\
Components: V, S\
Duration: Instantaneous\
You convert raw materials into products of the same\
material. For example, you can fabricate a wooden\
bridge from a clump of trees, a rope from a patch of\
hemp, or clothes from flax or wool.\
Choose raw materials that you can see within\
range. You can fabricate a Large or smaller object\
(contained within a 10-foot Cube or eight connected\
5-foot Cubes) given a sufficient quantity of mate-\
rial. If you’re working with metal, stone, or another\
mineral substance, however, the fabricated object\
can be no larger than Medium (contained within a\
5-foot Cube). The quality of any fabricated objects is\
based on the quality of the raw materials.\
Creatures and magic items can’t be created by\
this spell. You also can’t use it to create items that\
require a high degree of skill—such as weapons and\
armor—unless you have proficiency with the type\
of Artisan’s Tools used to craft such objects.\

Original layout: [[../../srd-5.2.1/pages/129|page 129]].
