---
splat_schema: 1
splat_id: 804e0928-5a2a-52de-965c-777d72385552
splat_kind: spell
splat_key: dnd2024:spells/lightning_bolt
splat_name: Lightning Bolt
splat_origin:
  key: dnd2024:reference/spell/lightning_bolt
splat_sources:
- title: SRD 5.2.1
  locator: pp. 144
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: evocation
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: a bit of fur and a crystal rod
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/lightning_bolt
  implementation: dnd2024:spell_lightning_bolt
  parameters:
    targeting:
      kind: creatures_in_area
      shape: line
      length_feet: 100
      width_feet: 5
      origin: caster
      direction: caster_choice
    save:
      ability: dexterity
      failure:
        damage:
          dice: 8d6
          type: lightning
      success:
        damage_fraction:
          numerator: 1
          denominator: 2
    slot_scaling:
      above: 3
      damage_per_level: 1d6
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_lightning_bolt
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
    save_dc:
      type: integer
      required: true
    choices:
      type: choice
      required: true
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - spatial_effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: ab6439705a0987ba521a04afe458af0b2bd97a71ec7fb3bbedef919ff4e84c11
  source_path: builtin/srd-5.2.1/spell/lightning_bolt.md
  pages:
  - 144
  engine_gaps: &id001
  - dnd2024:spell_lightning_bolt
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 3a592abf60c7390441134823e604df8aff3fd726ad61c23a190530bfe9781952
splat_clauses:
- key: source_rules
  text: '# Lightning Bolt Level 3 Evocation (Sorcerer, Wizard) Casting Time: Action Range: Self Components:
    V, S, M (a bit of fur and a crystal rod) Duration: Instantaneous A stroke of lightning forming a 100-foot-long,
    5-foot-wide Line blasts out from you in a direction you choose. Each creature in the Line makes a
    Dexterity saving throw, taking 8d6 Lightning damage on a failed save or half as much damage on a successful
    one. Using a Higher-Level Spell Slot. The damage increases by 1d6 for each spell slot level above
    3.'
  sha256: aeda3e846d13d3cb955ac7b37ef76763ee80173be208e07e08692a01ef14e418
  status: contract_specified
splat_requires_mechanics: *id001
---

# Lightning Bolt

<!-- SRD page 144 -->

Level 3 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a bit of fur and a crystal rod)\
Duration: Instantaneous\
A stroke of lightning forming a 100-foot-long,\
5-foot-wide Line blasts out from you in a direction\
you choose. Each creature in the Line makes a Dex-\
terity saving throw, taking 8d6 Lightning damage\
on a failed save or half as much damage on a suc-\
cessful one.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d6 for each spell slot level above 3.\

Original layout: [[../../srd-5.2.1/pages/144|page 144]].
