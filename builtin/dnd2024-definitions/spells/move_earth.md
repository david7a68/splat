---
splat_schema: 1
splat_id: 06a61f39-b3d7-50c2-bf50-fc19094ec9b9
splat_kind: spell
splat_key: dnd2024:spells/move_earth
splat_name: Move Earth
splat_origin:
  key: dnd2024:reference/spell/move_earth
splat_sources:
- title: SRD 5.2.1
  locator: pp. 151
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: transmutation
  classes:
  - druid
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 2 hours
  casting_time: Action
  ritual: false
  range:
    feet: 120
  material:
    description: a miniature shovel
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/move_earth
  implementation: dnd2024:spell_move_earth
  parameters:
    targeting:
      kind: terrain_area
      max_side_feet: 40
      within_range: true
      materials:
      - dirt
      - sand
      - clay
    reshape:
      chooser: caster
      modes:
      - raise_elevation
      - lower_elevation
      - create_trench
      - fill_trench
      - erect_wall
      - flatten_wall
      - form_pillar
      - other_shape
      max_extent:
        fraction_of_largest_area_dimension:
          numerator: 1
          denominator: 2
      completion_minutes: 10
      ordinary_trapping_or_injury: false
      exceptional_trapping_or_injury:
        type: choice
        chooser: gm
        requires: exceptional_terrain_circumstances
        resolution_required: true
    new_area:
      event: each_10_minutes_of_concentration_completed
      optional: true
      within_range: true
    exclusions:
    - natural_stone_manipulation
    - stone_construction_manipulation
    - direct_plant_growth_effect
    carried_with_terrain:
    - rocks
    - structures
    - plants
    unstable_structure:
      collapse:
        chooser: gm
        type: boolean
        requires: reshaping_makes_structure_unstable
    spell_lifetime:
      concentration: true
      maximum:
        value: 2
        unit: hour
splat_contracts:
- id: dnd2024:spell_move_earth
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
    locations:
      type: location_list
      required: true
    choices:
      type: choice
      required: true
    campaign:
      type: campaign
      required: true
  outputs:
  - effects
  - locations
  events:
  - spell.cast_completed
  - time.elapsed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - spatial_effects
  - campaign_state
splat_conversion:
  status: specified
  category: spell
  source_sha256: 5e5bc3edc26ea4be56fcd1741c8b12cc7c0dc0599593eaa01245931ca33f76c9
  source_path: builtin/srd-5.2.1/spell/move_earth.md
  pages:
  - 151
  engine_gaps: &id001
  - dnd2024:spell_move_earth
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: fdfd1445fff2e132606de9c4a670aa9a6f491156ceafb8e34be123e537bd34d3
splat_clauses:
- key: source_rules
  text: '# Move Earth Level 6 Transmutation (Druid, Sorcerer, Wizard) Casting Time: Action Range: 120
    feet Components: V, S, M (a miniature shovel) Duration: Concentration, up to 2 hours Choose an area
    of terrain no larger than 40 feet on a side within range. You can reshape dirt, sand, or clay in the
    area in any manner you choose for the duration. You can raise or lower the area’s elevation, create
    or fill in a trench, erect or flatten a wall, or form a pillar. The extent of any such changes can’t
    exceed half the area’s largest dimension. For example, if you affect a 40-foot square, you can create
    a pillar up to 20 feet high, raise or lower the square’s elevation by up to 20 feet, dig a trench
    up to 20 feet deep, and so on. It takes 10 minutes for these changes to complete. Because the terrain’s
    transformation occurs slowly, creatures in the area can’t usually be trapped or injured by the ground’s
    movement. At the end of every 10 minutes you spend concentrating on the spell, you can choose a new
    area of terrain to affect within range. This spell can’t manipulate natural stone or stone construction.
    Rocks and structures shift to accommodate the new terrain. If the way you shape the terrain would
    make a structure unstable, it might collapse. Similarly, this spell doesn’t directly affect plant
    growth. The moved earth carries any plants along with it.'
  sha256: 3404af1e4327129ec98f388cbafb06c401e8d2f94863914b173d2e89acaf2887
  status: contract_specified
splat_requires_mechanics: *id001
---

# Move Earth

<!-- SRD page 151 -->

Level 6 Transmutation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a miniature shovel)\
Duration: Concentration, up to 2 hours\
Choose an area of terrain no larger than 40 feet on\
a side within range. You can reshape dirt, sand, or\
clay in the area in any manner you choose for the\
duration. You can raise or lower the area’s eleva-\
tion, create or fill in a trench, erect or flatten a wall,\
or form a pillar. The extent of any such changes\
can’t exceed half the area’s largest dimension. For\
example, if you affect a 40-foot square, you can\
create a pillar up to 20 feet high, raise or lower the\
square’s elevation by up to 20 feet, dig a trench up\
to 20 feet deep, and so on. It takes 10 minutes for\
these changes to complete. Because the terrain’s\
transformation occurs slowly, creatures in the area\
can’t usually be trapped or injured by the ground’s\
movement.\
At the end of every 10 minutes you spend concen-\
trating on the spell, you can choose a new area of\
terrain to affect within range.\
This spell can’t manipulate natural stone or stone\
construction. Rocks and structures shift to accom-\
modate the new terrain. If the way you shape the\
terrain would make a structure unstable, it might\
collapse.\
Similarly, this spell doesn’t directly affect plant\
growth. The moved earth carries any plants along\
with it.\

Original layout: [[../../srd-5.2.1/pages/151|page 151]].
