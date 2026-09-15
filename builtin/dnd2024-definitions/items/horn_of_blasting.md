---
splat_schema: 1
splat_id: 917826ed-6fb1-50a9-bf0b-d900c688c292
splat_kind: item
splat_key: dnd2024:items/horn_of_blasting
splat_name: Horn of Blasting
splat_origin:
  key: dnd2024:reference/magic_item/horn_of_blasting
splat_sources:
- title: SRD 5.2.1
  locator: pp. 226
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/horn_of_blasting
  implementation: dnd2024:item_horn_of_blasting
  parameters:
    action: magic
    activation: blow_horn
    audible_range_ft: 600
    area:
      shape: cone
      size_ft: 30
    save:
      ability: constitution
      dc: 15
      failure:
        damage:
          amount:
            roll:
              count: 5
              sides: 8
          type: thunder
        condition: deafened
        condition_duration:
          amount: 1
          unit: minutes
      success:
        damage_fraction:
        - 1
        - 2
        condition: null
    objects:
      materials:
      - glass
      - crystal
      worn_or_carried: false
      damage:
        amount:
          roll:
            count: 10
            sides: 8
        type: thunder
    each_use_explosion:
      probability:
      - 1
      - 5
      target: user
      damage:
        amount:
          roll:
            count: 10
            sides: 6
        type: force
      destroy_item: true
    attunement_required: false
splat_contracts:
- id: dnd2024:item_horn_of_blasting
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - item.activation_requested
  - time.elapsed
  capabilities:
  - item_effects
  - dice
  - effect_duration
  - spatial_targeting
  - damage
  - conditions
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 7c0ba7d5fc0d30dfc7e17af4d5a872ec3ce09c67e485933168a9d917d67a964b
  source_path: builtin/srd-5.2.1/magic_item/horn_of_blasting.md
  pages:
  - 226
  engine_gaps: &id001
  - dnd2024:item_horn_of_blasting
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 0b6e755ac473e22b98a4ec9d99c3b9a25b65727cc0e36ef4d7fcfb93f31e6cfa
splat_clauses:
- key: source_rules
  text: '# Horn of Blasting Wondrous Item, Rare You can take a Magic action to blow the horn, which emits
    a thunderous blast in a 30-foot Cone that is audible out to 600 feet. Each creature in the Cone makes
    a DC 15 Constitution saving throw. On a failed save, a creature takes 5d8 Thunder damage and has the
    Deafened condition for 1 minute. On a successful save, a creature takes half as much damage only.
    Glass or crystal objects in the Cone that aren’t being worn or carried take 10d8 Thunder damage. Each
    use of the horn’s magic has a 20 percent chance of causing the horn to explode. The explosion deals
    10d6 Force damage to the user and destroys the horn.'
  sha256: e0bd8f57ec71b3c1ff2c5621e03b9ded9112e3ad5b44be053b846d9fec3779f2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Horn of Blasting

<!-- SRD page 226 -->

Wondrous Item, Rare\
You can take a Magic action to blow the horn, which\
emits a thunderous blast in a 30-foot Cone that is\
audible out to 600 feet. Each creature in the Cone\
makes a DC 15 Constitution saving throw. On a\
failed save, a creature takes 5d8 Thunder damage\
and has the Deafened condition for 1 minute. On a\
successful save, a creature takes half as much dam-\
age only. Glass or crystal objects in the Cone that\
aren’t being worn or carried take 10d8 Thunder\
damage.\
Each use of the horn’s magic has a 20 percent\
chance of causing the horn to explode. The ex-\
plosion deals 10d6 Force damage to the user and\
destroys the horn.\

Original layout: [[../../srd-5.2.1/pages/226|page 226]].
