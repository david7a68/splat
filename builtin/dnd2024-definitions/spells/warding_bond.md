---
splat_schema: 1
splat_id: 4a37cfcf-025d-51f5-9a98-be41338641fe
splat_kind: spell
splat_key: dnd2024:spells/warding_bond
splat_name: Warding Bond
splat_origin:
  key: dnd2024:reference/spell/warding_bond
splat_sources:
- title: SRD 5.2.1
  locator: pp. 173
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: abjuration
  classes:
  - cleric
  - paladin
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: a pair of platinum rings worth 50+ GP each, which you and the target must wear for the
      duration
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/warding_bond
  implementation: dnd2024:spell_warding_bond
  parameters:
    targeting:
      kind: creature
      count: 1
      willing: true
      other_than_caster: true
      touch: true
    ongoing_requirement:
      caster_and_target_wear_component_rings: true
    benefits:
      within_caster_feet: 60
      ac_bonus: 1
      saving_throw_bonus: 1
      damage_resistance: all
      duration: spell_duration
    shared_damage:
      event: target.takes_damage
      recipient: caster
      amount: same_as_target_damage_taken
    ends_on_any:
    - caster_zero_hp
    - separation_exceeds_60_feet
    - warding_bond_cast_again_on_either_connected_creature
    spell_lifetime:
      concentration: false
      maximum:
        value: 1
        unit: hour
splat_contracts:
- id: dnd2024:spell_warding_bond
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
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  - damage.applied
  - health.changed
  - position.changed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - spatial_effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: 23cf3b80dbe2d36f17a68c90594aeb9822e612b47cf77a9208b8cadbd8e101da
  source_path: builtin/srd-5.2.1/spell/warding_bond.md
  pages:
  - 173
  engine_gaps: &id001
  - dnd2024:spell_warding_bond
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: c6f1576f42b3b888d870153a73db5f6d549a3035a8e19591c402fae70815ff70
splat_clauses:
- key: source_rules
  text: '# Warding Bond Level 2 Abjuration (Cleric, Paladin) Casting Time: Action Range: Touch Components:
    V, S, M (a pair of platinum rings worth 50+ GP each, which you and the target must wear for the duration)
    Duration: 1 hour You touch another creature that is willing and create a mystic connection between
    you and the target until the spell ends. While the target is within 60 feet of you, it gains a +1
    bonus to AC and saving throws, and it has Resistance to all damage. Also, each time it takes damage,
    you take the same amount of damage. The spell ends if you drop to 0 Hit Points or if you and the target
    become separated by more than 60 feet. It also ends if the spell is cast again on either of the connected
    creatures.'
  sha256: 9c7921623fa12be3a5392d87c642a3ffcc576b4a1308d1d229636258a8ed94fe
  status: contract_specified
splat_requires_mechanics: *id001
---

# Warding Bond

<!-- SRD page 173 -->

Level 2 Abjuration (Cleric, Paladin)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a pair of platinum rings worth\
50+ GP each, which you and the target must wear for\
the duration)\
Duration: 1 hour\
You touch another creature that is willing and\
create a mystic connection between you and the\
target until the spell ends. While the target is\
within 60 feet of you, it gains a +1 bonus to AC and\
saving throws, and it has Resistance to all damage.\
Also, each time it takes damage, you take the same\
amount of damage.\
The spell ends if you drop to 0 Hit Points or if you\
and the target become separated by more than 60\
feet. It also ends if the spell is cast again on either of\
the connected creatures.\

Original layout: [[../../srd-5.2.1/pages/173|page 173]].
