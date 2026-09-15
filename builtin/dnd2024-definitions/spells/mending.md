---
splat_schema: 1
splat_id: af9bbc02-fe63-5b5f-8af3-c07cd85d8a85
splat_kind: spell
splat_key: dnd2024:spells/mending
splat_name: Mending
splat_origin:
  key: dnd2024:reference/spell/mending
splat_sources:
- title: SRD 5.2.1
  locator: pp. 148
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: transmutation
  classes:
  - bard
  - cleric
  - druid
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: 1 minute
  ritual: false
  range:
    mode: touch
  material:
    description: two lodestones
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/mending
  implementation: dnd2024:spell_mending
  parameters:
    targeting:
      kind: object
      touch: true
      break_or_tear_count: 1
      break_or_tear_max_dimension_feet: 1
    repair:
      physical: true
      damage_trace_remaining: false
      magic_item_eligible: true
      restores_magic: false
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_mending
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    objects:
      type: object_list
      required: true
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
splat_conversion:
  status: specified
  category: spell
  source_sha256: e7e75b4d8b043d412023d9f5a2762a0ed78a2165e97dc7a284be107f62990112
  source_path: builtin/srd-5.2.1/spell/mending.md
  pages:
  - 148
  engine_gaps: &id001
  - dnd2024:spell_mending
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 5d85e1cbf131632f487134bb53a20efab8709de710edf3c38ae28fb3ff70e5ba
splat_clauses:
- key: source_rules
  text: '# Mending Transmutation Cantrip (Bard, Cleric, Druid, Sorcerer, Wizard) Casting Time: 1 minute
    Range: Touch Components: V, S, M (two lodestones) Duration: Instantaneous This spell repairs a single
    break or tear in an object you touch, such as a broken chain link, two halves of a broken key, a torn
    cloak, or a leaking wineskin. As long as the break or tear is no larger than 1 foot in any dimension,
    you mend it, leaving no trace of the former damage. This spell can physically repair a magic item,
    but it can’t restore magic to such an object.'
  sha256: 7c3851d19119ad9644fe7bc9ae5839832a72714488559dbea1e214b7224d25b8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mending

<!-- SRD page 148 -->

Transmutation Cantrip (Bard, Cleric, Druid, Sorcerer,\
Wizard)\
Casting Time: 1 minute\
Range: Touch\
Components: V, S, M (two lodestones)\
Duration: Instantaneous\
This spell repairs a single break or tear in an object\
you touch, such as a broken chain link, two halves of\
a broken key, a torn cloak, or a leaking wineskin. As\
long as the break or tear is no larger than 1 foot in\
any dimension, you mend it, leaving no trace of the\
former damage.\
This spell can physically repair a magic item, but\
it can’t restore magic to such an object.\

Original layout: [[../../srd-5.2.1/pages/148|page 148]].
