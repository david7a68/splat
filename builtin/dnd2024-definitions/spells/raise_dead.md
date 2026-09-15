---
splat_schema: 1
splat_id: 0670f084-a1af-5192-ad4e-b7804324279a
splat_kind: spell
splat_key: dnd2024:spells/raise_dead
splat_name: Raise Dead
splat_origin:
  key: dnd2024:reference/spell/raise_dead
splat_sources:
- title: SRD 5.2.1
  locator: pp. 157
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: necromancy
  classes:
  - bard
  - cleric
  - paladin
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: 1 hour
  ritual: false
  range:
    mode: touch
  material:
    description: a diamond worth 500+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/raise_dead
  implementation: dnd2024:spell_raise_dead
  parameters:
    targeting:
      kind: dead_creature
      touch: true
      dead_max_days: 10
      excluded_type_at_death: undead
    failure_if: missing_survival_essential_body_parts_or_organs
    revival:
      hit_points: 1
      neutralizes: poisons_present_at_death
      closes: all_mortal_wounds
      restores_missing_parts: false
    ordeal:
      rolls: d20_tests
      modifier: -4
      recovery:
        event: target.long_rest_finished
        reduction_in_penalty: 1
        stop_at: 0
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_raise_dead
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
    campaign:
      type: campaign
      required: true
  outputs:
  - effects
  events:
  - spell.cast_completed
  - rest.completed
  capabilities:
  - spellcasting
  - effects
  - campaign_state
  - rest_lifecycle
splat_conversion:
  status: specified
  category: spell
  source_sha256: ea49512c4b9d5a20608b7f14c83a23d6717a897f48d0718064792d90db7bb4e1
  source_path: builtin/srd-5.2.1/spell/raise_dead.md
  pages:
  - 157
  engine_gaps: &id001
  - dnd2024:spell_raise_dead
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: dbf34ce26768320c288fd3eb749a805094dd77318446c8c66b26dac18c1c99d9
splat_clauses:
- key: source_rules
  text: '# Raise Dead Level 5 Necromancy (Bard, Cleric, Paladin) Casting Time: 1 hour Range: Touch Components:
    V, S, M (a diamond worth 500+ GP, which the spell consumes) Duration: Instantaneous With a touch,
    you revive a dead creature if it has been dead no longer than 10 days and it wasn’t Undead when it
    died. The creature returns to life with 1 Hit Point. This spell also neutralizes any poisons that
    affected the creature at the time of death. This spell closes all mortal wounds, but it doesn’t restore
    missing body parts. If the creature is lacking body parts or organs integral for its survival— its
    head, for instance—the spell automatically fails. Coming back from the dead is an ordeal. The target
    takes a -4 penalty to D20 Tests. Every time the target finishes a Long Rest, the penalty is reduced
    by 1 until it becomes 0.'
  sha256: e691a9a3aad17f74832406389c703083427ba4aff7d7c7dda77e9fab7dc26241
  status: contract_specified
splat_requires_mechanics: *id001
---

# Raise Dead

<!-- SRD page 157 -->

Level 5 Necromancy (Bard, Cleric, Paladin)\
Casting Time: 1 hour\
Range: Touch\
Components: V, S, M (a diamond worth 500+ GP, which\
the spell consumes)\
Duration: Instantaneous\
With a touch, you revive a dead creature if it has\
been dead no longer than 10 days and it wasn’t Un-\
dead when it died.\
The creature returns to life with 1 Hit Point. This\
spell also neutralizes any poisons that affected the\
creature at the time of death.\
This spell closes all mortal wounds, but it doesn’t\
restore missing body parts. If the creature is lack-\
ing body parts or organs integral for its survival—\
its head, for instance—the spell automatically fails.\
Coming back from the dead is an ordeal. The tar-\
get takes a −4 penalty to D20 Tests. Every time the\
target finishes a Long Rest, the penalty is reduced\
by 1 until it becomes 0.\

Original layout: [[../../srd-5.2.1/pages/157|page 157]].
