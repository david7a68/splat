---
splat_schema: 1
splat_id: 0533a1be-f938-5436-908d-f50e2c3a505b
splat_kind: spell
splat_key: dnd2024:spells/divine_favor
splat_name: Divine Favor
splat_origin:
  key: dnd2024:reference/spell/divine_favor
splat_sources:
- title: SRD 5.2.1
  locator: pp. 125
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: transmutation
  classes:
  - paladin
  components:
  - verbal
  - somatic
  concentration: false
  duration: 1 minute
  casting_time: Bonus Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/divine_favor
  implementation: dnd2024:spell_divine_favor
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 60
    trigger:
      event: attack.hit
      attacker: caster
      weapon_required: true
    extra_damage:
      dice: 1d4
      type: radiant
splat_contracts:
- id: dnd2024:spell_divine_favor
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
  outputs:
  - effects
  - damage
  events:
  - attack.hit
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - attacks
  - damage
splat_conversion:
  status: specified
  category: spell
  source_sha256: b8857bd4eb83d6106a1633a7efae2b9ae8a3f8506c02a6a48a775c941e365c65
  source_path: builtin/srd-5.2.1/spell/divine_favor.md
  pages:
  - 125
  engine_gaps: &id001
  - dnd2024:spell_divine_favor
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 1c2f6f6f9c0239c42bd7599c604484951df746181376ebe4127045c5ab4721e5
splat_clauses:
- key: source_rules
  text: '# Divine Favor Level 1 Transmutation (Paladin) Casting Time: Bonus Action Range: Self Components:
    V, S Duration: 1 minute Until the spell ends, your attacks with weapons deal an extra 1d4 Radiant
    damage on a hit.'
  sha256: 66fafd9644ab28666216d3707bdd347e70d7509c1ad3019924ebb4a682b22257
  status: contract_specified
splat_requires_mechanics: *id001
---

# Divine Favor

<!-- SRD page 125 -->

Level 1 Transmutation (Paladin)\
Casting Time: Bonus Action\
Range: Self\
Components: V, S\
Duration: 1 minute\
Until the spell ends, your attacks with weapons deal\
an extra 1d4 Radiant damage on a hit.\

Original layout: [[../../srd-5.2.1/pages/125|page 125]].
