---
splat_schema: 1
splat_id: 94b66205-d1c9-5a86-8d01-2c6c7c49c9e9
splat_kind: spell
splat_key: dnd2024:spells/holy_aura
splat_name: Holy Aura
splat_origin:
  key: dnd2024:reference/spell/holy_aura
splat_sources:
- title: SRD 5.2.1
  locator: pp. 141
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: abjuration
  classes:
  - cleric
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: a reliquary worth 1,000+ GP
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/holy_aura
  implementation: dnd2024:spell_holy_aura
  parameters:
    targeting:
      kind: chosen_creatures_in_aura
      shape: emanation
      radius_feet: 30
      origin: caster
    duration: spell_duration
    benefits_while_inside:
      saving_throws: advantage
      attacks_by_other_creatures_against_affected: disadvantage
    retaliation:
      event: melee_attack_roll_hits_affected_creature
      attacker_types:
      - fiend
      - undead
      save:
        ability: constitution
        failure:
          condition: blinded
          until: attacker_next_turn_end
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_holy_aura
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
  events:
  - spell.cast_completed
  - area.entered
  - attack.hit
  - position.changed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - conditions
  - spatial_effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: b89c7efd695e92a26f128c4284929fd81a6f705993bd2fe51fbadcb50796cbd4
  source_path: builtin/srd-5.2.1/spell/holy_aura.md
  pages:
  - 141
  engine_gaps: &id001
  - dnd2024:spell_holy_aura
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 836642f486801ed8f9e3242ef095c5997d1ad6c152926be0ff223aa1a6902b77
splat_clauses:
- key: source_rules
  text: '# Holy Aura Level 8 Abjuration (Cleric) Casting Time: Action Range: Self Components: V, S, M
    (a reliquary worth 1,000+ GP) Duration: Concentration, up to 1 minute For the duration, you emit an
    aura in a 30-foot Emanation. While in the aura, creatures of your choice have Advantage on all saving
    throws, and other creatures have Disadvantage on attack rolls against them. In addition, when a Fiend
    or an Undead hits an affected creature with a melee attack roll, the attacker must succeed on a Constitution
    saving throw or have the Blinded condition until the end of its next turn.'
  sha256: c0abe93127a87d2067cb97b48cd97b715416a9defbf895fc60b1ee89e2416117
  status: contract_specified
splat_requires_mechanics: *id001
---

# Holy Aura

<!-- SRD page 141 -->

Level 8 Abjuration (Cleric)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a reliquary worth 1,000+ GP)\
Duration: Concentration, up to 1 minute\
For the duration, you emit an aura in a 30-foot Em-\
anation. While in the aura, creatures of your choice\
have Advantage on all saving throws, and other\
creatures have Disadvantage on attack rolls against\
them. In addition, when a Fiend or an Undead hits\
an affected creature with a melee attack roll, the at-\
tacker must succeed on a Constitution saving throw\
or have the Blinded condition until the end of its\
next turn.\

Original layout: [[../../srd-5.2.1/pages/141|page 141]].
