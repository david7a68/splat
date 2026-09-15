---
splat_schema: 1
splat_id: c60032dc-0ab4-5660-afd7-386b0c6fc57d
splat_kind: spell
splat_key: dnd2024:spells/resistance
splat_name: Resistance
splat_origin:
  key: dnd2024:reference/spell/resistance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 158
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: abjuration
  classes:
  - cleric
  - druid
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    mode: touch
splat_rule:
  mechanic: dnd2024:spells/resistance
  implementation: dnd2024:spell_resistance
  parameters:
    targeting:
      kind: creature
      count: 1
      willing: true
      touch: true
    damage_type_choice:
    - acid
    - bludgeoning
    - cold
    - fire
    - lightning
    - necrotic
    - piercing
    - poison
    - radiant
    - slashing
    - thunder
    reduction:
      event: target.takes_chosen_damage_type
      total_damage_reduced_by: 1d4
      max_benefits_per_creature_per_turn: 1
      duration: spell_duration
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_resistance
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
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
splat_conversion:
  status: specified
  category: spell
  source_sha256: aefedc18449e1f7c0f6a213b7907ae7eb95ae9f39f7dd52ffde9afa5a53a5d37
  source_path: builtin/srd-5.2.1/spell/resistance.md
  pages:
  - 158
  engine_gaps: &id001
  - dnd2024:spell_resistance
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: e6ff83e7624d63dc20d0446aaac58aa1f7e11914a4815e6c276cfec35df7ecba
splat_clauses:
- key: source_rules
  text: '# Resistance Abjuration Cantrip (Cleric, Druid) Casting Time: Action Range: Touch Component:
    V, S Duration: Concentration, up to 1 minute You touch a willing creature and choose a damage type:
    Acid, Bludgeoning, Cold, Fire, Lightning, Necrotic, Piercing, Poison, Radiant, Slashing, or Thunder.
    When the creature takes damage of the chosen type before the spell ends, the creature reduces the
    total damage taken by 1d4. A creature can benefit from this spell only once per turn.'
  sha256: abca9e9d12d998285d64553e27e2f13f07f03954e9c692dcaa58b30ccf0c7784
  status: contract_specified
splat_requires_mechanics: *id001
---

# Resistance

<!-- SRD page 158 -->

Abjuration Cantrip (Cleric, Druid)\
Casting Time: Action\
Range: Touch\
Component: V, S\
Duration: Concentration, up to 1 minute\
You touch a willing creature and choose a damage\
type: Acid, Bludgeoning, Cold, Fire, Lightning, Ne-\
crotic, Piercing, Poison, Radiant, Slashing, or Thun-\
der. When the creature takes damage of the chosen\
type before the spell ends, the creature reduces the\
total damage taken by 1d4. A creature can benefit\
from this spell only once per turn.\

Original layout: [[../../srd-5.2.1/pages/158|page 158]].
