---
splat_schema: 1
splat_id: af3616c0-4b1f-5a0d-b2d9-1f0fff2fb376
splat_kind: spell
splat_key: dnd2024:spells/web
splat_name: Web
splat_origin:
  key: dnd2024:reference/spell/web
splat_sources:
- title: SRD 5.2.1
  locator: pp. 174
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: conjuration
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    feet: 60
  material:
    description: a bit of spiderweb
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/web
  implementation: dnd2024:spell_web
  parameters:
    area:
      shape: cube
      side_feet: 20
      difficult_terrain: true
      lightly_obscured: true
    support:
      valid:
      - anchored_between_two_solid_masses
      - layered_floor_wall_or_ceiling
      unsupported:
        collapse: true
        end_at: caster.next_turn.start
      flat_surface_depth_feet: 5
    triggers:
    - area.first_entry_on_turn
    - turn.start
    save:
      ability: dexterity
      dc:
        binding: save_dc
      failure:
        condition: restrained
        until_any:
        - leaves_webs
        - breaks_free
      success:
        effect: none
    escape:
      action: action
      ability: strength
      skill: athletics
      dc:
        binding: save_dc
      success: end_restrained
    fire:
      cube_side_feet: 5
      burn_away_rounds: 1
      damage_event: turn.start
      target: creature_in_burning_webs
      damage:
        amount:
          dice:
            count: 2
            sides: 4
            add: 0
        type: fire
splat_contracts:
- id: dnd2024:spell_web
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    save_dc:
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
  - web_area
  - saves
  - restrained
  - escape_check
  - burning_webs
  - damage
  events:
  - spell.cast_completed
  - turn.start
  - area.enter
  capabilities:
  - spellcasting
  - effects
  - damage
  - saving_throws
  - conditions
  - spatial_areas
  - perception
  - time
  - checks
splat_conversion:
  status: specified
  category: spell
  source_sha256: 0d38279be42915b7f7d797dd9654718058bca826e8f692031900047b8427e4b0
  source_path: builtin/srd-5.2.1/spell/web.md
  pages:
  - 174
  engine_gaps: &id001
  - dnd2024:spell_web
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: c6fbc5336c99bb45c9f86ae52ada64119019d1ee046fd7fcb7d675d6a471ada8
splat_clauses:
- key: source_rules
  text: '# Web Level 2 Conjuration (Sorcerer, Wizard) Casting Time: Action Range: 60 feet Components:
    V, S, M (a bit of spiderweb) Duration: Concentration, up to 1 hour You conjure a mass of sticky webbing
    at a point within range. The webs fill a 20-foot Cube there for the duration. The webs are Difficult
    Terrain, and the area within them is Lightly Obscured. If the webs aren’t anchored between two solid
    masses (such as walls or trees) or layered across a floor, wall, or ceiling, the web collapses on
    itself, and the spell ends at the start of your next turn. Webs layered over a flat surface have a
    depth of 5 feet. The first time a creature enters the webs on a turn or starts its turn there, it
    must succeed on a Dexterity saving throw or have the Restrained condition while in the webs or until
    it breaks free. A creature Restrained by the webs can take an action to make a Strength (Athletics)
    check against your spell save DC. If it succeeds, it is no longer Restrained. The webs are flammable.
    Any 5-foot Cube of webs exposed to fire burns away in 1 round, dealing 2d4 Fire damage to any creature
    that starts its turn in the fire.'
  sha256: 2be32144b68fc735f1b65407becfc457284da416947ca66c92d7384ea0395766
  status: contract_specified
splat_requires_mechanics: *id001
---

# Web

<!-- SRD page 174 -->

Level 2 Conjuration (Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a bit of spiderweb)\
Duration: Concentration, up to 1 hour\
You conjure a mass of sticky webbing at a point\
within range. The webs fill a 20-foot Cube there for\
the duration. The webs are Difficult Terrain, and the\
area within them is Lightly Obscured.\
If the webs aren’t anchored between two solid\
masses (such as walls or trees) or layered across\
a floor, wall, or ceiling, the web collapses on itself,\
and the spell ends at the start of your next turn.\
Webs layered over a flat surface have a depth of 5\
feet.\
The first time a creature enters the webs on a\
turn or starts its turn there, it must succeed on a\
Dexterity saving throw or have the Restrained con-\
dition while in the webs or until it breaks free.\
A creature Restrained by the webs can take an\
action to make a Strength (Athletics) check against\
your spell save DC. If it succeeds, it is no longer\
Restrained.\
The webs are flammable. Any 5-foot Cube of webs\
exposed to fire burns away in 1 round, dealing 2d4\
Fire damage to any creature that starts its turn in\
the fire.\

Original layout: [[../../srd-5.2.1/pages/174|page 174]].
