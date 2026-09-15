---
splat_schema: 1
splat_id: 6c2d30e1-4f6e-58d9-969d-cc0b02db8430
splat_kind: spell
splat_key: dnd2024:spells/mislead
splat_name: Mislead
splat_origin:
  key: dnd2024:reference/spell/mislead
splat_sources:
- title: SRD 5.2.1
  locator: pp. 150
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: illusion
  classes:
  - bard
  - warlock
  - wizard
  components:
  - somatic
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/mislead
  implementation: dnd2024:spell_mislead
  parameters:
    targeting:
      kind: self
    invisible:
      condition: invisible
      duration: spell_duration
      ends_immediately_after:
      - caster_attack_roll
      - caster_damage_dealt
      - caster_spell_cast
      double_ends_with_invisibility: false
    double:
      spawn_location: caster_space_at_cast
      duration: spell_duration
      intangible: true
      invulnerable: true
      control:
        economy: magic_action
        movement_max:
          op: multiply
          args:
          - 2
          - stat: caster_speed
        behaviors:
        - gesture
        - speak
        - caster_chosen_behavior
      shared_senses:
        channels:
        - vision
        - hearing
        origin: double_location
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: hour
splat_contracts:
- id: dnd2024:spell_mislead
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
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
  - damage.applied
  - attack.hit
  - attack.roll_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - conditions
  - perception
  - controlled_participants
splat_conversion:
  status: specified
  category: spell
  source_sha256: 15aab9723317bbb4827b5676af814ee8c8d63fd3745195d3b1d4e611ffdd6f03
  source_path: builtin/srd-5.2.1/spell/mislead.md
  pages:
  - 150
  engine_gaps: &id001
  - dnd2024:spell_mislead
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: b5bd7feec571f59f86941ce72f558f711c4f222a4ee2a84971e209e22defffed
splat_clauses:
- key: source_rules
  text: '# Mislead Level 5 Illusion (Bard, Warlock, Wizard) Casting Time: Action Range: Self Components:
    S Duration: Concentration, up to 1 hour You gain the Invisible condition at the same time that an
    illusory double of you appears where you are standing. The double lasts for the duration, but the
    invisibility ends immediately after you make an attack roll, deal damage, or cast a spell. As a Magic
    action, you can move the illusory double up to twice your Speed and make it gesture, speak, and behave
    in whatever way you choose. It is intangible and invulnerable. You can see through its eyes and hear
    through its ears as if you were located where it is.'
  sha256: a69fe6c065e61be5425e7098d98d139177ff331bca9655118b1ae37e35680503
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mislead

<!-- SRD page 150 -->

Level 5 Illusion (Bard, Warlock, Wizard)\
Casting Time: Action\
Range: Self\
Components: S\
Duration: Concentration, up to 1 hour\
You gain the Invisible condition at the same time\
that an illusory double of you appears where you\
are standing. The double lasts for the duration, but\
the invisibility ends immediately after you make an\
attack roll, deal damage, or cast a spell.\
As a Magic action, you can move the illusory dou-\
ble up to twice your Speed and make it gesture,\
speak, and behave in whatever way you choose. It is\
intangible and invulnerable.\
You can see through its eyes and hear through its\
ears as if you were located where it is.\

Original layout: [[../../srd-5.2.1/pages/150|page 150]].
