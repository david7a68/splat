---
splat_schema: 1
splat_id: edf1ef04-81a2-55ba-98f0-e742a7834c1f
splat_kind: spell
splat_key: dnd2024:spells/resurrection
splat_name: Resurrection
splat_origin:
  key: dnd2024:reference/spell/resurrection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 158, 159
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: necromancy
  classes:
  - bard
  - cleric
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
    description: a diamond worth 1,000+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/resurrection
  implementation: dnd2024:spell_resurrection
  parameters:
    targets:
      kind: dead_creature
      touched: true
      dead_years_maximum: 100
      death_from_old_age: false
      undead_when_died: false
    revive:
      hp: all
      neutralize: poisons_at_time_of_death
      close: all_mortal_wounds
      restore: missing_body_parts
    target_penalty:
      rolls: d20_tests
      initial: -4
      recovery:
        event: target_long_rest_completed
        increase: 1
        maximum: 0
    caster_costly_revival:
      requires_dead_days_minimum: 365
      prohibit: cast_spells
      disadvantage: d20_tests
      until: caster_long_rest_completed
splat_contracts:
- id: dnd2024:spell_resurrection
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
      required: false
      type: choice
  outputs:
  - effects
  - health
  events:
  - spell.cast_completed
  - rest.completed
  - time.elapsed
  capabilities:
  - spellcasting
  - effects
  - roll_modifiers
  - health
splat_conversion:
  status: specified
  category: spell
  source_sha256: cf628b30365e82d88b812633179595a2ae22e2210427c02288bb40d7419c5177
  source_path: builtin/srd-5.2.1/spell/resurrection.md
  pages:
  - 158
  - 159
  engine_gaps: &id001
  - dnd2024:spell_resurrection
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 12ad3aab3dc6269407f151aacf69559822bd0b694202ae4b0398c7a19d87b33b
splat_clauses:
- key: source_rules
  text: '# Resurrection Level 7 Necromancy (Bard, Cleric) Casting Time: 1 hour Range: Touch Components:
    V, S, M (a diamond worth 1,000+ GP, which the spell consumes) Duration: Instantaneous With a touch,
    you revive a dead creature that has been dead for no more than a century, didn’t die of old age, and
    wasn’t Undead when it died. The creature returns to life with all its Hit Points. This spell also
    neutralizes any poisons that affected the creature at the time of death. This spell closes all mortal
    wounds and restores any missing body parts. Coming back from the dead is an ordeal. The target takes
    a -4 penalty to D20 Tests. Every time the target finishes a Long Rest, the penalty is reduced by 1
    until it becomes 0. Casting this spell to revive a creature that has been dead for 365 days or longer
    taxes you. Until you finish a Long Rest, you can’t cast spells again, and you have Disadvantage on
    D20 Tests.'
  sha256: 1efe5b7d76b87def5469ec3b4a4e13a62a951df37b89977377107d2f3783b28f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Resurrection

<!-- SRD page 158 -->

Level 7 Necromancy (Bard, Cleric)\
Casting Time: 1 hour\
Range: Touch\
Components: V, S, M (a diamond worth 1,000+ GP,\
which the spell consumes)\
Duration: Instantaneous\

<!-- SRD page 159 -->

With a touch, you revive a dead creature that has\
been dead for no more than a century, didn’t die of\
old age, and wasn’t Undead when it died.\
The creature returns to life with all its Hit Points.\
This spell also neutralizes any poisons that affected\
the creature at the time of death. This spell closes\
all mortal wounds and restores any missing body\
parts.\
Coming back from the dead is an ordeal. The tar-\
get takes a −4 penalty to D20 Tests. Every time the\
target finishes a Long Rest, the penalty is reduced\
by 1 until it becomes 0.\
Casting this spell to revive a creature that has\
been dead for 365 days or longer taxes you. Until\
you finish a Long Rest, you can’t cast spells again,\
and you have Disadvantage on D20 Tests.\

Original layout: [[../../srd-5.2.1/pages/158|page 158]], [[../../srd-5.2.1/pages/159|page 159]].
