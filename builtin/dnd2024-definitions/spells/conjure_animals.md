---
splat_schema: 1
splat_id: 54e4346b-9165-5413-a574-d02241065131
splat_kind: spell
splat_key: dnd2024:spells/conjure_animals
splat_name: Conjure Animals
splat_origin:
  key: dnd2024:reference/spell/conjure_animals
splat_sources:
- title: SRD 5.2.1
  locator: pp. 117, 118
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: conjuration
  classes:
  - druid
  - ranger
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/conjure_animals
  implementation: dnd2024:spell_conjure_animals
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 600
    pack:
      size: large
      spectral: true
      tangible: false
      initial_space:
        unoccupied: true
        visible: true
        range_feet: 60
      animal_appearance:
        chooser: caster
    caster_save_benefit:
      ability: strength
      advantage: true
      within_pack_feet: 5
    movement:
      event: participant.moved
      subject: caster
      caster_turn_only: true
      optional: true
      maximum_feet: 30
      destination:
        unoccupied: true
        visible_to_caster: true
    damage_trigger:
      events:
      - pack.moved_within_distance
      - creature.entered_distance
      - turn.end
      distance_feet: 10
      target_kind: creature
      target_visible_to_caster: true
      chooser: caster
      optional: true
      save_limit:
        count: 1
        per: turn
        scope: target
    save:
      ability: dexterity
      dc: spell_save_dc
      success: no_damage
    failure_damage:
      dice:
        base_count: 3
        sides: 10
        extra_per_slot: 1
        base_slot: 3
      type: slashing
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_conjure_animals
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    pack_location:
      type: location
      required: true
    appearance:
      type: choice
      required: true
    targets:
      required: false
      type: participant_list
    movement_choice:
      required: false
      type: location
    save_dc:
      type: integer
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - damage
  events:
  - concentration.ended
  - creature.entered_distance
  - pack.moved_within_distance
  - participant.moved
  - spell.cast_completed
  - turn.end
  capabilities:
  - spellcasting
  - effects
  - movement
  - areas
  - damage
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: b1e642fc3f5b84708ff31d52cfdf39bfb947d1a1850f496717df162f21436f70
  source_path: builtin/srd-5.2.1/spell/conjure_animals.md
  pages:
  - 117
  - 118
  engine_gaps: &id001
  - dnd2024:spell_conjure_animals
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: dc44dcb4e782bd573791884a5c8e7838dff91adfee0285445b74897277ab290b
splat_clauses:
- key: source_rules
  text: '# Conjure Animals Level 3 Conjuration (Druid, Ranger) Casting Time: Action Range: 60 feet Components:
    V, S Duration: Concentration, up to 10 minutes You conjure nature spirits that appear as a Large pack
    of spectral, intangible animals in an unoccupied space you can see within range. The pack lasts for
    the duration, and you choose the spirits’ animal form, such as wolves, serpents, or birds. You have
    Advantage on Strength saving throws while you’re within 5 feet of the pack, and when you move on your
    turn, you can also move the pack up to 30 feet to an unoccupied space you can see. Whenever the pack
    moves within 10 feet of a creature you can see and whenever a creature you can see enters a space
    within 10 feet of the pack or ends its turn there, you can force that creature to make a Dexterity
    saving throw. On a failed save, the creature takes 3d10 Slashing damage. A creature makes this save
    only once per turn. Using a Higher-Level Spell Slot. The damage increases by 1d10 for each spell slot
    level above 3.'
  sha256: 7749c6a8bc584a31c7e2403157a676bd0d0ebbafb87266aae4eaf205ed5a0f34
  status: contract_specified
splat_requires_mechanics: *id001
---

# Conjure Animals

<!-- SRD page 117 -->

Level 3 Conjuration (Druid, Ranger)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes\
You conjure nature spirits that appear as a Large\
pack of spectral, intangible animals in an unoccu-\
pied space you can see within range. The pack lasts\
for the duration, and you choose the spirits’ animal\
form, such as wolves, serpents, or birds.\
You have Advantage on Strength saving throws\
while you’re within 5 feet of the pack, and when you\
move on your turn, you can also move the pack up\
to 30 feet to an unoccupied space you can see.\
Whenever the pack moves within 10 feet of a crea-\
ture you can see and whenever a creature you can\
see enters a space within 10 feet of the pack or ends\

<!-- SRD page 118 -->

its turn there, you can force that creature to make\
a Dexterity saving throw. On a failed save, the crea-\
ture takes 3d10 Slashing damage. A creature makes\
this save only once per turn.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d10 for each spell slot level above 3.\

Original layout: [[../../srd-5.2.1/pages/117|page 117]], [[../../srd-5.2.1/pages/118|page 118]].
