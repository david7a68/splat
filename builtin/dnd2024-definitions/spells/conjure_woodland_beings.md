---
splat_schema: 1
splat_id: 2d39873f-7dc8-5f51-b6d6-87cef64321af
splat_kind: spell
splat_key: dnd2024:spells/conjure_woodland_beings
splat_name: Conjure Woodland Beings
splat_origin:
  key: dnd2024:reference/spell/conjure_woodland_beings
splat_sources:
- title: SRD 5.2.1
  locator: pp. 118, 119
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
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
    mode: self
splat_rule:
  mechanic: dnd2024:spells/conjure_woodland_beings
  implementation: dnd2024:spell_conjure_woodland_beings
  parameters:
    area:
      shape: emanation
      radius_feet: 10
      origin: caster
    triggers:
    - area.moves_into_creature
    - area.enter
    - turn.end
    target:
      kind: creature
      count: each_eligible
      visible_to_caster: true
    optional: true
    save:
      ability: wisdom
      dc:
        binding: save_dc
      failure:
        damage:
          amount:
            dice:
              count: 5
              sides: 8
              add: 0
          type: force
      success:
        damage_fraction:
          numerator: 1
          denominator: 2
    limit:
      saves_per_creature_per_turn: 1
    grant:
      action: disengage
      cost: bonus_action
      duration: spell
    scaling:
      per_slot_above: 4
      damage_dice: 1
      sides: 8
splat_contracts:
- id: dnd2024:spell_conjure_woodland_beings
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
  - aura_effect
  - saves
  - damage
  - bonus_disengage
  events:
  - spell.cast_completed
  - turn.end
  - area.enter
  - area.moved
  capabilities:
  - spellcasting
  - effects
  - damage
  - saving_throws
  - spatial_areas
  - movement
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: e797d1e71ca8d6c7241af5a982c73ef859e8bbfcd62de85efd2ec909908b54a6
  source_path: builtin/srd-5.2.1/spell/conjure_woodland_beings.md
  pages:
  - 118
  - 119
  engine_gaps: &id001
  - dnd2024:spell_conjure_woodland_beings
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: c8b279c36acac89531f39d97122f8dafd52d74b5726a6cb8fb09394aec83f49e
splat_clauses:
- key: source_rules
  text: '# Conjure Woodland Beings Level 4 Conjuration (Druid, Ranger) Casting Time: Action Range: Self
    Components: V, S Duration: Concentration, up to 10 minutes You conjure nature spirits that flit around
    you in a 10-foot Emanation for the duration. Whenever the Emanation enters the space of a creature
    you can see and whenever a creature you can see enters the Emanation or ends its turn there, you can
    force that creature to make a Wisdom saving throw. The creature takes 5d8 Force damage on a failed
    save or half as much damage on a successful one. A creature makes this save only once per turn. In
    addition, you can take the Disengage action as a Bonus Action for the spell’s duration. Using a Higher-Level
    Spell Slot. The damage increases by 1d8 for each spell slot level above 4.'
  sha256: 51e5e30bde00eb98b5b6f3b4065e20406a69c42bc4fea6a80570917f69e7cd93
  status: contract_specified
splat_requires_mechanics: *id001
---

# Conjure Woodland Beings

<!-- SRD page 118 -->

Level 4 Conjuration (Druid, Ranger)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 10 minutes\
You conjure nature spirits that flit around you in a\
10-foot Emanation for the duration. Whenever the\
Emanation enters the space of a creature you can\

<!-- SRD page 119 -->

see and whenever a creature you can see enters\
the Emanation or ends its turn there, you can force\
that creature to make a Wisdom saving throw. The\
creature takes 5d8 Force damage on a failed save or\
half as much damage on a successful one. A creature\
makes this save only once per turn.\
In addition, you can take the Disengage action as a\
Bonus Action for the spell’s duration.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 4.\

Original layout: [[../../srd-5.2.1/pages/118|page 118]], [[../../srd-5.2.1/pages/119|page 119]].
