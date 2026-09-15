---
splat_schema: 1
splat_id: 3fc3bcca-9101-5850-aec5-f2ee508380ba
splat_kind: spell
splat_key: dnd2024:spells/elementalism
splat_name: Elementalism
splat_origin:
  key: dnd2024:reference/spell/elementalism
splat_sources:
- title: SRD 5.2.1
  locator: pp. 127
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: transmutation
  classes:
  - druid
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 30
splat_rule:
  mechanic: dnd2024:spells/elementalism
  implementation: dnd2024:spell_elementalism
  parameters:
    selection:
      effect_count: 1
      within_spell_range: true
    modes:
      beckon_air:
        shape: cube
        side_feet: 5
        effects:
        - ripple_cloth
        - stir_dust
        - rustle_leaves
        - close_doors
        - close_shutters
        doors_shutters_held_open_immune: true
      beckon_earth:
        choices:
        - cover: thin_dust_or_sand
          shape: square
          side_feet: 5
        - write_words_maximum: 1
          medium:
          - dirt
          - sand
          handwriting: caster
      beckon_fire:
        shape: cube
        side_feet: 5
        embers_harmless: true
        smoke_color_choice: true
        smoke_scent_choice: true
        scent_duration_minutes: 1
        can_light:
        - candles
        - torches
        - lamps
      beckon_water:
        choices:
        - shape: cube
          side_feet: 5
          effect: lightly_dampen_creatures_and_objects
          temperature: cool
        - clean_water_cups: 1
          destination:
          - open_container
          - surface
          evaporates_after_minutes: 1
      sculpt_element:
        materials:
        - dirt
        - sand
        - fire
        - smoke
        - mist
        - water
        maximum_cube_side_feet: 1
        shape_choice: crude
        duration_hours: 1
splat_contracts:
- id: dnd2024:spell_elementalism
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    choices:
      required: false
      type: choice
    items:
      required: false
      type: item_list
  outputs:
  - effects
  - items
  events:
  - spell.cast_completed
  - movement.completed
  capabilities:
  - spellcasting
  - effects
  - spatial_relationships
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: fdedfcf5d40b843015c4abece257b4062f3661196da030b79c4468f52b4ae559
  source_path: builtin/srd-5.2.1/spell/elementalism.md
  pages:
  - 127
  engine_gaps: &id001
  - dnd2024:spell_elementalism
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 7853d8218ab34b3a3a263168ed163dcbf5d021659488004fb25c53da80a1b87f
splat_clauses:
- key: source_rules
  text: '# Elementalism Transmutation Cantrip (Druid, Sorcerer, Wizard) Casting Time: Action Range: 30
    feet Components: V, S Duration: Instantaneous You exert control over the elements, creating one of
    the following effects within range. Beckon Air. You create a breeze strong enough to ripple cloth,
    stir dust, rustle leaves, and close open doors and shutters, all in a 5-foot Cube. Doors and shutters
    being held open by someone or something aren’t affected. Beckon Earth. You create a thin shroud of
    dust or sand that covers surfaces in a 5-foot-square area, or you cause a single word to appear in
    your handwriting in a patch of dirt or sand. Beckon Fire. You create a thin cloud of harmless embers
    and colored, scented smoke in a 5-foot Cube. You choose the color and scent, and the embers can light
    candles, torches, or lamps in that area. The smoke’s scent lingers for 1 minute. Beckon Water. You
    create a spray of cool mist that lightly dampens creatures and objects in a 5-foot Cube. Alternatively,
    you create 1 cup of clean water either in an open container or on a surface, and the water evaporates
    in 1 minute. Sculpt Element. You cause dirt, sand, fire, smoke, mist, or water that can fit in a 1-foot
    Cube to assume a crude shape (such as that of a creature) for 1 hour.'
  sha256: 66f5d4fb761a058298dc4fe8da7c3164438179db734aa1d9151e82896ab68a5e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Elementalism

<!-- SRD page 127 -->

Transmutation Cantrip (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Instantaneous\
You exert control over the elements, creating one of\
the following effects within range.\
Beckon Air. You create a breeze strong enough to\
ripple cloth, stir dust, rustle leaves, and close open\
doors and shutters, all in a 5-foot Cube. Doors and\
shutters being held open by someone or something\
aren’t affected.\
Beckon Earth. You create a thin shroud of dust or\
sand that covers surfaces in a 5-foot-square area, or\
you cause a single word to appear in your handwrit-\
ing in a patch of dirt or sand.\
Beckon Fire. You create a thin cloud of harmless\
embers and colored, scented smoke in a 5-foot Cube.\
You choose the color and scent, and the embers can\
light candles, torches, or lamps in that area. The\
smoke’s scent lingers for 1 minute.\
Beckon Water. You create a spray of cool mist that\
lightly dampens creatures and objects in a 5-foot\
Cube. Alternatively, you create 1 cup of clean water\
either in an open container or on a surface, and the\
water evaporates in 1 minute.\
Sculpt Element. You cause dirt, sand, fire, smoke,\
mist, or water that can fit in a 1-foot Cube to assume\
a crude shape (such as that of a creature) for 1 hour.\

Original layout: [[../../srd-5.2.1/pages/127|page 127]].
