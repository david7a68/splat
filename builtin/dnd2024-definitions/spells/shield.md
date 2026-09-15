---
splat_schema: 1
splat_id: 33e0cba6-30a4-5de9-98df-c9152ea6c596
splat_kind: spell
splat_key: dnd2024:spells/shield
splat_name: Shield
splat_origin:
  key: dnd2024:reference/spell/shield
splat_sources:
- title: SRD 5.2.1
  locator: pp. 161, 162
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: abjuration
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: 1 round
  casting_time: Reaction, which you take when you are hit by an attack roll or targeted by the Magic Missile
    spell
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/shield
  implementation: dnd2024:spell_shield
  parameters:
    targets:
      kind: caster
    protection:
      until: caster_next_turn_start
      ac_bonus: 5
      includes_triggering_attack: true
      no_damage_from_spell:
        key: dnd2024:spells/magic_missile
splat_contracts:
- id: dnd2024:spell_shield
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    choices:
      required: false
      type: choice
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - spell.cast_completed
  - attack.roll
  - attack.hit
  - turn.start
  - spell.cast_pending
  capabilities:
  - spellcasting
  - effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: 342af40118c34eb8039b9589c1c7ea9937df0a912fa91aa7a819e6237db91e9e
  source_path: builtin/srd-5.2.1/spell/shield.md
  pages:
  - 161
  - 162
  engine_gaps: &id001
  - dnd2024:spell_shield
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: a161bc2dbf2b6f126dd4c2c6da445092cd1c4834ca84c27dc1e4cf15096bf16d
splat_clauses:
- key: source_rules
  text: '# Shield Level 1 Abjuration (Sorcerer, Wizard) Casting Time: Reaction, which you take when you
    are hit by an attack roll or targeted by the Magic Missile spell Range: Self Components: V, S Duration:
    1 round An imperceptible barrier of magical force protects you. Until the start of your next turn,
    you have a +5 bonus to AC, including against the triggering attack, and you take no damage from Magic
    Missile.'
  sha256: 8d1af2e8fecdd1712b3501c9d7662fbc92d3f6bf6ffbf48991d77f210485e5b0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Shield

<!-- SRD page 161 -->

Level 1 Abjuration (Sorcerer, Wizard)\
Casting Time: Reaction, which you take when you are\
hit by an attack roll or targeted by the Magic Missile\
spell\
Range: Self\
Components: V, S\
Duration: 1 round\
An imperceptible barrier of magical force protects\
you. Until the start of your next turn, you have a +5\

<!-- SRD page 162 -->

bonus to AC, including against the triggering attack,\
and you take no damage from Magic Missile.\

Original layout: [[../../srd-5.2.1/pages/161|page 161]], [[../../srd-5.2.1/pages/162|page 162]].
