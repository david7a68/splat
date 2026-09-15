---
splat_schema: 1
splat_id: 62374024-138f-53e9-ac03-6d7e614f524c
splat_kind: spell
splat_key: dnd2024:spells/awaken
splat_name: Awaken
splat_origin:
  key: dnd2024:reference/spell/awaken
splat_sources:
- title: SRD 5.2.1
  locator: pp. 111, 112
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: transmutation
  classes:
  - bard
  - druid
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: 8 hours
  ritual: false
  range:
    mode: touch
  material:
    description: an agate worth 1,000+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/awaken
  implementation: dnd2024:spell_awaken
  parameters:
    targeting:
      touch: true
      options:
      - kind: creature
        types:
        - beast
        - plant
        intelligence_max: 3
      - kind: natural_plant
        creature: false
    grant:
      intelligence: 10
      speaks_language:
        choice: caster_known_language
        count: 1
    natural_plant:
      becomes_type: plant
      movable_parts:
      - limbs
      - roots
      - vines
      - creepers
      senses: human_equivalent
      statistics:
        chooser: gm
        type: definition
        constraint: appropriate_awakened_plant
    charm:
      condition: charmed
      source: caster
      days: 30
      ends_on_damage_by:
      - caster
      - caster_allies
      after_end_attitude:
        chooser: awakened_creature
        type: choice
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_awaken
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
    forms:
      type: definition_list
      required: true
    choices:
      type: choice
      required: true
    campaign:
      type: campaign
      required: true
  outputs:
  - effects
  - damage_results
  - participants
  events:
  - spell.cast_completed
  - damage.applied
  - time.elapsed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - conditions
  - participant_lifecycle
  - forms
  - campaign_state
  - perception
splat_conversion:
  status: specified
  category: spell
  source_sha256: 9d4a140bbea6a34107521bddb6fd4ed3025f837abfe92df8b00770e2cf820bab
  source_path: builtin/srd-5.2.1/spell/awaken.md
  pages:
  - 111
  - 112
  engine_gaps: &id001
  - dnd2024:spell_awaken
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 297b6a9e2e34ef1c46db48c35b6f00e8a8c6c655c5d32f578e95b21938f4f0c8
splat_clauses:
- key: source_rules
  text: '# Awaken Level 5 Transmutation (Bard, Druid) Casting Time: 8 hours Range: Touch Components: V,
    S, M (an agate worth 1,000+ GP, which the spell consumes) Duration: Instantaneous You spend the casting
    time tracing magical pathways within a precious gemstone, and then touch the target. The target must
    be either a Beast or Plant creature with an Intelligence of 3 or less or a natural plant that isn’t
    a creature. The target gains an Intelligence of 10 and the ability to speak one language you know.
    If the target is a natural plant, it becomes a Plant creature and gains the ability to move its limbs,
    roots, vines, creepers, and so forth, and it gains senses similar to a human’s. The GM chooses statistics
    appropriate for the awakened Plant, such as the statistics for the Awakened Shrub or Awakened Tree
    in “Monsters.” The awakened target has the Charmed condition for 30 days or until you or your allies
    deal damage to it. When that condition ends, the awakened creature chooses its attitude toward you.'
  sha256: acaddec31d806c5e50b18197f12f89d3f569e705bbb6603785a67ee7fc556483
  status: contract_specified
splat_requires_mechanics: *id001
---

# Awaken

<!-- SRD page 111 -->

Level 5 Transmutation (Bard, Druid)\
Casting Time: 8 hours\
Range: Touch\
Components: V, S, M (an agate worth 1,000+ GP, which\
the spell consumes)\
Duration: Instantaneous\
You spend the casting time tracing magical path-\
ways within a precious gemstone, and then touch\
the target. The target must be either a Beast or\
Plant creature with an Intelligence of 3 or less or a\
natural plant that isn’t a creature. The target gains\

<!-- SRD page 112 -->

an Intelligence of 10 and the ability to speak one\
language you know. If the target is a natural plant,\
it becomes a Plant creature and gains the ability to\
move its limbs, roots, vines, creepers, and so forth,\
and it gains senses similar to a human’s. The GM\
chooses statistics appropriate for the awakened\
Plant, such as the statistics for the Awakened\
Shrub or Awakened Tree in “Monsters.”\
The awakened target has the Charmed condition\
for 30 days or until you or your allies deal damage\
to it. When that condition ends, the awakened crea-\
ture chooses its attitude toward you.\

Original layout: [[../../srd-5.2.1/pages/111|page 111]], [[../../srd-5.2.1/pages/112|page 112]].
