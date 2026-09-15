---
splat_schema: 1
splat_id: 10ccec18-2354-5c74-9132-2d99af8798d3
splat_kind: rule
splat_key: dnd2024:glossary/dead
splat_name: Dead
splat_origin:
  key: dnd2024:reference/glossary/dead
splat_sources:
- title: SRD 5.2.1
  locator: pp. 180
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/dead
  parameters:
    hit_points: 0
    healing_requires_prior_magical_revival: true
    revival:
      spirit_knows_caster: true
      spirit_can_refuse: true
      requires_spirit_return: true
      hit_points: revival_effect_defined
      ongoing_effects_return:
        kinds:
        - condition
        - magical_contagion
        - curse
        only_if_duration_ongoing: true
        unless_revival_states_otherwise: true
      exhaustion_levels:
        op: max
        args:
        - 0
        - op: subtract
          args:
          - var: death_exhaustion_levels
          - 1
    attunements_ended: true
  implementation: dnd2024:glossary_dead
splat_contracts:
- id: dnd2024:glossary_dead
  inputs:
    subject:
      type: participant
      required: true
    revival_effect:
      required: false
      type: effect
    spirit_accepts:
      required: false
      type: boolean
  outputs:
  - death_state
  - revival_result
  events:
  - creature.died
  - revival.pending
  capabilities:
  - death
  - hit_points
  - effects
  - attunement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 5a7164a1ad47c3d734e66fad6ced570a711d930a2a688a1ad8da0fb719f161e1
  source_path: builtin/srd-5.2.1/glossary/dead.md
  pages:
  - 180
  engine_gaps: &id001
  - dnd2024:glossary_dead
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: af24c9226fbc39ad836706d91e4fb049948f8645e62b22c57ee4322a109f9b4a
splat_clauses:
- key: source_rules
  text: '# Dead A dead creature has no Hit Points and can’t regain them unless it is first revived by
    magic such as the Raise Dead or Revivify spell. When such a spell is cast, the spirit knows who is
    casting it and can refuse. The spirit of a dead creature has left the body and departed for the Outer
    Planes, and reviving the creature requires calling the spirit back. If the creature returns to life,
    the revival effect determines the creature’s current Hit Points. Unless otherwise stated, the creature
    returns to life with any conditions, magical contagions, or curses that were affecting it at death
    if the durations of those effects are still ongoing. If the creature died with any Exhaustion levels,
    it returns with 1 fewer level. If the creature had Attunement to one or more magic items, it is no
    longer attuned to them.'
  sha256: d3685f457cca5371088adf5fddfaf176b451b3e8f32f1b8d014b86b8a9fe3df4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dead

<!-- SRD page 180 -->

A dead creature has no Hit Points and can’t regain\
them unless it is first revived by magic such as the\
Raise Dead or Revivify spell. When such a spell is\
cast, the spirit knows who is casting it and can re-\
fuse. The spirit of a dead creature has left the body\
and departed for the Outer Planes, and reviving the\
creature requires calling the spirit back.\
If the creature returns to life, the revival effect\
determines the creature’s current Hit Points. Un-\
less otherwise stated, the creature returns to life\
with any conditions, magical contagions, or curses\
that were affecting it at death if the durations of\
those effects are still ongoing. If the creature died\
with any Exhaustion levels, it returns with 1 fewer\
level. If the creature had Attunement to one or more\
magic items, it is no longer attuned to them.\

Original layout: [[../../srd-5.2.1/pages/180|page 180]].
