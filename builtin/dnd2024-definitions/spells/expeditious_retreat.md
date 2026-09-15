---
splat_schema: 1
splat_id: 4dfeb27b-06b6-55e8-af6d-e8fe7a8f2948
splat_kind: spell
splat_key: dnd2024:spells/expeditious_retreat
splat_name: Expeditious Retreat
splat_origin:
  key: dnd2024:reference/spell/expeditious_retreat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 129
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: transmutation
  classes:
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Bonus Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/expeditious_retreat
  implementation: dnd2024:spell_expeditious_retreat
  parameters:
    targeting:
      kind: self
    on_cast:
      action: dash
      included_in_casting_economy: true
    grant:
      action: dash
      economy: bonus_action
      duration: spell_duration
    spell_lifetime:
      concentration: true
      maximum:
        value: 10
        unit: minute
splat_contracts:
- id: dnd2024:spell_expeditious_retreat
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
  outputs:
  - effects
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: b5d288e0df4d39c4d6bd1e0aa71df760158fd38f95338167255dd776472d7db1
  source_path: builtin/srd-5.2.1/spell/expeditious_retreat.md
  pages:
  - 129
  engine_gaps: &id001
  - dnd2024:spell_expeditious_retreat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 0e25460b2ff9ddbc4ea642c83aaf2e314c6727e10a2029843fd191df54c5c6b9
splat_clauses:
- key: source_rules
  text: '# Expeditious Retreat Level 1 Transmutation (Sorcerer, Warlock, Wizard) Casting Time: Bonus Action
    Range: Self Components: V, S Duration: Concentration, up to 10 minutes You take the Dash action, and
    until the spell ends, you can take that action again as a Bonus Action.'
  sha256: 556db4fda8c5488d215de4071eee5c5ac3af14f76173dce77159fc5fe52b0a4d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Expeditious Retreat

<!-- SRD page 129 -->

Level 1 Transmutation (Sorcerer, Warlock, Wizard)\
Casting Time: Bonus Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 10 minutes\
You take the Dash action, and until the spell ends,\
you can take that action again as a Bonus Action.\

Original layout: [[../../srd-5.2.1/pages/129|page 129]].
