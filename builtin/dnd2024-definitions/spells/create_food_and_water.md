---
splat_schema: 1
splat_id: c2ec689e-084d-513b-80ff-c14e8b91a1dc
splat_kind: spell
splat_key: dnd2024:spells/create_food_and_water
splat_name: Create Food and Water
splat_origin:
  key: dnd2024:reference/spell/create_food_and_water
splat_sources:
- title: SRD 5.2.1
  locator: pp. 120
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: conjuration
  classes:
  - cleric
  - paladin
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 30
splat_rule:
  mechanic: dnd2024:spells/create_food_and_water
  implementation: dnd2024:spell_create_food_and_water
  parameters:
    placement:
      choice:
      - ground
      - containers
      within_range: true
    food:
      pounds: 45
      nourishing: true
      taste: bland
      appearance_choice: food_kind
      uneaten_spoils_hours: 24
    water:
      gallons: 30
      fresh: true
      clean: true
splat_contracts:
- id: dnd2024:spell_create_food_and_water
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - food_objects
  - water_quantity
  - food_expiry
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - choices
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: 8a5480ad136a07cd5255804cedbb9ea745d058cd90b93c476a42b2d7ac2dbd5e
  source_path: builtin/srd-5.2.1/spell/create_food_and_water.md
  pages:
  - 120
  engine_gaps: &id001
  - dnd2024:spell_create_food_and_water
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 91e2d8b5fb036cbd457c910f515faa9620a3611fbfdf8702ab9b53c164ca710d
splat_clauses:
- key: source_rules
  text: '# Create Food and Water Level 3 Conjuration (Cleric, Paladin) Casting Time: Action Range: 30
    feet Components: V, S Duration: Instantaneous You create 45 pounds of food and 30 gallons of fresh
    water on the ground or in containers within range—both useful in fending off the hazards of malnutrition
    and dehydration. The food is bland but nourishing and looks like a food of your choice, and the water
    is clean. The food spoils after 24 hours if uneaten.'
  sha256: 6db3427987ca45c9dd2e8505ab248c55ad91a880be542f196ac3be377be2ed4e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Create Food and Water

<!-- SRD page 120 -->

Level 3 Conjuration (Cleric, Paladin)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Instantaneous\
You create 45 pounds of food and 30 gallons of\
fresh water on the ground or in containers within\
range—both useful in fending off the hazards of\
malnutrition and dehydration. The food is bland but\
nourishing and looks like a food of your choice, and\
the water is clean. The food spoils after 24 hours if\
uneaten.\

Original layout: [[../../srd-5.2.1/pages/120|page 120]].
