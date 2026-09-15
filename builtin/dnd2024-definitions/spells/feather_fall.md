---
splat_schema: 1
splat_id: f90e1217-fb74-504e-981f-d809e6574934
splat_kind: spell
splat_key: dnd2024:spells/feather_fall
splat_name: Feather Fall
splat_origin:
  key: dnd2024:reference/spell/feather_fall
splat_sources:
- title: SRD 5.2.1
  locator: pp. 130
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: transmutation
  classes:
  - bard
  - sorcerer
  - wizard
  components:
  - verbal
  - material
  concentration: false
  duration: 1 minute
  casting_time: Reaction, which you take when you or a creature you can see within 60 feet of you falls
  ritual: false
  range:
    feet: 60
  material:
    description: a small feather or piece of down
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/feather_fall
  implementation: dnd2024:spell_feather_fall
  parameters:
    target:
      kind: creature
      count:
        maximum: 5
      visible_to_caster: false
      falling: true
    descent_feet_per_round: 60
    landing:
      before_spell_end: true
      fall_damage: 0
      end_spell_on_landed_target: true
splat_contracts:
- id: dnd2024:spell_feather_fall
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - descent_effect
  - prevented_fall_damage
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
splat_conversion:
  status: specified
  category: spell
  source_sha256: 71e3e7bba0dfba6b2a5f87abc6ca576907b5c295dc45a3b9a21cf5f1716f1b9b
  source_path: builtin/srd-5.2.1/spell/feather_fall.md
  pages:
  - 130
  engine_gaps: &id001
  - dnd2024:spell_feather_fall
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: b48a3dc5cab689b7345958abaf590dfb0086c8defad82bbebc68d7a3b255ac29
splat_clauses:
- key: source_rules
  text: '# Feather Fall Level 1 Transmutation (Bard, Sorcerer, Wizard) Casting Time: Reaction, which you
    take when you or a creature you can see within 60 feet of you falls Range: 60 feet Components: V,
    M (a small feather or piece of down) Duration: 1 minute Choose up to five falling creatures within
    range. A falling creature’s rate of descent slows to 60 feet per round until the spell ends. If a
    creature lands before the spell ends, the creature takes no damage from the fall, and the spell ends
    for that creature.'
  sha256: 477fcc902cd1f8436aa96b508752105eaacf5901f1fef329f45924c7d9e68f69
  status: contract_specified
splat_requires_mechanics: *id001
---

# Feather Fall

<!-- SRD page 130 -->

Level 1 Transmutation (Bard, Sorcerer, Wizard)\
Casting Time: Reaction, which you take when you or a\
creature you can see within 60 feet of you falls\
Range: 60 feet\
Components: V, M (a small feather or piece of down)\
Duration: 1 minute\
Choose up to five falling creatures within range. A\
falling creature’s rate of descent slows to 60 feet per\
round until the spell ends. If a creature lands before\
the spell ends, the creature takes no damage from\
the fall, and the spell ends for that creature.\

Original layout: [[../../srd-5.2.1/pages/130|page 130]].
