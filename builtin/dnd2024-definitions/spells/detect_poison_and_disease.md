---
splat_schema: 1
splat_id: d53fd494-856f-5144-b5a3-f8dc0a4a977c
splat_kind: spell
splat_key: dnd2024:spells/detect_poison_and_disease
splat_name: Detect Poison and Disease
splat_origin:
  key: dnd2024:reference/spell/detect_poison_and_disease
splat_sources:
- title: SRD 5.2.1
  locator: pp. 123
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: divination
  classes:
  - cleric
  - druid
  - paladin
  - ranger
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action or Ritual
  ritual: true
  range:
    mode: self
  material:
    description: a yew leaf
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/detect_poison_and_disease
  implementation: dnd2024:spell_detect_poison_and_disease
  parameters:
    sense:
      radius_feet: 30
      origin: caster
      duration: spell_duration
      categories:
      - poison
      - poisonous_creature
      - venomous_creature
      - magical_contagion
      outputs:
      - location
      - kind
      barrier_blocks:
      - materials:
        - stone
        - dirt
        - wood
        minimum_thickness_feet: 1
      - material: metal
        minimum_thickness_inches: 1
      - material: lead
        any_thin_sheet: true
splat_contracts:
- id: dnd2024:spell_detect_poison_and_disease
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    choices:
      required: false
      type: choice
  outputs:
  - effects
  - knowledge
  events:
  - spell.cast_completed
  - movement.completed
  capabilities:
  - spellcasting
  - effects
  - spatial_relationships
  - knowledge
splat_conversion:
  status: specified
  category: spell
  source_sha256: 6c95779e52fda9595e904bdc66473340f894d23a2fb389d5617a092856256412
  source_path: builtin/srd-5.2.1/spell/detect_poison_and_disease.md
  pages:
  - 123
  engine_gaps: &id001
  - dnd2024:spell_detect_poison_and_disease
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 1e6ca2155e2ef463e7af349e03b47c228aac74b621ea47fed0a8cfdc23d69f0b
splat_clauses:
- key: source_rules
  text: '# Detect Poison and Disease Level 1 Divination (Cleric, Druid, Paladin, Ranger) Casting Time:
    Action or Ritual Range: Self Components: V, S, M (a yew leaf) Duration: Concentration, up to 10 minutes
    For the duration, you sense the location of poisons, poisonous or venomous creatures, and magical
    contagions within 30 feet of yourself. You sense the kind of poison, creature, or contagion in each
    case. The spell is blocked by 1 foot of stone, dirt, or wood; 1 inch of metal; or a thin sheet of
    lead.'
  sha256: 601a84ed3d9376d19ecd09ae94018ff9b3276ff6a3ef4bbcb8f900aed74a58d0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Detect Poison and Disease

<!-- SRD page 123 -->

Level 1 Divination (Cleric, Druid, Paladin, Ranger)\
Casting Time: Action or Ritual\
Range: Self\
Components: V, S, M (a yew leaf)\
Duration: Concentration, up to 10 minutes\
For the duration, you sense the location of poisons,\
poisonous or venomous creatures, and magical\
contagions within 30 feet of yourself. You sense the\
kind of poison, creature, or contagion in each case.\
The spell is blocked by 1 foot of stone, dirt, or\
wood; 1 inch of metal; or a thin sheet of lead.\

Original layout: [[../../srd-5.2.1/pages/123|page 123]].
