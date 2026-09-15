---
splat_schema: 1
splat_id: ae563e1d-69bc-597d-86fb-9bd7183e2e1a
splat_kind: spell
splat_key: dnd2024:spells/spirit_guardians
splat_name: Spirit Guardians
splat_origin:
  key: dnd2024:reference/spell/spirit_guardians
splat_sources:
- title: SRD 5.2.1
  locator: pp. 164, 165
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: conjuration
  classes:
  - cleric
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: a prayer scroll
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/spirit_guardians
  implementation: dnd2024:spell_spirit_guardians
  parameters:
    area:
      shape: emanation
      radius_feet: 15
      origin: caster
    appearance_by_alignment:
      good_or_neutral:
        choice:
        - angelic
        - fey
      evil: fiendish
    exempt:
      designated_creatures: at_cast
    affected:
      speed_fraction_while_inside:
        numerator: 1
        denominator: 2
    triggers:
    - area.moves_into_creature
    - area.enter
    - turn.end
    save:
      ability: wisdom
      dc:
        binding: save_dc
      failure:
        damage:
          amount:
            dice:
              count: 3
              sides: 8
              add: 0
          type_by_caster_alignment:
            good_or_neutral: radiant
            evil: necrotic
      success:
        damage_fraction:
          numerator: 1
          denominator: 2
    limit:
      saves_per_creature_per_turn: 1
    scaling:
      per_slot_above: 3
      damage_dice: 1
      sides: 8
splat_contracts:
- id: dnd2024:spell_spirit_guardians
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
  - speed_modifier
  - saves
  - damage
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
  - choices
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: 73a03709235ce113e22af594454391c3bb87a8874f8544613b140a75c1c78042
  source_path: builtin/srd-5.2.1/spell/spirit_guardians.md
  pages:
  - 164
  - 165
  engine_gaps: &id001
  - dnd2024:spell_spirit_guardians
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 6afc86f0b278e2145b440f3fb1b65aa2089143811a2992949e566b8b7c65407f
splat_clauses:
- key: source_rules
  text: '# Spirit Guardians Level 3 Conjuration (Cleric) Casting Time: Action Range: Self Components:
    V, S, M (a prayer scroll) Duration: Concentration, up to 10 minutes Protective spirits flit around
    you in a 15-foot Emanation for the duration. If you are good or neutral, their spectral form appears
    angelic or fey (your choice). If you are evil, they appear fiendish. When you cast this spell, you
    can designate creatures to be unaffected by it. Any other creature’s Speed is halved in the Emanation,
    and whenever the Emanation enters a creature’s space and whenever a creature enters the Emanation
    or ends its turn there, the creature must make a Wisdom saving throw. On a failed save, the creature
    takes 3d8 Radiant damage (if you are good or neutral) or 3d8 Necrotic damage (if you are evil). On
    a successful save, the creature takes half as much damage. A creature makes this save only once per
    turn. Using a Higher-Level Spell Slot. The damage increases by 1d8 for each spell slot level above
    3.'
  sha256: 65bf88dcab475f576fe31c1f460ebd3b01439ff0604ccf4c5d682c2c7130ff3b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spirit Guardians

<!-- SRD page 164 -->

Level 3 Conjuration (Cleric)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a prayer scroll)\
Duration: Concentration, up to 10 minutes\
Protective spirits flit around you in a 15-foot Ema-\
nation for the duration. If you are good or neutral,\
their spectral form appears angelic or fey (your\
choice). If you are evil, they appear fiendish.\
When you cast this spell, you can designate crea-\
tures to be unaffected by it. Any other creature’s\
Speed is halved in the Emanation, and whenever the\
Emanation enters a creature’s space and whenever\

<!-- SRD page 165 -->

a creature enters the Emanation or ends its turn\
there, the creature must make a Wisdom saving\
throw. On a failed save, the creature takes 3d8 Ra-\
diant damage (if you are good or neutral) or 3d8 Ne-\
crotic damage (if you are evil). On a successful save,\
the creature takes half as much damage. A creature\
makes this save only once per turn.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 3.\

Original layout: [[../../srd-5.2.1/pages/164|page 164]], [[../../srd-5.2.1/pages/165|page 165]].
