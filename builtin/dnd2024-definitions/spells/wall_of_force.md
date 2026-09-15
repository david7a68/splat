---
splat_schema: 1
splat_id: 523f2b01-7d7f-5e8a-b654-c6141dd94e91
splat_kind: spell
splat_key: dnd2024:spells/wall_of_force
splat_name: Wall of Force
splat_origin:
  key: dnd2024:reference/spell/wall_of_force
splat_sources:
- title: SRD 5.2.1
  locator: pp. 172
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: evocation
  classes:
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 120
  material:
    description: a shard of glass
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/wall_of_force
  implementation: dnd2024:spell_wall_of_force
  parameters:
    targeting:
      kind: chosen_point_in_range
    wall:
      invisible: true
      orientation: caster_choice_any
      support_choice:
      - free_floating
      - solid_surface
      shape_choices:
      - shape: hemispherical_dome
        radius_max_feet: 10
      - shape: globe
        radius_max_feet: 10
      - shape: flat_panels
        count: 10
        width_feet: 10
        height_feet: 10
        each_contiguous_with_another: true
      thickness_inches:
        numerator: 1
        denominator: 4
      duration: spell_duration
    initial_overlap:
      targets: creatures_cut_by_wall
      push_to_side: caster_choice
    barrier:
      physical_passage: forbidden
      damage_immunity: all
      dispel_magic: ineffective
      disintegrate: instant_destruction
      extends_into: ethereal_plane
      ethereal_passage: forbidden
    spell_lifetime:
      concentration: true
      maximum:
        value: 10
        unit: minute
splat_contracts:
- id: dnd2024:spell_wall_of_force
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
  outputs:
  - effects
  - damage_results
  - locations
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - spatial_effects
  - planar_travel
splat_conversion:
  status: specified
  category: spell
  source_sha256: 0fc382f11903de45503d9b620d182d8d2d103cd27265f712de7994ff495f7fe2
  source_path: builtin/srd-5.2.1/spell/wall_of_force.md
  pages:
  - 172
  engine_gaps: &id001
  - dnd2024:spell_wall_of_force
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 6be06e9a473a22bd84b458ff4f77a246c0bcc8cb6ab94e075be4ebe3ff422f87
splat_clauses:
- key: source_rules
  text: '# Wall of Force Level 5 Evocation (Wizard) Casting Time: Action Range: 120 feet Components: V,
    S, M (a shard of glass) Duration: Concentration, up to 10 minutes An Invisible wall of force springs
    into existence at a point you choose within range. The wall appears in any orientation you choose,
    as a horizontal or vertical barrier or at an angle. It can be free floating or resting on a solid
    surface. You can form it into a hemispherical dome or a globe with a radius of up to 10 feet, or you
    can shape a flat surface made up of ten 10-foot-by-10-foot panels. Each panel must be contiguous with
    another panel. In any form, the wall is 1/4 inch thick and lasts for the duration. If the wall cuts
    through a creature’s space when it appears, the creature is pushed to one side of the wall (you choose
    which side). Nothing can physically pass through the wall. It is immune to all damage and can’t be
    dispelled by Dispel Magic. A Disintegrate spell destroys the wall instantly, however. The wall also
    extends into the Ethereal Plane and blocks ethereal travel through the wall.'
  sha256: 963f2efe6bb2e5a96a1fd4ddb1d636474a12247a353a7b484c848cd8b2256d12
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wall of Force

<!-- SRD page 172 -->

Level 5 Evocation (Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a shard of glass)\
Duration: Concentration, up to 10 minutes\
An Invisible wall of force springs into existence at\
a point you choose within range. The wall appears\
in any orientation you choose, as a horizontal or\
vertical barrier or at an angle. It can be free floating\
or resting on a solid surface. You can form it into a\
hemispherical dome or a globe with a radius of up\
to 10 feet, or you can shape a flat surface made up\
of ten 10-foot-by-10-foot panels. Each panel must\
be contiguous with another panel. In any form, the\
wall is 1/4 inch thick and lasts for the duration. If\
the wall cuts through a creature’s space when it ap-\
pears, the creature is pushed to one side of the wall\
(you choose which side).\
Nothing can physically pass through the wall. It\
is immune to all damage and can’t be dispelled by\
Dispel Magic. A Disintegrate spell destroys the wall\
instantly, however. The wall also extends into the\
Ethereal Plane and blocks ethereal travel through\
the wall.\

Original layout: [[../../srd-5.2.1/pages/172|page 172]].
