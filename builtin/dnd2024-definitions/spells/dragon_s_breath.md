---
splat_schema: 1
splat_id: cc701950-d55b-5b07-bea0-f54931e8b41e
splat_kind: spell
splat_key: dnd2024:spells/dragon_s_breath
splat_name: Dragon’s Breath
splat_origin:
  key: dnd2024:reference/spell/dragon_s_breath
splat_sources:
- title: SRD 5.2.1
  locator: pp. 126
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: transmutation
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Bonus Action
  ritual: false
  range:
    mode: touch
  material:
    description: a hot pepper
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/dragon_s_breath
  implementation: dnd2024:spell_dragon_s_breath
  parameters:
    targeting:
      kind: creature
      count: 1
      willing: true
      touch: true
    damage_choice:
    - acid
    - cold
    - fire
    - lightning
    - poison
    granted_action:
      duration: spell_duration
      economy: magic_action
      area:
        shape: cone
        length_feet: 15
        origin: target
      targets: all_creatures_in_area
      save:
        ability: dexterity
        dc: caster_spell_save_dc
        failure:
          damage:
            dice: 3d6
            type: chosen_type
        success:
          damage_fraction:
            numerator: 1
            denominator: 2
    slot_scaling:
      above: 2
      damage_per_level: 1d6
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_dragon_s_breath
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
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - spatial_effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: aa2a95d3ad37db67fb89d1892e08d113915eadf50ba5c9ae3d223e4631adff2f
  source_path: builtin/srd-5.2.1/spell/dragon_s_breath.md
  pages:
  - 126
  engine_gaps: &id001
  - dnd2024:spell_dragon_s_breath
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: e4292e2d6b279d4bb0fd6b132bc544690cc0bc39f5f585813ecb8345cc8bf8d0
splat_clauses:
- key: source_rules
  text: '# Dragon’s Breath Level 2 Transmutation (Sorcerer, Wizard) Casting Time: Bonus Action Range:
    Touch Components: V, S, M (a hot pepper) Duration: Concentration, up to 1 minute You touch one willing
    creature, and choose Acid, Cold, Fire, Lightning, or Poison. Until the spell ends, the target can
    take a Magic action to exhale a 15-foot Cone. Each creature in that area makes a Dexterity saving
    throw, taking 3d6 damage of the chosen type on a failed save or half as much damage on a successful
    one. Using a Higher-Level Spell Slot. The damage increases by 1d6 for each spell slot level above
    2.'
  sha256: ee674f5a4feb0fc5e75a998169db27899f8c80b9e70aa55cf9aa88faa23a7972
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dragon’s Breath

<!-- SRD page 126 -->

Level 2 Transmutation (Sorcerer, Wizard)\
Casting Time: Bonus Action\
Range: Touch\
Components: V, S, M (a hot pepper)\
Duration: Concentration, up to 1 minute\
You touch one willing creature, and choose Acid,\
Cold, Fire, Lightning, or Poison. Until the spell\
ends, the target can take a Magic action to exhale\
a 15-foot Cone. Each creature in that area makes a\
Dexterity saving throw, taking 3d6 damage of the\
chosen type on a failed save or half as much damage\
on a successful one.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d6 for each spell slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/126|page 126]].
