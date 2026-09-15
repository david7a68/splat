---
splat_schema: 1
splat_id: 5cf2e046-608a-5241-aebe-1cc8b112d6e6
splat_kind: spell
splat_key: dnd2024:spells/clone
splat_name: Clone
splat_origin:
  key: dnd2024:reference/spell/clone
splat_sources:
- title: SRD 5.2.1
  locator: pp. 115, 116
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: necromancy
  classes:
  - wizard
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
    description: a diamond worth 1,000+ GP, which the spell consumes, and a sealable vessel worth 2,000+
      GP that is large enough to hold the creature being cloned
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/clone
  implementation: dnd2024:spell_clone
  parameters:
    targeting:
      touch: true
      options:
      - kind: creature
      - kind: flesh_sample
        volume_min_cubic_inches: 1
    duplicate:
      location: component_vessel
      maturity_days: 120
      age_choice:
      - same_as_original
      - younger
      before_transfer: inert
      preservation:
        duration: indefinite
        requires: vessel_undisturbed
    soul_transfer:
      event: original.died
      requires:
      - clone_mature
      - soul_free
      - soul_willing
      destination: clone
      preserved:
      - physical_identity
      - personality
      - memories
      - abilities
      equipment: none
      original_remains:
        inert: true
        revival_possible: false
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_clone
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
    objects:
      type: object_list
      required: true
    choices:
      type: choice
      required: true
    campaign:
      type: campaign
      required: true
  outputs:
  - effects
  events:
  - spell.cast_completed
  - time.elapsed
  - health.changed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - campaign_state
splat_conversion:
  status: specified
  category: spell
  source_sha256: a55c4c417e31a04ae46b34297c5a2e23487e270841cfe92c3ec5b461d14b330b
  source_path: builtin/srd-5.2.1/spell/clone.md
  pages:
  - 115
  - 116
  engine_gaps: &id001
  - dnd2024:spell_clone
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: d746389041ab8b2dc14abce6f7218c0e381c8ddfe0a3cb0f27722beea88a14b7
splat_clauses:
- key: source_rules
  text: '# Clone Level 8 Necromancy (Wizard) Casting Time: 1 hour Range: Touch Components: V, S, M (a
    diamond worth 1,000+ GP, which the spell consumes, and a sealable vessel worth 2,000+ GP that is large
    enough to hold the creature being cloned) Duration: Instantaneous You touch a creature or at least
    1 cubic inch of its flesh. An inert duplicate of that creature forms inside the vessel used in the
    spell’s casting and finishes growing after 120 days; you choose whether the finished clone is the
    same age as the creature or younger. The clone remains inert and endures indefinitely while its vessel
    remains undisturbed. If the original creature dies after the clone finishes forming, the creature’s
    soul transfers to the clone if the soul is free and willing to return. The clone is physically identical
    to the original and has the same personality, memories, and abilities, but none of the original’s
    equipment. The creature’s original remains, if any, become inert and can’t be revived, since the creature’s
    soul is elsewhere.'
  sha256: 793bab75e11ae7b622b564c73292638185a1055f897c191275be0666b6fefab9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Clone

<!-- SRD page 115 -->

Level 8 Necromancy (Wizard)\
Casting Time: 1 hour\
Range: Touch\
Components: V, S, M (a diamond worth 1,000+ GP,\
which the spell consumes, and a sealable vessel worth\
2,000+ GP that is large enough to hold the creature\
being cloned)\
Duration: Instantaneous\

<!-- SRD page 116 -->

You touch a creature or at least 1 cubic inch of its\
flesh. An inert duplicate of that creature forms\
inside the vessel used in the spell’s casting and fin-\
ishes growing after 120 days; you choose whether\
the finished clone is the same age as the creature or\
younger. The clone remains inert and endures indef-\
initely while its vessel remains undisturbed.\
If the original creature dies after the clone fin-\
ishes forming, the creature’s soul transfers to the\
clone if the soul is free and willing to return. The\
clone is physically identical to the original and has\
the same personality, memories, and abilities, but\
none of the original’s equipment. The creature’s\
original remains, if any, become inert and can’t be\
revived, since the creature’s soul is elsewhere.\

Original layout: [[../../srd-5.2.1/pages/115|page 115]], [[../../srd-5.2.1/pages/116|page 116]].
