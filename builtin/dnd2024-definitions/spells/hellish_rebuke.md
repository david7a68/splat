---
splat_schema: 1
splat_id: e8e788a3-9823-5375-99ad-87e814401cce
splat_kind: spell
splat_key: dnd2024:spells/hellish_rebuke
splat_name: Hellish Rebuke
splat_origin:
  key: dnd2024:reference/spell/hellish_rebuke
splat_sources:
- title: SRD 5.2.1
  locator: pp. 140
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: evocation
  classes:
  - warlock
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Reaction, which you take in response to taking damage from a creature that you can see
    within 60 feet of yourself
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/hellish_rebuke
  implementation: dnd2024:spell_hellish_rebuke
  parameters:
    targets:
      kind: creature
      identity: triggering_damage_source
      visible: true
      maximum_distance_feet: 60
    save:
      ability: dexterity
      dc:
        binding: save_dc
      failure:
        damage:
          dice:
            count: &id001
              op: add
              args:
              - 2
              - op: multiply
                args:
                - 1
                - op: subtract
                  args:
                  - binding: cast_level
                  - 1
            sides: 10
            add: 0
          type: fire
      success:
        damage:
          dice:
            count: *id001
            sides: 10
            add: 0
          type: fire
        damage_fraction:
          numerator: 1
          denominator: 2
splat_contracts:
- id: dnd2024:spell_hellish_rebuke
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
      required: false
      type: choice
    trigger:
      required: false
      type: event
  outputs:
  - effects
  - damage
  events:
  - spell.cast_completed
  - damage.applied
  - save.before
  - save.after
  - spell.cast_pending
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - saving_throws
splat_conversion:
  status: specified
  category: spell
  source_sha256: cc55337ae6aafa2dcfb7929b9f707f67e98f1f3c076204588e179da00f7ccbf3
  source_path: builtin/srd-5.2.1/spell/hellish_rebuke.md
  pages:
  - 140
  engine_gaps: &id002
  - dnd2024:spell_hellish_rebuke
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 99c4ad23f03b788ec502754291041c20b3505dbadba864f160fa680a63d2cf6b
splat_clauses:
- key: source_rules
  text: '# Hellish Rebuke Level 1 Evocation (Warlock) Casting Time: Reaction, which you take in response
    to taking damage from a creature that you can see within 60 feet of yourself Range: 60 feet Components:
    V, S Duration: Instantaneous The creature that damaged you is momentarily surrounded by green flames.
    It makes a Dexterity saving throw, taking 2d10 Fire damage on a failed save or half as much damage
    on a successful one. Using a Higher-Level Spell Slot. The damage increases by 1d10 for each spell
    slot level above 1.'
  sha256: 55b7b9324c2670a51c3088f681660d87d118b302eb36f7fc01ce8a0a3fbfd233
  status: contract_specified
splat_requires_mechanics: *id002
---

# Hellish Rebuke

<!-- SRD page 140 -->

Level 1 Evocation (Warlock)\
Casting Time: Reaction, which you take in response to\
taking damage from a creature that you can see within\
60 feet of yourself\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous\
The creature that damaged you is momentarily sur-\
rounded by green flames. It makes a Dexterity sav-\
ing throw, taking 2d10 Fire damage on a failed save\
or half as much damage on a successful one.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d10 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/140|page 140]].
