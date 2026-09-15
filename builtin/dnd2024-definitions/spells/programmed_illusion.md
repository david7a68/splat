---
splat_schema: 1
splat_id: 4798c2d4-9515-5463-91a6-1adeff908ce8
splat_kind: spell
splat_key: dnd2024:spells/programmed_illusion
splat_name: Programmed Illusion
splat_origin:
  key: dnd2024:reference/spell/programmed_illusion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 156
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: illusion
  classes:
  - bard
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Until dispelled
  casting_time: Action
  ritual: false
  range:
    feet: 120
  material:
    description: jade dust worth 25+ GP
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/programmed_illusion
  implementation: dnd2024:spell_programmed_illusion
  parameters:
    spell_lifetime:
      concentration: false
      until: dispelled_or_source_specific_end
    illusion:
      kinds:
      - object
      - creature
      - visible_phenomenon
      range_feet: 120
      maximum_cube_side_feet: 30
      initial_state: imperceptible
      visual_behavior: caster_script_at_cast
      sounds: caster_script_at_cast
      performance_seconds_max: 300
    trigger:
      chooser: caster
      timing: cast
      modalities:
      - visual
      - audible
      maximum_distance_from_area_feet: 30
      condition_resolution: supplied_trigger_predicate
    cycle:
      on_trigger: perform_script
      after_performance:
        disappear: true
        dormant_seconds: 600
      after_dormancy: triggerable_again
    physical_interaction:
      objects_pass_through: true
      reveals_illusion: true
    examination:
      economy: study_action
      check:
        ability: intelligence
        skill: investigation
        dc: spell_save_dc
      success: discern_illusion
    discerned:
      image: see_through
      noise: hollow
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_programmed_illusion
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    origin:
      type: location
      required: true
    script:
      type: choice
      required: true
    trigger:
      type: choice
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
  capabilities:
  - spellcasting
  - effects
  - illusions
  - areas
  - tests
splat_conversion:
  status: specified
  category: spell
  source_sha256: 66e3168476f7fedadd077d75f15a11fc63263737506c2d1a34bbb5df730a5f63
  source_path: builtin/srd-5.2.1/spell/programmed_illusion.md
  pages:
  - 156
  engine_gaps: &id001
  - dnd2024:spell_programmed_illusion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 8e762b95bfa7f261530103b6b293b977fa64612040b993a825c48a382147d47a
splat_clauses:
- key: source_rules
  text: '# Programmed Illusion Level 6 Illusion (Bard, Wizard) Casting Time: Action Range: 120 feet Components:
    V, S, M (jade dust worth 25+ GP) Duration: Until dispelled You create an illusion of an object, a
    creature, or some other visible phenomenon within range that activates when a specific trigger occurs.
    The illusion is imperceptible until then. It must be no larger than a 30-foot Cube, and you decide
    when you cast the spell how the illusion behaves and what sounds it makes. This scripted performance
    can last up to 5 minutes. When the trigger you specify occurs, the illusion springs into existence
    and performs in the manner you described. Once the illusion finishes performing, it disappears and
    remains dormant for 10 minutes, after which the illusion can be activated again. The trigger can be
    as general or as detailed as you like, though it must be based on visual or audible phenomena that
    occur within 30 feet of the area. For example, you could create an illusion of yourself to appear
    and warn off others who attempt to open a trapped door. Physical interaction with the image reveals
    it to be illusory, since things can pass through it. A creature that takes the Study action to examine
    the image can determine that it is an illusion with a successful Intelligence (Investigation) check
    against your spell save DC. If a creature discerns the illusion for what it is, the creature can see
    through the image, and any noise it makes sounds hollow to the creature.'
  sha256: 78fcd384cf022956dd118f7c8965ad505096877ebaa962baaf0a1ae5c5349e50
  status: contract_specified
splat_requires_mechanics: *id001
---

# Programmed Illusion

<!-- SRD page 156 -->

Level 6 Illusion (Bard, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (jade dust worth 25+ GP)\
Duration: Until dispelled\
You create an illusion of an object, a creature, or\
some other visible phenomenon within range that\
activates when a specific trigger occurs. The illu-\
sion is imperceptible until then. It must be no larger\
than a 30-foot Cube, and you decide when you cast\
the spell how the illusion behaves and what sounds\
it makes. This scripted performance can last up to 5\
minutes.\
When the trigger you specify occurs, the illusion\
springs into existence and performs in the manner\
you described. Once the illusion finishes perform-\
ing, it disappears and remains dormant for 10 min-\
utes, after which the illusion can be activated again.\
The trigger can be as general or as detailed as you\
like, though it must be based on visual or audible\
phenomena that occur within 30 feet of the area.\
For example, you could create an illusion of yourself\
to appear and warn off others who attempt to open\
a trapped door.\
Physical interaction with the image reveals it\
to be illusory, since things can pass through it. A\
creature that takes the Study action to examine\
the image can determine that it is an illusion with a\
successful Intelligence (Investigation) check against\
your spell save DC. If a creature discerns the illu-\
sion for what it is, the creature can see through the\
image, and any noise it makes sounds hollow to the\
creature.\

Original layout: [[../../srd-5.2.1/pages/156|page 156]].
