---
splat_schema: 1
splat_id: 5340380a-57af-5094-9f38-02fe5c88b00b
splat_kind: spell
splat_key: dnd2024:spells/silence
splat_name: Silence
splat_origin:
  key: dnd2024:reference/spell/silence
splat_sources:
- title: SRD 5.2.1
  locator: pp. 162
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: illusion
  classes:
  - bard
  - cleric
  - ranger
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action or Ritual
  ritual: true
  range:
    feet: 120
splat_rule:
  mechanic: dnd2024:spells/silence
  implementation: dnd2024:spell_silence
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 600
    area:
      shape: sphere
      radius_feet: 20
      origin: chosen_point
      range_feet: 120
    sound:
      creation_inside: forbidden
      passage_through: forbidden
    entirely_inside:
      creatures_and_objects:
        damage_immunity: thunder
      creatures:
        condition: deafened
    spellcasting_inside:
      verbal_component: forbidden
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_silence
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    origin:
      type: location
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  events:
  - concentration.ended
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - areas
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: c6f702b46f18f4fdd8ed93a7926a8c18db1fc02666b64f12c74d52207caa68b4
  source_path: builtin/srd-5.2.1/spell/silence.md
  pages:
  - 162
  engine_gaps: &id001
  - dnd2024:spell_silence
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: f07eceffd02189ad4602cf60ce4c29d702f0fc9b43148a209839148e087df5b7
splat_clauses:
- key: source_rules
  text: '# Silence Level 2 Illusion (Bard, Cleric, Ranger) Casting Time: Action or Ritual Range: 120 feet
    Components: V, S Duration: Concentration, up to 10 minutes For the duration, no sound can be created
    within or pass through a 20-foot-radius Sphere centered on a point you choose within range. Any creature
    or object entirely inside the Sphere has Immunity to Thunder damage, and creatures have the Deafened
    condition while entirely inside it. Casting a spell that includes a Verbal component is impossible
    there.'
  sha256: bf01103eac4bf9caedb4ff3ebb74a88eb098a93b52a965cbc6eda144116f7c40
  status: contract_specified
splat_requires_mechanics: *id001
---

# Silence

<!-- SRD page 162 -->

Level 2 Illusion (Bard, Cleric, Ranger)\
Casting Time: Action or Ritual\
Range: 120 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes\
For the duration, no sound can be created within\
or pass through a 20-foot-radius Sphere centered\
on a point you choose within range. Any creature\
or object entirely inside the Sphere has Immunity\
to Thunder damage, and creatures have the Deaf-\
ened condition while entirely inside it. Casting a\
spell that includes a Verbal component is impossible\
there.\

Original layout: [[../../srd-5.2.1/pages/162|page 162]].
