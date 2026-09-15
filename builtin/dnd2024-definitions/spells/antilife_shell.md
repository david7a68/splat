---
splat_schema: 1
splat_id: b9a97dc5-72f3-50aa-9b9b-4d952a58ad86
splat_kind: spell
splat_key: dnd2024:spells/antilife_shell
splat_name: Antilife Shell
splat_origin:
  key: dnd2024:reference/spell/antilife_shell
splat_sources:
- title: SRD 5.2.1
  locator: pp. 109
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: abjuration
  classes:
  - druid
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/antilife_shell
  implementation: dnd2024:spell_antilife_shell
  parameters:
    area:
      shape: emanation
      radius_feet: 10
      origin: caster
      duration: spell_duration
    barrier:
      excluded_types:
      - construct
      - undead
      blocks:
      - creature_passing
      - creature_reaching
      allows:
      - spell_casting_through
      - ranged_weapon_attacks_through
      - reach_weapon_attacks_through
    end_when:
      caster_moves_forcing_affected_creature_through: true
splat_contracts:
- id: dnd2024:spell_antilife_shell
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
  events:
  - spell.cast_completed
  - movement.completed
  capabilities:
  - spellcasting
  - effects
  - spatial_relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: d75fa30ad4a5bbff8f8ea3561d558386eb98dbbe0641650d203f8f3e443f2c29
  source_path: builtin/srd-5.2.1/spell/antilife_shell.md
  pages:
  - 109
  engine_gaps: &id001
  - dnd2024:spell_antilife_shell
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: fb4ab01daa26f8db7cfeaa0f92085018291b140d4f1da6067579c476a4097cbb
splat_clauses:
- key: source_rules
  text: '# Antilife Shell Level 5 Abjuration (Druid) Casting Time: Action Range: Self Components: V, S
    Duration: Concentration, up to 1 hour An aura extends from you in a 10-foot Emanation for the duration.
    The aura prevents creatures other than Constructs and Undead from passing or reaching through it.
    An affected creature can cast spells or make attacks with Ranged or Reach weapons through the barrier.
    If you move so that an affected creature is forced to pass through the barrier, the spell ends.'
  sha256: 32f5100cc807ff881d5a7ef29096c922537e1bdbcd03628a36cab4a704029c5f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Antilife Shell

<!-- SRD page 109 -->

Level 5 Abjuration (Druid)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 1 hour\
An aura extends from you in a 10-foot Emanation\
for the duration. The aura prevents creatures other\
than Constructs and Undead from passing or reach-\
ing through it. An affected creature can cast spells\
or make attacks with Ranged or Reach weapons\
through the barrier.\
If you move so that an affected creature is forced\
to pass through the barrier, the spell ends.\

Original layout: [[../../srd-5.2.1/pages/109|page 109]].
