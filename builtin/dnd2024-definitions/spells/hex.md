---
splat_schema: 1
splat_id: bf2a6ef1-f17f-5c00-b9dd-6620049145bf
splat_kind: spell
splat_key: dnd2024:spells/hex
splat_name: Hex
splat_origin:
  key: dnd2024:reference/spell/hex
splat_sources:
- title: SRD 5.2.1
  locator: pp. 140
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: enchantment
  classes:
  - warlock
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Bonus Action
  ritual: false
  range:
    feet: 90
  material:
    description: the petrified eye of a newt
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/hex
  implementation: dnd2024:spell_hex
  parameters:
    targeting:
      kind: creature
      count: 1
      visible: true
      within_range: true
    curse:
      duration: spell_duration
      additional_damage:
        event: caster_hits_target_with_attack_roll
        dice: 1d6
        type: necrotic
      disadvantage:
        rolls: ability_checks
        ability: caster_choice_at_cast
    transfer:
      requires: target_dropped_to_zero_hp_before_spell_ends
      economy: bonus_action
      timing: later_turn
      target: new_creature
    slot_duration_hours:
      '1': 1
      '2': 4
      '3': 8
      '4': 8
      '5': 24
      '6': 24
      '7': 24
      '8': 24
      '9': 24
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: hour
splat_contracts:
- id: dnd2024:spell_hex
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
    choices:
      type: choice
      required: true
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  - damage.applied
  - attack.hit
  - health.changed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
splat_conversion:
  status: specified
  category: spell
  source_sha256: 6d3b3f4721d2f4b069752788643a46f324352a80394637c1121ae0aadcb00575
  source_path: builtin/srd-5.2.1/spell/hex.md
  pages:
  - 140
  engine_gaps: &id001
  - dnd2024:spell_hex
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 9a090afbb8d48bdcbe5eeac566a5231b6371b3dc9189ba903d2acc673012bd17
splat_clauses:
- key: source_rules
  text: '# Hex Level 1 Enchantment (Warlock) Casting Time: Bonus Action Range: 90 feet Components: V,
    S, M (the petrified eye of a newt) Duration: Concentration, up to 1 hour You place a curse on a creature
    that you can see within range. Until the spell ends, you deal an extra 1d6 Necrotic damage to the
    target whenever you hit it with an attack roll. Also, choose one ability when you cast the spell.
    The target has Disadvantage on ability checks made with the chosen ability. If the target drops to
    0 Hit Points before this spell ends, you can take a Bonus Action on a later turn to curse a new creature.
    Using a Higher-Level Spell Slot. Your Concentration can last longer with a spell slot of level 2 (up
    to 4 hours), 3–4 (up to 8 hours), or 5+ (24 hours).'
  sha256: 375629f6d0e9e07457a7e6df70d59fc9b07042a1676042bd62126b0d31913656
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hex

<!-- SRD page 140 -->

Level 1 Enchantment (Warlock)\
Casting Time: Bonus Action\
Range: 90 feet\
Components: V, S, M (the petrified eye of a newt)\
Duration: Concentration, up to 1 hour\
You place a curse on a creature that you can see\
within range. Until the spell ends, you deal an extra\
1d6 Necrotic damage to the target whenever you hit\
it with an attack roll. Also, choose one ability when\
you cast the spell. The target has Disadvantage on\
ability checks made with the chosen ability.\
If the target drops to 0 Hit Points before this spell\
ends, you can take a Bonus Action on a later turn to\
curse a new creature.\
Using a Higher-Level Spell Slot. Your Concentra-\
tion can last longer with a spell slot of level 2 (up to\
4 hours), 3–4 (up to 8 hours), or 5+ (24 hours).\

Original layout: [[../../srd-5.2.1/pages/140|page 140]].
