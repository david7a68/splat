---
splat_schema: 1
splat_id: 91923fd8-5c7e-565e-8d34-ce84dec1e42b
splat_kind: spell
splat_key: dnd2024:spells/commune_with_nature
splat_name: Commune with Nature
splat_origin:
  key: dnd2024:reference/spell/commune_with_nature
splat_sources:
- title: SRD 5.2.1
  locator: pp. 116, 117
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: divination
  classes:
  - druid
  - ranger
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: 1 minute or Ritual
  ritual: true
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/commune_with_nature
  implementation: dnd2024:spell_commune_with_nature
  parameters:
    area:
      outdoors_radius_miles: 3
      natural_underground_radius_feet: 300
      fails_where_nature_replaced_by_construction: true
    knowledge:
      choose_facts_count: 3
      choices:
        settlements:
          output: locations
        planar_portals:
          output: locations
        powerful_creature:
          minimum_challenge_rating: 10
          creature_types:
          - celestial
          - elemental
          - fey
          - fiend
          - undead
          chooser: gm
          count: 1
          output: location
        most_prevalent_kind:
          caster_category_choice:
          - plant
          - mineral
          - beast
          output: kind
        bodies_of_water:
          output: locations
splat_contracts:
- id: dnd2024:spell_commune_with_nature
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    choices:
      required: false
      type: choice
  outputs:
  - effects
  - knowledge
  events:
  - spell.cast_completed
  - movement.completed
  capabilities:
  - spellcasting
  - effects
  - spatial_relationships
  - knowledge
splat_conversion:
  status: specified
  category: spell
  source_sha256: e06c02f1abaa54bcc94d7f29d91042020ee650d8499698dd58e4d656b31a246d
  source_path: builtin/srd-5.2.1/spell/commune_with_nature.md
  pages:
  - 116
  - 117
  engine_gaps: &id001
  - dnd2024:spell_commune_with_nature
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: a034efdfd8e6dd2aa10fcec8b5f04f559a93dcf39f9db9e9373a57c85e6d72f6
splat_clauses:
- key: source_rules
  text: '# Commune with Nature Level 5 Divination (Druid, Ranger) Casting Time: 1 minute or Ritual Range:
    Self Components: V, S Duration: Instantaneous You commune with nature spirits and gain knowledge of
    the surrounding area. In the outdoors, the spell gives you knowledge of the area within 3 miles of
    you. In caves and other natural underground settings, the radius is limited to 300 feet. The spell
    doesn’t function where nature has been replaced by construction, such as in castles and settlements.
    Choose three of the following facts; you learn those facts as they pertain to the spell’s area: •
    Locations of settlements • Locations of portals to other planes of existence • Location of one Challenge
    Rating 10+ creature (GM’s choice) that is a Celestial, an Elemental, a Fey, a Fiend, or an Undead
    • The most prevalent kind of plant, mineral, or Beast (you choose which to learn) • Locations of bodies
    of water For example, you could determine the location of a powerful monster in the area, the locations
    of bodies of water, and the locations of any towns.'
  sha256: 2c2463fd41da0bf49c2d68ea87e50e1471adde7437394971e3177115fa263eb1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Commune with Nature

<!-- SRD page 116 -->

Level 5 Divination (Druid, Ranger)\
Casting Time: 1 minute or Ritual\
Range: Self\
Components: V, S\
Duration: Instantaneous\
You commune with nature spirits and gain knowl-\
edge of the surrounding area. In the outdoors, the\
spell gives you knowledge of the area within 3 miles\
of you. In caves and other natural underground\
settings, the radius is limited to 300 feet. The spell\

<!-- SRD page 117 -->

doesn’t function where nature has been replaced by\
construction, such as in castles and settlements.\
Choose three of the following facts; you learn\
those facts as they pertain to the spell’s area:\
•  Locations of settlements\
•  Locations of portals to other planes of existence\
•  Location of one Challenge Rating 10+ creature\
(GM’s choice) that is a Celestial, an Elemental, a\
Fey, a Fiend, or an Undead\
•  The most prevalent kind of plant, mineral, or\
Beast (you choose which to learn)\
•  Locations of bodies of water\
For example, you could determine the location of a\
powerful monster in the area, the locations of bod-\
ies of water, and the locations of any towns.\

Original layout: [[../../srd-5.2.1/pages/116|page 116]], [[../../srd-5.2.1/pages/117|page 117]].
