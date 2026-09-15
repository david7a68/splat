---
splat_schema: 1
splat_id: 05638e36-0859-546d-99bc-47ed8f8331e2
splat_kind: spell
splat_key: dnd2024:spells/locate_animals_or_plants
splat_name: Locate Animals or Plants
splat_origin:
  key: dnd2024:reference/spell/locate_animals_or_plants
splat_sources:
- title: SRD 5.2.1
  locator: pp. 144
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: divination
  classes:
  - bard
  - druid
  - ranger
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action or Ritual
  ritual: true
  range:
    mode: self
  material:
    description: fur from a bloodhound
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/locate_animals_or_plants
  implementation: dnd2024:spell_locate_animals_or_plants
  parameters:
    query:
      selection: named_or_described_kind
      categories:
      - beast
      - plant_creature
      - nonmagical_plant
      radius_miles: 5
      match: closest
      output:
      - direction
      - distance
      requires_match_present: true
splat_contracts:
- id: dnd2024:spell_locate_animals_or_plants
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
  source_sha256: 685bdb973f85c5ae299f980ee5839a2e5d183e981dbc4fa7a68927122aa27294
  source_path: builtin/srd-5.2.1/spell/locate_animals_or_plants.md
  pages:
  - 144
  engine_gaps: &id001
  - dnd2024:spell_locate_animals_or_plants
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 43266684943f150fdaffafa65783b13b3fb8256f421072894dab3bf5429d2924
splat_clauses:
- key: source_rules
  text: '# Locate Animals or Plants Level 2 Divination (Bard, Druid, Ranger) Casting Time: Action or Ritual
    Range: Self Components: V, S, M (fur from a bloodhound) Duration: Instantaneous Describe or name a
    specific kind of Beast, Plant creature, or nonmagical plant. You learn the direction and distance
    to the closest creature or plant of that kind within 5 miles, if any are present.'
  sha256: 1a88f8fd5e6e41a536bb48e9e2b5f4ca53ab4540d7a59fee0f7ec91659e32a05
  status: contract_specified
splat_requires_mechanics: *id001
---

# Locate Animals or Plants

<!-- SRD page 144 -->

Level 2 Divination (Bard, Druid, Ranger)\
Casting Time: Action or Ritual\
Range: Self\
Components: V, S, M (fur from a bloodhound)\
Duration: Instantaneous\
Describe or name a specific kind of Beast, Plant\
creature, or nonmagical plant. You learn the direc-\
tion and distance to the closest creature or plant of\
that kind within 5 miles, if any are present.\

Original layout: [[../../srd-5.2.1/pages/144|page 144]].
