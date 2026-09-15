---
splat_schema: 1
splat_id: 04c83d44-8888-569d-93c9-6d1dd55650c0
splat_kind: spell
splat_key: dnd2024:spells/instant_summons
splat_name: Instant Summons
splat_origin:
  key: dnd2024:reference/spell/instant_summons
splat_sources:
- title: SRD 5.2.1
  locator: pp. 143
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: conjuration
  classes:
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Until dispelled
  casting_time: 1 minute or Ritual
  ritual: true
  range:
    mode: touch
  material:
    description: a sapphire worth 1,000+ GP
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/instant_summons
  implementation: dnd2024:spell_instant_summons
  parameters:
    target:
      kind: object
      count: 1
      visible_to_caster: false
      touch: true
      maximum_weight_pounds: 10
      maximum_dimension_feet: 6
    sapphire:
      touch: true
      different_each_cast: true
      invisible_inscription: object_name
    mark:
      invisible: true
      'on': object
    summon:
      action: magic
      requirements:
      - speak_object_name
      - crush_sapphire
      range: any_physical_or_planar_distance
      success:
        move_object_to: caster_hand
        end_spell: true
      held_or_carried_by_other_creature:
        transport: false
        reveal:
        - holder_identity
        - holder_current_location
splat_contracts:
- id: dnd2024:spell_instant_summons
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
  - object_mark
  - sapphire_link
  - object_transport
  - holder_information
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - movement
  - perception
  - inventory
  - summoning
splat_conversion:
  status: specified
  category: spell
  source_sha256: 06a7da6e6ad2349aa8bae24521abaac6ac10596defc772383d6f5d9d0e3a2b15
  source_path: builtin/srd-5.2.1/spell/instant_summons.md
  pages:
  - 143
  engine_gaps: &id001
  - dnd2024:spell_instant_summons
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: b473d9f7b6bca01b6f71a0285e4ea9bd5c8a2d8bd2ff2796d3dd8e9fdf7b22ec
splat_clauses:
- key: source_rules
  text: '# Instant Summons Level 6 Conjuration (Wizard) Casting Time: 1 minute or Ritual Range: Touch
    Components: V, S, M (a sapphire worth 1,000+ GP) Duration: Until dispelled You touch the sapphire
    used in the casting and an object weighing 10 pounds or less whose longest dimension is 6 feet or
    less. The spell leaves an Invisible mark on that object and invisibly inscribes the object’s name
    on the sapphire. Each time you cast this spell, you must use a different sapphire. Thereafter, you
    can take a Magic action to speak the object’s name and crush the sapphire. The object instantly appears
    in your hand regardless of physical or planar distances, and the spell ends. If another creature is
    holding or carrying the object, crushing the sapphire doesn’t transport it, but instead you learn
    who that creature is and where that creature is currently located.'
  sha256: 7adabe9109b33697e13d47880a113214ebc92e019f68599cca362e0a4f5df56a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Instant Summons

<!-- SRD page 143 -->

Level 6 Conjuration (Wizard)\
Casting Time: 1 minute or Ritual\
Range: Touch\
Components: V, S, M (a sapphire worth 1,000+ GP)\
Duration: Until dispelled\
You touch the sapphire used in the casting and an\
object weighing 10 pounds or less whose longest\
dimension is 6 feet or less. The spell leaves an Invis-\
ible mark on that object and invisibly inscribes the\
object’s name on the sapphire. Each time you cast\
this spell, you must use a different sapphire.\
Thereafter, you can take a Magic action to speak\
the object’s name and crush the sapphire. The ob-\
ject instantly appears in your hand regardless of\
physical or planar distances, and the spell ends.\
If another creature is holding or carrying the ob-\
ject, crushing the sapphire doesn’t transport it, but\
instead you learn who that creature is and where\
that creature is currently located.\

Original layout: [[../../srd-5.2.1/pages/143|page 143]].
